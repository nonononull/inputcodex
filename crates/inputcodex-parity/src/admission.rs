use std::collections::BTreeSet;

use serde::Deserialize;

use crate::{ValidationCode, ValidationIssue};

const ADMISSION_MATRIX_SCHEMA: &str = "inputcodex.side-effect-admission-matrix.v1";

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AdmissionBucket {
    Write,
    Process,
    Network,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum OwnerState {
    Missing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AdmissionState {
    Blocked,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SideEffectAdmissionMatrix {
    schema_version: String,
    release: AdmissionRelease,
    entries: Vec<AdmissionEntry>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct AdmissionRelease {
    tag: String,
    tag_commit: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AdmissionEntry {
    source_id: String,
    feature_id: String,
    primary_bucket: AdmissionBucket,
    owner_state: OwnerState,
    required_owner_kinds: Vec<String>,
    blocker_refs: Vec<String>,
    admission: AdmissionState,
    implementation_authorized: bool,
}

impl SideEffectAdmissionMatrix {
    #[must_use]
    pub fn entries(&self) -> &[AdmissionEntry] {
        &self.entries
    }

    pub(crate) fn release_tag(&self) -> &str {
        &self.release.tag
    }

    pub(crate) fn release_commit(&self) -> &str {
        &self.release.tag_commit
    }
}

impl AdmissionEntry {
    #[must_use]
    pub fn source_id(&self) -> &str {
        &self.source_id
    }

    #[must_use]
    pub fn feature_id(&self) -> &str {
        &self.feature_id
    }

    #[must_use]
    pub const fn primary_bucket(&self) -> AdmissionBucket {
        self.primary_bucket
    }

    #[must_use]
    pub const fn owner_state(&self) -> OwnerState {
        self.owner_state
    }

    #[must_use]
    pub fn required_owner_kinds(&self) -> &[String] {
        &self.required_owner_kinds
    }

    #[must_use]
    pub fn blocker_refs(&self) -> &[String] {
        &self.blocker_refs
    }

    #[must_use]
    pub const fn admission(&self) -> AdmissionState {
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

#[must_use]
pub fn validate_side_effect_admission_matrix(
    matrix: &SideEffectAdmissionMatrix,
) -> Vec<ValidationIssue> {
    let mut issues = Vec::new();
    let mut source_ids = BTreeSet::new();
    let mut previous_source_id: Option<&str> = None;

    if matrix.schema_version != ADMISSION_MATRIX_SCHEMA {
        issues.push(ValidationIssue::new(
            ValidationCode::SchemaVersionMismatch,
            "side-effect-admission-matrix.schema_version",
        ));
    }
    if matrix.release.tag.trim().is_empty()
        || matrix.release.tag_commit.len() != 40
        || !matrix
            .release
            .tag_commit
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit())
    {
        issues.push(ValidationIssue::new(
            ValidationCode::InvalidAdmissionMetadata,
            "side-effect-admission-matrix.release",
        ));
    }

    for entry in &matrix.entries {
        if entry.source_id.trim().is_empty()
            || entry.feature_id.trim().is_empty()
            || !entry.feature_id.starts_with("feature.")
            || !is_strict_string_sequence(&entry.required_owner_kinds)
            || !is_strict_string_sequence(&entry.blocker_refs)
        {
            issues.push(ValidationIssue::new(
                ValidationCode::InvalidAdmissionMetadata,
                entry.source_id.clone(),
            ));
        }
        if !source_ids.insert(entry.source_id.as_str()) {
            issues.push(ValidationIssue::new(
                ValidationCode::DuplicateAdmissionSource,
                entry.source_id.clone(),
            ));
        }
        if previous_source_id.is_some_and(|previous| previous >= entry.source_id.as_str()) {
            issues.push(ValidationIssue::new(
                ValidationCode::AdmissionOrderMismatch,
                entry.source_id.clone(),
            ));
        }
        previous_source_id = Some(entry.source_id.as_str());

        if entry.admission != AdmissionState::Blocked || entry.implementation_authorized {
            issues.push(ValidationIssue::new(
                ValidationCode::AdmissionAuthorizationMismatch,
                entry.source_id.clone(),
            ));
        }
    }

    issues
}

pub(crate) fn classify_admission_bucket(side_effects: &[String]) -> Option<AdmissionBucket> {
    if side_effects.iter().any(|effect| {
        matches!(
            effect.as_str(),
            "clipboard-write"
                | "database-write"
                | "environment-write"
                | "filesystem-write"
                | "injection"
                | "request-mutation"
        )
    }) {
        return Some(AdmissionBucket::Write);
    }
    if side_effects
        .iter()
        .any(|effect| effect == "process-control")
    {
        return Some(AdmissionBucket::Process);
    }
    if side_effects
        .iter()
        .any(|effect| matches!(effect.as_str(), "network-listen" | "network-read"))
    {
        return Some(AdmissionBucket::Network);
    }
    None
}

pub(crate) fn required_owner_kinds(feature_id: &str, side_effects: &[String]) -> Vec<String> {
    let mut owners = BTreeSet::new();
    for effect in side_effects {
        match effect.as_str() {
            "clipboard-write" => {
                owners.insert("clipboard-mutation");
            }
            "database-write" => {
                owners.insert("database-transaction");
            }
            "environment-write" => {
                owners.insert("environment-mutation");
            }
            "filesystem-write" => {
                owners.insert("application-file-mutation");
            }
            "injection" => {
                owners.insert("injection-owner");
            }
            "network-listen" => {
                owners.insert("network-listener");
            }
            "network-read" => {
                owners.insert("bounded-network-transport");
            }
            "process-control" => {
                owners.insert("process-controller");
            }
            "request-mutation" => {
                owners.insert("request-mutation-owner");
            }
            _ => {}
        }
    }

    if matches!(
        feature_id,
        "feature.provider-network.context-entry-management"
            | "feature.provider-network.model-catalog"
            | "feature.provider-network.provider-configuration-application"
            | "feature.provider-network.provider-diagnostics"
            | "feature.provider-network.provider-import"
            | "feature.provider-network.relay-profile-management"
            | "feature.provider-network.sub2api-billing-observation"
    ) {
        owners.insert("credential-profile");
    }
    if feature_id == "feature.foundation-platform.settings-management" {
        owners.insert("settings-document");
    }

    owners.into_iter().map(str::to_owned).collect()
}

pub(crate) fn required_blocker_refs(feature_id: &str) -> Vec<String> {
    let mut blockers = BTreeSet::from(["issue:140"]);
    match feature_id {
        "feature.foundation-platform.diagnostics" => {
            blockers.insert("issue:95");
        }
        "feature.foundation-platform.environment-conflicts" => {
            blockers.insert("issue:85");
        }
        "feature.foundation-platform.settings-management" => {
            blockers.insert("issue:94");
        }
        "feature.foundation-platform.watcher" => {
            blockers.insert("issue:136");
        }
        "feature.provider-network.context-entry-management" => {
            blockers.insert("issue:100");
        }
        "feature.provider-network.model-catalog"
        | "feature.provider-network.provider-configuration-application"
        | "feature.provider-network.provider-diagnostics"
        | "feature.provider-network.provider-import"
        | "feature.provider-network.relay-profile-management"
        | "feature.provider-network.sub2api-billing-observation" => {
            blockers.insert("issue:126");
        }
        "feature.provider-network.network-environment" => {
            blockers.insert("issue:88");
        }
        "feature.remote-install.zed-remote" => {
            blockers.insert("issue:132");
            blockers.insert("issue:134");
        }
        "feature.session-data.local-session-management" => {
            blockers.insert("issue:125");
            blockers.insert("issue:134");
        }
        "feature.session-data.provider-metadata-maintenance" => {
            blockers.insert("issue:130");
            blockers.insert("issue:134");
        }
        _ => {}
    }
    blockers.into_iter().map(str::to_owned).collect()
}

fn is_strict_string_sequence(values: &[String]) -> bool {
    !values.is_empty()
        && values
            .iter()
            .all(|value| !value.trim().is_empty() && value.trim() == value)
        && values
            .windows(2)
            .all(|pair| pair[0].as_str() < pair[1].as_str())
}
