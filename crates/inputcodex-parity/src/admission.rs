use std::collections::{BTreeMap, BTreeSet};

use serde::Deserialize;

use crate::{ParityStatus, SourceDisposition, SourceIndex, ValidationCode, ValidationIssue};

pub(crate) const SIDE_EFFECT_ADMISSION_MATRIX_SCHEMA: &str =
    "inputcodex.side-effect-admission-matrix.v1";

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
    tag_commit: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SideEffectBucket {
    Write,
    Process,
    Network,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum RequiredOwnerKind {
    Filesystem,
    Database,
    Environment,
    Clipboard,
    ProcessController,
    NetworkTransport,
    CredentialProfile,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AdmissionOwnerState {
    Missing,
    Present,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AdmissionDecision {
    Blocked,
    Admitted,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SideEffectAdmissionEntry {
    source_id: String,
    feature_id: String,
    bucket: SideEffectBucket,
    required_owner_kinds: Vec<RequiredOwnerKind>,
    owner_state: AdmissionOwnerState,
    blocker_refs: Vec<String>,
    admission: AdmissionDecision,
    implementation_authorized: bool,
}

impl SideEffectAdmissionMatrix {
    #[must_use]
    pub fn release_tag(&self) -> &str {
        &self.release.tag
    }

    #[must_use]
    pub fn release_commit(&self) -> &str {
        &self.release.tag_commit
    }

    #[must_use]
    pub fn entries(&self) -> &[SideEffectAdmissionEntry] {
        &self.entries
    }

    pub(crate) fn schema_version(&self) -> &str {
        &self.schema_version
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
    pub const fn bucket(&self) -> SideEffectBucket {
        self.bucket
    }

    #[must_use]
    pub fn required_owner_kinds(&self) -> &[RequiredOwnerKind] {
        &self.required_owner_kinds
    }

    #[must_use]
    pub const fn owner_state(&self) -> AdmissionOwnerState {
        self.owner_state
    }

    #[must_use]
    pub fn blocker_refs(&self) -> &[String] {
        &self.blocker_refs
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

pub(crate) fn validate_side_effect_admission_matrix(
    matrix: &SideEffectAdmissionMatrix,
    source_index: &SourceIndex,
    feature_statuses: &BTreeMap<String, ParityStatus>,
    expected_release_tag: &str,
    expected_release_commit: &str,
) -> Vec<ValidationIssue> {
    let mut issues = Vec::new();
    if matrix.schema_version() != SIDE_EFFECT_ADMISSION_MATRIX_SCHEMA {
        issues.push(ValidationIssue::new(
            ValidationCode::AdmissionSchemaMismatch,
            "parity/admission/side-effect-admission-matrix.yml",
        ));
    }
    if matrix.release_tag() != expected_release_tag
        || matrix.release_commit() != expected_release_commit
    {
        issues.push(ValidationIssue::new(
            ValidationCode::AdmissionReleaseMismatch,
            "parity/admission/side-effect-admission-matrix.yml:release",
        ));
    }

    let expected_sources = source_index
        .sources()
        .iter()
        .filter_map(|source| {
            let feature_id = source
                .disposition()
                .and_then(SourceDisposition::feature_id)?;
            (feature_statuses.get(feature_id) == Some(&ParityStatus::Unassessed))
                .then_some((source.id(), (source, feature_id)))
        })
        .collect::<BTreeMap<_, _>>();
    let owner_kinds = expected_owner_kinds(source_index, feature_statuses);

    let mut previous_source_id: Option<&str> = None;
    let mut matrix_source_ids = BTreeSet::new();
    for entry in matrix.entries() {
        if previous_source_id.is_some_and(|previous| previous >= entry.source_id()) {
            issues.push(ValidationIssue::new(
                ValidationCode::AdmissionEntryOrderMismatch,
                entry.source_id(),
            ));
        }
        previous_source_id = Some(entry.source_id());
        if !matrix_source_ids.insert(entry.source_id()) {
            issues.push(ValidationIssue::new(
                ValidationCode::AdmissionSourceClosureMismatch,
                entry.source_id(),
            ));
        }

        let Some((_, expected_feature_id)) = expected_sources.get(entry.source_id()) else {
            issues.push(ValidationIssue::new(
                ValidationCode::AdmissionSourceClosureMismatch,
                entry.source_id(),
            ));
            continue;
        };
        if entry.feature_id() != *expected_feature_id {
            issues.push(ValidationIssue::new(
                ValidationCode::AdmissionFeatureMismatch,
                entry.source_id(),
            ));
        }

        if expected_bucket(expected_feature_id).is_none_or(|bucket| bucket != entry.bucket()) {
            issues.push(ValidationIssue::new(
                ValidationCode::AdmissionBucketMismatch,
                entry.source_id(),
            ));
        }
        let expected_owner_kinds = owner_kinds
            .get(*expected_feature_id)
            .map(|kinds| kinds.iter().copied().collect::<Vec<_>>())
            .unwrap_or_default();
        if entry.required_owner_kinds() != expected_owner_kinds
            || entry.owner_state() != AdmissionOwnerState::Missing
        {
            issues.push(ValidationIssue::new(
                ValidationCode::AdmissionOwnerMismatch,
                entry.source_id(),
            ));
        }
        if expected_blocker_refs(expected_feature_id)
            .is_none_or(|refs| entry.blocker_refs() != refs)
        {
            issues.push(ValidationIssue::new(
                ValidationCode::AdmissionBlockerMismatch,
                entry.source_id(),
            ));
        }
        if entry.admission() != AdmissionDecision::Blocked || entry.implementation_authorized() {
            issues.push(ValidationIssue::new(
                ValidationCode::AdmissionStateMismatch,
                entry.source_id(),
            ));
        }
    }

    let expected_source_ids = expected_sources.keys().copied().collect::<BTreeSet<_>>();
    if matrix_source_ids != expected_source_ids {
        issues.push(ValidationIssue::new(
            ValidationCode::AdmissionSourceClosureMismatch,
            "parity/admission/side-effect-admission-matrix.yml:entries",
        ));
    }
    issues
}

fn expected_owner_kinds(
    source_index: &SourceIndex,
    feature_statuses: &BTreeMap<String, ParityStatus>,
) -> BTreeMap<String, BTreeSet<RequiredOwnerKind>> {
    let mut owner_kinds = BTreeMap::<String, BTreeSet<RequiredOwnerKind>>::new();
    for source in source_index.sources() {
        let Some(feature_id) = source.disposition().and_then(SourceDisposition::feature_id) else {
            continue;
        };
        if feature_statuses.get(feature_id) != Some(&ParityStatus::Unassessed) {
            continue;
        }
        let feature_owner_kinds = owner_kinds.entry(feature_id.to_owned()).or_default();
        for effect in source.side_effects() {
            let owner_kind = match effect.as_str() {
                "filesystem-read" | "filesystem-write" => Some(RequiredOwnerKind::Filesystem),
                "database-read" | "database-write" => Some(RequiredOwnerKind::Database),
                "environment-read" | "environment-write" => Some(RequiredOwnerKind::Environment),
                "clipboard-write" => Some(RequiredOwnerKind::Clipboard),
                "process-control" => Some(RequiredOwnerKind::ProcessController),
                "network-read" | "network-listen" => Some(RequiredOwnerKind::NetworkTransport),
                _ => None,
            };
            if let Some(owner_kind) = owner_kind {
                feature_owner_kinds.insert(owner_kind);
            }
        }
    }

    for feature_id in [
        "feature.provider-network.aggregate-routing",
        "feature.provider-network.context-entry-management",
        "feature.provider-network.model-catalog",
        "feature.provider-network.provider-configuration-application",
        "feature.provider-network.provider-diagnostics",
        "feature.provider-network.provider-import",
        "feature.provider-network.relay-profile-management",
        "feature.provider-network.sub2api-billing-observation",
    ] {
        owner_kinds
            .entry(feature_id.to_owned())
            .or_default()
            .insert(RequiredOwnerKind::CredentialProfile);
    }
    owner_kinds
}

fn expected_bucket(feature_id: &str) -> Option<SideEffectBucket> {
    match feature_id {
        "feature.remote-install.upstream-worktree" | "feature.remote-install.zed-remote" => {
            Some(SideEffectBucket::Process)
        }
        "feature.provider-network.network-environment"
        | "feature.provider-network.protocol-proxy"
        | "feature.provider-network.provider-diagnostics"
        | "feature.provider-network.sub2api-billing-observation" => Some(SideEffectBucket::Network),
        "feature.foundation-platform.application-lifecycle"
        | "feature.foundation-platform.diagnostics"
        | "feature.foundation-platform.environment-conflicts"
        | "feature.foundation-platform.settings-management"
        | "feature.foundation-platform.watcher"
        | "feature.plugin-script.dream-skin-library"
        | "feature.provider-network.aggregate-routing"
        | "feature.provider-network.context-entry-management"
        | "feature.provider-network.model-catalog"
        | "feature.provider-network.provider-configuration-application"
        | "feature.provider-network.provider-import"
        | "feature.provider-network.relay-profile-management"
        | "feature.remote-install.application-update"
        | "feature.remote-install.entrypoint-installation"
        | "feature.session-data.local-session-management"
        | "feature.session-data.provider-metadata-maintenance" => Some(SideEffectBucket::Write),
        _ => None,
    }
}

fn expected_blocker_refs(feature_id: &str) -> Option<&'static [&'static str]> {
    match feature_id {
        "feature.foundation-platform.application-lifecycle" => Some(&["issue:77"]),
        "feature.foundation-platform.diagnostics" => Some(&["issue:95"]),
        "feature.foundation-platform.environment-conflicts" => Some(&["issue:85"]),
        "feature.foundation-platform.settings-management" => Some(&["issue:94"]),
        "feature.foundation-platform.watcher" => Some(&["issue:136"]),
        "feature.plugin-script.dream-skin-library" => Some(&["issue:140"]),
        "feature.provider-network.aggregate-routing"
        | "feature.provider-network.model-catalog"
        | "feature.provider-network.protocol-proxy"
        | "feature.provider-network.provider-configuration-application"
        | "feature.provider-network.provider-diagnostics"
        | "feature.provider-network.provider-import"
        | "feature.provider-network.relay-profile-management"
        | "feature.provider-network.sub2api-billing-observation" => Some(&["issue:126"]),
        "feature.provider-network.context-entry-management" => Some(&["issue:100"]),
        "feature.provider-network.network-environment" => Some(&["issue:88"]),
        "feature.remote-install.application-update" => Some(&["rule:own-repository-updates"]),
        "feature.remote-install.entrypoint-installation"
        | "feature.remote-install.upstream-worktree" => Some(&["issue:140"]),
        "feature.remote-install.zed-remote" | "feature.session-data.local-session-management" => {
            Some(&["issue:134"])
        }
        "feature.session-data.provider-metadata-maintenance" => Some(&["issue:130", "issue:134"]),
        _ => None,
    }
}
