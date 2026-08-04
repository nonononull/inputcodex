use std::collections::{BTreeMap, BTreeSet};

use serde::Deserialize;

use crate::{ParityStatus, SourceIndex, ValidationCode, ValidationIssue};

const ADMISSION_MATRIX_SCHEMA: &str = "inputcodex.side-effect-admission-matrix.v1";
const ADMISSION_BASELINE_COMMIT: &str = "c26e97ee534b74ebe1252346477640dc196f89b9";
const RELEASE_TAG: &str = "v1.2.44";
const RELEASE_COMMIT: &str = "77091ccaee4423f35a1b2c51c4ecd703e6201092";

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SideEffectBucket {
    Write,
    Process,
    Network,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TypedOwnerState {
    Missing,
    Partial,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TypedOwnerKind {
    FilesystemMutation,
    DatabaseMutation,
    EnvironmentMutation,
    ClipboardMutation,
    ProcessController,
    NetworkTransport,
    CredentialProfile,
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
    baseline_commit: String,
    release: AdmissionRelease,
    bucket_precedence: Vec<SideEffectBucket>,
    sources: Vec<SideEffectAdmissionEntry>,
}

impl SideEffectAdmissionMatrix {
    #[must_use]
    pub fn sources(&self) -> &[SideEffectAdmissionEntry] {
        &self.sources
    }
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct AdmissionRelease {
    tag: String,
    tag_commit: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SideEffectAdmissionEntry {
    source_id: String,
    feature_id: String,
    bucket: SideEffectBucket,
    typed_owner: TypedOwner,
    blocker_refs: Vec<String>,
    admission: AdmissionDecision,
    implementation_authorized: bool,
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
    pub const fn typed_owner(&self) -> &TypedOwner {
        &self.typed_owner
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

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TypedOwner {
    state: TypedOwnerState,
    kinds: Vec<TypedOwnerKind>,
}

struct AdmissionExpectation {
    state: TypedOwnerState,
    kinds: &'static [TypedOwnerKind],
    blocker_refs: &'static [&'static str],
}

impl TypedOwner {
    #[must_use]
    pub const fn state(&self) -> TypedOwnerState {
        self.state
    }

    #[must_use]
    pub fn kinds(&self) -> &[TypedOwnerKind] {
        &self.kinds
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
    source_index: &SourceIndex,
    feature_statuses: &BTreeMap<String, ParityStatus>,
) -> Vec<ValidationIssue> {
    let mut issues = Vec::new();
    if matrix.schema_version != ADMISSION_MATRIX_SCHEMA {
        issues.push(ValidationIssue::new(
            ValidationCode::SchemaVersionMismatch,
            "side-effect-admission-matrix.schema_version",
        ));
    }
    if matrix.baseline_commit != ADMISSION_BASELINE_COMMIT {
        issues.push(ValidationIssue::new(
            ValidationCode::AdmissionMetadataInvalid,
            "side-effect-admission-matrix.baseline_commit",
        ));
    }
    if matrix.release.tag != RELEASE_TAG || matrix.release.tag_commit != RELEASE_COMMIT {
        issues.push(ValidationIssue::new(
            ValidationCode::ReleaseMismatch,
            "side-effect-admission-matrix.release",
        ));
    }
    if matrix.bucket_precedence
        != [
            SideEffectBucket::Write,
            SideEffectBucket::Process,
            SideEffectBucket::Network,
        ]
    {
        issues.push(ValidationIssue::new(
            ValidationCode::AdmissionMetadataInvalid,
            "side-effect-admission-matrix.bucket_precedence",
        ));
    }

    let feature_sources = source_index
        .sources()
        .iter()
        .filter_map(|source| {
            let feature_id = source.disposition()?.feature_id()?;
            Some((source.id(), (feature_id, source)))
        })
        .collect::<BTreeMap<_, _>>();
    let target_sources = feature_sources
        .iter()
        .filter_map(|(source_id, (feature_id, source))| {
            (feature_statuses.get(*feature_id) == Some(&ParityStatus::Unassessed))
                .then_some((*source_id, (*feature_id, *source)))
        })
        .collect::<BTreeMap<_, _>>();
    let feature_buckets = aggregate_feature_buckets(target_sources.values().copied(), &mut issues);
    let mut matrix_source_ids = BTreeSet::new();

    for entry in &matrix.sources {
        if !matrix_source_ids.insert(entry.source_id.as_str()) {
            issues.push(ValidationIssue::new(
                ValidationCode::DuplicateAdmissionSource,
                entry.source_id.clone(),
            ));
        }
        if !has_valid_metadata(entry) {
            issues.push(ValidationIssue::new(
                ValidationCode::AdmissionMetadataInvalid,
                entry.source_id.clone(),
            ));
        }
        if entry.implementation_authorized || entry.admission != AdmissionDecision::Blocked {
            issues.push(ValidationIssue::new(
                ValidationCode::AdmissionUnauthorized,
                entry.source_id.clone(),
            ));
        }

        let Some((expected_feature_id, _source)) = feature_sources.get(entry.source_id.as_str())
        else {
            issues.push(ValidationIssue::new(
                ValidationCode::UnexpectedAdmissionSource,
                entry.source_id.clone(),
            ));
            continue;
        };
        if entry.feature_id != **expected_feature_id {
            issues.push(ValidationIssue::new(
                ValidationCode::AdmissionFeatureMismatch,
                format!("{}:{}", entry.source_id, entry.feature_id),
            ));
        }
        if let Some(expectation) = admission_expectation(expected_feature_id) {
            if entry.typed_owner.state != expectation.state
                || entry.typed_owner.kinds.as_slice() != expectation.kinds
            {
                issues.push(ValidationIssue::new(
                    ValidationCode::AdmissionOwnerMismatch,
                    entry.source_id.clone(),
                ));
            }
            if !entry
                .blocker_refs
                .iter()
                .map(String::as_str)
                .eq(expectation.blocker_refs.iter().copied())
            {
                issues.push(ValidationIssue::new(
                    ValidationCode::AdmissionBlockerMismatch,
                    entry.source_id.clone(),
                ));
            }
        } else {
            issues.push(ValidationIssue::new(
                ValidationCode::AdmissionMetadataInvalid,
                format!("{}:{}", entry.source_id, expected_feature_id),
            ));
        }
        if feature_statuses.get(entry.feature_id.as_str()) != Some(&ParityStatus::Unassessed) {
            issues.push(ValidationIssue::new(
                ValidationCode::AdmissionTargetStatusMismatch,
                format!("{}:{}", entry.source_id, entry.feature_id),
            ));
            continue;
        }
        if feature_buckets.get(*expected_feature_id) != Some(&entry.bucket) {
            issues.push(ValidationIssue::new(
                ValidationCode::AdmissionBucketMismatch,
                format!("{}:{}", entry.source_id, entry.feature_id),
            ));
        }
    }

    for source_id in target_sources.keys() {
        if !matrix_source_ids.contains(source_id) {
            issues.push(ValidationIssue::new(
                ValidationCode::MissingAdmissionSource,
                *source_id,
            ));
        }
    }

    issues
}

fn admission_expectation(feature_id: &str) -> Option<AdmissionExpectation> {
    use TypedOwnerKind::{
        ClipboardMutation, CredentialProfile, DatabaseMutation, EnvironmentMutation,
        FilesystemMutation, NetworkTransport, ProcessController,
    };
    use TypedOwnerState::{Missing, Partial};

    let expectation = match feature_id {
        "feature.foundation-platform.application-lifecycle" => AdmissionExpectation {
            state: Missing,
            kinds: &[FilesystemMutation, ProcessController],
            blocker_refs: &["issue:140"],
        },
        "feature.foundation-platform.diagnostics" => AdmissionExpectation {
            state: Partial,
            kinds: &[FilesystemMutation, ClipboardMutation],
            blocker_refs: &["issue:95"],
        },
        "feature.foundation-platform.environment-conflicts" => AdmissionExpectation {
            state: Partial,
            kinds: &[EnvironmentMutation],
            blocker_refs: &["issue:85"],
        },
        "feature.foundation-platform.settings-management" => AdmissionExpectation {
            state: Partial,
            kinds: &[FilesystemMutation],
            blocker_refs: &["issue:94"],
        },
        "feature.foundation-platform.watcher" => AdmissionExpectation {
            state: Partial,
            kinds: &[FilesystemMutation, ProcessController],
            blocker_refs: &["issue:136", "issue:140"],
        },
        "feature.plugin-script.dream-skin-library" => AdmissionExpectation {
            state: Missing,
            kinds: &[FilesystemMutation],
            blocker_refs: &["issue:140"],
        },
        "feature.provider-network.aggregate-routing" => AdmissionExpectation {
            state: Missing,
            kinds: &[FilesystemMutation, CredentialProfile],
            blocker_refs: &["issue:126"],
        },
        "feature.provider-network.context-entry-management" => AdmissionExpectation {
            state: Partial,
            kinds: &[FilesystemMutation, CredentialProfile],
            blocker_refs: &["issue:100"],
        },
        "feature.provider-network.model-catalog" => AdmissionExpectation {
            state: Missing,
            kinds: &[FilesystemMutation, NetworkTransport, CredentialProfile],
            blocker_refs: &["issue:126"],
        },
        "feature.provider-network.network-environment" => AdmissionExpectation {
            state: Partial,
            kinds: &[NetworkTransport],
            blocker_refs: &["issue:88"],
        },
        "feature.provider-network.protocol-proxy" => AdmissionExpectation {
            state: Missing,
            kinds: &[NetworkTransport],
            blocker_refs: &["issue:126"],
        },
        "feature.provider-network.provider-configuration-application" => AdmissionExpectation {
            state: Missing,
            kinds: &[FilesystemMutation, ProcessController, CredentialProfile],
            blocker_refs: &["issue:126", "issue:140"],
        },
        "feature.provider-network.provider-diagnostics" => AdmissionExpectation {
            state: Missing,
            kinds: &[CredentialProfile, NetworkTransport],
            blocker_refs: &["issue:126"],
        },
        "feature.provider-network.provider-import" => AdmissionExpectation {
            state: Missing,
            kinds: &[FilesystemMutation, CredentialProfile],
            blocker_refs: &["issue:126"],
        },
        "feature.provider-network.relay-profile-management" => AdmissionExpectation {
            state: Partial,
            kinds: &[FilesystemMutation, CredentialProfile],
            blocker_refs: &["issue:97", "issue:126"],
        },
        "feature.provider-network.sub2api-billing-observation" => AdmissionExpectation {
            state: Missing,
            kinds: &[CredentialProfile, NetworkTransport],
            blocker_refs: &["issue:126"],
        },
        "feature.remote-install.application-update" => AdmissionExpectation {
            state: Missing,
            kinds: &[FilesystemMutation, ProcessController, NetworkTransport],
            blocker_refs: &["issue:140"],
        },
        "feature.remote-install.entrypoint-installation" => AdmissionExpectation {
            state: Missing,
            kinds: &[FilesystemMutation, ProcessController],
            blocker_refs: &["issue:140"],
        },
        "feature.remote-install.upstream-worktree" => AdmissionExpectation {
            state: Missing,
            kinds: &[ProcessController],
            blocker_refs: &["issue:140"],
        },
        "feature.remote-install.zed-remote" => AdmissionExpectation {
            state: Missing,
            kinds: &[ProcessController],
            blocker_refs: &["issue:132", "issue:134"],
        },
        "feature.session-data.local-session-management" => AdmissionExpectation {
            state: Partial,
            kinds: &[DatabaseMutation, FilesystemMutation],
            blocker_refs: &["issue:134"],
        },
        "feature.session-data.provider-metadata-maintenance" => AdmissionExpectation {
            state: Missing,
            kinds: &[DatabaseMutation, FilesystemMutation],
            blocker_refs: &["issue:130", "issue:134"],
        },
        _ => return None,
    };
    Some(expectation)
}

fn aggregate_feature_buckets<'a>(
    sources: impl Iterator<Item = (&'a str, &'a crate::SourceEntry)>,
    issues: &mut Vec<ValidationIssue>,
) -> BTreeMap<&'a str, SideEffectBucket> {
    let mut buckets = BTreeMap::new();
    for (feature_id, source) in sources {
        let Some(source_bucket) = source_bucket(source.side_effects()) else {
            issues.push(ValidationIssue::new(
                ValidationCode::AdmissionBucketMismatch,
                source.id(),
            ));
            continue;
        };
        buckets
            .entry(feature_id)
            .and_modify(|bucket| {
                if source_bucket < *bucket {
                    *bucket = source_bucket;
                }
            })
            .or_insert(source_bucket);
    }
    buckets
}

fn source_bucket(side_effects: &[String]) -> Option<SideEffectBucket> {
    if side_effects.iter().any(|effect| {
        matches!(
            effect.as_str(),
            "filesystem-write" | "database-write" | "environment-write" | "clipboard-write"
        )
    }) {
        Some(SideEffectBucket::Write)
    } else if side_effects
        .iter()
        .any(|effect| effect == "process-control")
    {
        Some(SideEffectBucket::Process)
    } else if side_effects.iter().any(|effect| effect == "network-read") {
        Some(SideEffectBucket::Network)
    } else {
        None
    }
}

fn has_valid_metadata(entry: &SideEffectAdmissionEntry) -> bool {
    let owner_kinds = entry
        .typed_owner
        .kinds
        .iter()
        .copied()
        .collect::<BTreeSet<_>>();
    let blocker_refs = entry.blocker_refs.iter().collect::<BTreeSet<_>>();
    !entry.source_id.trim().is_empty()
        && !entry.feature_id.trim().is_empty()
        && !owner_kinds.is_empty()
        && owner_kinds.len() == entry.typed_owner.kinds.len()
        && !blocker_refs.is_empty()
        && blocker_refs.len() == entry.blocker_refs.len()
        && entry.blocker_refs.iter().all(|reference| {
            reference.strip_prefix("issue:").is_some_and(|number| {
                !number.is_empty() && number.bytes().all(|byte| byte.is_ascii_digit())
            }) || reference
                .strip_prefix("rule:")
                .is_some_and(|rule| !rule.trim().is_empty())
        })
}
