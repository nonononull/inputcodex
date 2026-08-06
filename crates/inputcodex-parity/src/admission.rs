use std::collections::{BTreeMap, BTreeSet};

use serde::Deserialize;

use crate::{ParityStatus, SourceIndex, ValidationCode, ValidationIssue};

const ADMISSION_MATRIX_SCHEMA: &str = "inputcodex.side-effect-admission-matrix.v1";
const OWNER_BLOCKER_REF: &str = "https://github.com/nonononull/inputcodex/issues/140";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AdmissionBucket {
    Write,
    ProcessControl,
    Network,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AdmissionOwnerState {
    Missing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AdmissionDecision {
    Blocked,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SideEffectAdmissionMatrix {
    schema_version: String,
    release: AdmissionRelease,
    entries: Vec<SideEffectAdmissionEntry>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct AdmissionRelease {
    tag: String,
    commit: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SideEffectAdmissionEntry {
    source_id: String,
    feature_id: String,
    bucket: AdmissionBucket,
    owner_state: AdmissionOwnerState,
    owner_kinds: Vec<String>,
    blocker_refs: Vec<String>,
    admission: AdmissionDecision,
    implementation_authorized: bool,
}

impl SideEffectAdmissionMatrix {
    #[must_use]
    pub fn schema_version(&self) -> &str {
        &self.schema_version
    }

    #[must_use]
    pub fn release_tag(&self) -> &str {
        &self.release.tag
    }

    #[must_use]
    pub fn release_commit(&self) -> &str {
        &self.release.commit
    }

    #[must_use]
    pub fn entries(&self) -> &[SideEffectAdmissionEntry] {
        &self.entries
    }
}

impl SideEffectAdmissionEntry {
    #[must_use]
    pub fn source_id(&self) -> &str {
        &self.source_id
    }

    #[must_use]
    pub fn feature_id(&self) -> &str {
        &self.feature_id
    }

    #[must_use]
    pub const fn bucket(&self) -> AdmissionBucket {
        self.bucket
    }

    #[must_use]
    pub const fn owner_state(&self) -> AdmissionOwnerState {
        self.owner_state
    }

    #[must_use]
    pub fn owner_kinds(&self) -> Vec<&str> {
        self.owner_kinds.iter().map(String::as_str).collect()
    }

    #[must_use]
    pub fn blocker_refs(&self) -> Vec<&str> {
        self.blocker_refs.iter().map(String::as_str).collect()
    }

    #[must_use]
    pub const fn admission(&self) -> AdmissionDecision {
        self.admission
    }

    #[must_use]
    pub const fn implementation_authorized(&self) -> bool {
        self.implementation_authorized
    }
}

pub fn parse_side_effect_admission_matrix(
    input: &str,
) -> yaml_serde::Result<SideEffectAdmissionMatrix> {
    yaml_serde::from_str(input)
}

struct ExpectedAdmission<'a> {
    feature_id: &'a str,
    bucket: AdmissionBucket,
    owner_kinds: Vec<&'static str>,
}

pub(crate) fn validate_side_effect_admission_matrix(
    matrix: &SideEffectAdmissionMatrix,
    source_index: &SourceIndex,
    feature_statuses: &BTreeMap<String, ParityStatus>,
    expected_release_tag: &str,
    expected_release_commit: &str,
) -> Vec<ValidationIssue> {
    let mut issues = Vec::new();
    let matrix_path = "parity/admission/side-effect-admission-matrix.yml";

    if matrix.schema_version != ADMISSION_MATRIX_SCHEMA {
        issues.push(ValidationIssue::new(
            ValidationCode::AdmissionMatrixSchemaMismatch,
            format!("{matrix_path}.schema_version"),
        ));
    }
    if matrix.release.tag != expected_release_tag {
        issues.push(ValidationIssue::new(
            ValidationCode::AdmissionMatrixReleaseMismatch,
            format!("{matrix_path}.release.tag"),
        ));
    }
    if matrix.release.commit != expected_release_commit {
        issues.push(ValidationIssue::new(
            ValidationCode::AdmissionMatrixReleaseMismatch,
            format!("{matrix_path}.release.commit"),
        ));
    }

    let mut feature_side_effects = BTreeMap::<&str, BTreeSet<&str>>::new();
    for source in source_index.sources() {
        let Some(feature_id) = source
            .disposition()
            .and_then(crate::SourceDisposition::feature_id)
        else {
            continue;
        };
        if feature_statuses.get(feature_id) != Some(&ParityStatus::Unassessed) {
            continue;
        }
        feature_side_effects
            .entry(feature_id)
            .or_default()
            .extend(source.side_effects().iter().map(String::as_str));
    }

    let mut expected = BTreeMap::new();
    for source in source_index.sources() {
        let Some(feature_id) = source
            .disposition()
            .and_then(crate::SourceDisposition::feature_id)
        else {
            continue;
        };
        let Some(side_effects) = feature_side_effects.get(feature_id) else {
            continue;
        };
        let Some(bucket) = admission_bucket(side_effects) else {
            issues.push(ValidationIssue::new(
                ValidationCode::AdmissionMatrixEntryMismatch,
                source.id(),
            ));
            continue;
        };
        expected.insert(
            source.id(),
            ExpectedAdmission {
                feature_id,
                bucket,
                owner_kinds: required_owner_kinds(feature_id, side_effects),
            },
        );
    }

    let mut seen = BTreeSet::new();
    let mut previous_source_id: Option<&str> = None;
    for entry in &matrix.entries {
        if !seen.insert(entry.source_id.as_str()) {
            issues.push(ValidationIssue::new(
                ValidationCode::DuplicateAdmissionSource,
                entry.source_id.clone(),
            ));
        } else if previous_source_id.is_some_and(|previous| previous > entry.source_id.as_str()) {
            issues.push(ValidationIssue::new(
                ValidationCode::AdmissionMatrixOrderMismatch,
                format!(
                    "{}:{}",
                    previous_source_id.expect("previous source 已存在"),
                    entry.source_id
                ),
            ));
        }
        previous_source_id = Some(entry.source_id.as_str());

        let Some(expected_entry) = expected.get(entry.source_id.as_str()) else {
            issues.push(ValidationIssue::new(
                ValidationCode::UnexpectedAdmissionSource,
                entry.source_id.clone(),
            ));
            continue;
        };
        let actual_owner_kinds = entry
            .owner_kinds
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>();
        if entry.feature_id != expected_entry.feature_id
            || entry.bucket != expected_entry.bucket
            || entry.owner_state != AdmissionOwnerState::Missing
            || actual_owner_kinds != expected_entry.owner_kinds
            || entry.blocker_refs != [OWNER_BLOCKER_REF]
            || entry.admission != AdmissionDecision::Blocked
            || entry.implementation_authorized
        {
            issues.push(ValidationIssue::new(
                ValidationCode::AdmissionMatrixEntryMismatch,
                entry.source_id.clone(),
            ));
        }
    }

    for source_id in expected.keys() {
        if !seen.contains(source_id) {
            issues.push(ValidationIssue::new(
                ValidationCode::MissingAdmissionSource,
                *source_id,
            ));
        }
    }

    issues
}

fn admission_bucket(side_effects: &BTreeSet<&str>) -> Option<AdmissionBucket> {
    if side_effects.iter().any(|effect| effect.ends_with("-write")) {
        Some(AdmissionBucket::Write)
    } else if side_effects.contains("process-control") {
        Some(AdmissionBucket::ProcessControl)
    } else if side_effects
        .iter()
        .any(|effect| effect.starts_with("network-"))
    {
        Some(AdmissionBucket::Network)
    } else {
        None
    }
}

fn required_owner_kinds(feature_id: &str, side_effects: &BTreeSet<&str>) -> Vec<&'static str> {
    let mut kinds = BTreeSet::new();
    if requires_credential_profile(feature_id) {
        kinds.insert("credential-profile");
    }
    if side_effects
        .iter()
        .any(|effect| effect.starts_with("network-"))
    {
        kinds.insert("network-transport");
    }
    if side_effects.contains("process-control") {
        kinds.insert("process-control");
    }
    if side_effects.iter().any(|effect| effect.ends_with("-write")) {
        kinds.insert("typed-mutation");
    }
    kinds.into_iter().collect()
}

fn requires_credential_profile(feature_id: &str) -> bool {
    matches!(
        feature_id,
        "feature.provider-network.aggregate-routing"
            | "feature.provider-network.context-entry-management"
            | "feature.provider-network.model-catalog"
            | "feature.provider-network.provider-configuration-application"
            | "feature.provider-network.provider-diagnostics"
            | "feature.provider-network.provider-import"
            | "feature.provider-network.relay-profile-management"
            | "feature.provider-network.sub2api-billing-observation"
    )
}
