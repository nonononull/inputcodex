use std::{
    collections::BTreeSet,
    path::{Component, Path},
};

use serde::Deserialize;

use crate::{ValidationCode, ValidationIssue};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum FeatureDomain {
    FoundationPlatform,
    ProviderNetwork,
    SessionData,
    PluginScript,
    RemoteInstall,
}

impl FeatureDomain {
    pub(crate) const fn as_str(self) -> &'static str {
        match self {
            Self::FoundationPlatform => "foundation-platform",
            Self::ProviderNetwork => "provider-network",
            Self::SessionData => "session-data",
            Self::PluginScript => "plugin-script",
            Self::RemoteInstall => "remote-install",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ParityStatus {
    Unassessed,
    Planned,
    Implementing,
    Implemented,
    Verified,
    ExceptionPending,
    ExceptionApproved,
    Retired,
}

#[derive(Debug, Deserialize)]
pub struct FeatureCatalog {
    schema_version: String,
    release: ReleaseMetadata,
    domain: FeatureDomain,
    features: Vec<FeatureDefinition>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct ReleaseMetadata {
    tag: String,
    tag_commit: String,
}

impl ReleaseMetadata {
    pub(crate) fn tag(&self) -> &str {
        &self.tag
    }

    pub(crate) fn tag_commit(&self) -> &str {
        &self.tag_commit
    }
}

impl FeatureCatalog {
    #[must_use]
    pub const fn domain(&self) -> FeatureDomain {
        self.domain
    }

    #[must_use]
    pub fn features(&self) -> &[FeatureDefinition] {
        &self.features
    }

    pub(crate) fn schema_version(&self) -> &str {
        &self.schema_version
    }

    pub(crate) const fn release(&self) -> &ReleaseMetadata {
        &self.release
    }
}

#[derive(Debug, Deserialize)]
pub struct FeatureDefinition {
    id: String,
    name: String,
    status: ParityStatus,
    evidence: Vec<FeatureEvidence>,
    entry_points: Vec<String>,
    platforms: FeaturePlatforms,
    decision_refs: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct FeatureEvidence {
    path: String,
    symbol: String,
}

impl FeatureEvidence {
    pub(crate) fn path(&self) -> &str {
        &self.path
    }

    pub(crate) fn symbol(&self) -> &str {
        &self.symbol
    }
}

#[derive(Debug, Deserialize)]
struct FeaturePlatforms {
    windows: FeaturePlatform,
    macos: FeaturePlatform,
}

#[derive(Debug, Deserialize)]
struct FeaturePlatform {
    applicability: String,
    semantics: String,
}

impl FeatureDefinition {
    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }

    #[must_use]
    pub const fn status(&self) -> ParityStatus {
        self.status
    }

    pub(crate) fn evidence(&self) -> &[FeatureEvidence] {
        &self.evidence
    }

    pub(crate) fn entry_points(&self) -> &[String] {
        &self.entry_points
    }

    pub(crate) fn has_required_metadata(&self) -> bool {
        !self.name.trim().is_empty()
            && !self.evidence.is_empty()
            && !self.entry_points.is_empty()
            && !self.platforms.windows.applicability.trim().is_empty()
            && !self.platforms.windows.semantics.trim().is_empty()
            && !self.platforms.macos.applicability.trim().is_empty()
            && !self.platforms.macos.semantics.trim().is_empty()
            && !self.decision_refs.is_empty()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SourceKind {
    TauriCommand,
    CoreModule,
    DataModule,
}

impl SourceKind {
    const fn prefix(self) -> &'static str {
        match self {
            Self::TauriCommand => "tauri-command",
            Self::CoreModule => "core-module",
            Self::DataModule => "data-module",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SourcePlatform {
    Windows,
    Macos,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum SourceDisposition {
    Feature {
        feature_id: String,
    },
    Excluded {
        code: String,
        reason: String,
        decision_refs: Vec<String>,
    },
    ExceptionPending {
        feature_id: String,
        reason: String,
        decision_refs: Vec<String>,
    },
}

impl SourceDisposition {
    pub(crate) fn feature_id(&self) -> Option<&str> {
        match self {
            Self::Feature { feature_id } | Self::ExceptionPending { feature_id, .. } => {
                Some(feature_id)
            }
            Self::Excluded { .. } => None,
        }
    }

    pub(crate) const fn is_exception_pending(&self) -> bool {
        matches!(self, Self::ExceptionPending { .. })
    }

    pub(crate) fn has_required_metadata(&self) -> bool {
        match self {
            Self::Feature { feature_id } => !feature_id.trim().is_empty(),
            Self::Excluded {
                code,
                reason,
                decision_refs,
            } => !code.trim().is_empty() && !reason.trim().is_empty() && !decision_refs.is_empty(),
            Self::ExceptionPending {
                feature_id,
                reason,
                decision_refs,
            } => {
                !feature_id.trim().is_empty()
                    && !reason.trim().is_empty()
                    && !decision_refs.is_empty()
            }
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct SourceIndex {
    schema_version: String,
    release: ReleaseMetadata,
    sources: Vec<SourceEntry>,
}

impl SourceIndex {
    #[must_use]
    pub fn sources(&self) -> &[SourceEntry] {
        &self.sources
    }

    pub(crate) fn schema_version(&self) -> &str {
        &self.schema_version
    }
}

#[derive(Debug, Deserialize)]
pub struct SourceEntry {
    id: String,
    kind: SourceKind,
    evidence: FeatureEvidence,
    platforms: Vec<SourcePlatform>,
    side_effects: Vec<String>,
    disposition: Option<SourceDisposition>,
}

impl SourceEntry {
    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }

    pub(crate) const fn kind(&self) -> SourceKind {
        self.kind
    }

    pub(crate) const fn evidence(&self) -> &FeatureEvidence {
        &self.evidence
    }

    pub(crate) fn disposition(&self) -> Option<&SourceDisposition> {
        self.disposition.as_ref()
    }

    fn has_required_metadata(&self) -> bool {
        !self.platforms.is_empty()
            && !self.side_effects.is_empty()
            && self
                .side_effects
                .iter()
                .all(|effect| !effect.trim().is_empty())
            && self
                .disposition
                .as_ref()
                .is_none_or(SourceDisposition::has_required_metadata)
    }
}

pub fn parse_feature_catalog(input: &str) -> yaml_serde::Result<FeatureCatalog> {
    yaml_serde::from_str(input)
}

pub fn parse_source_index(input: &str) -> yaml_serde::Result<SourceIndex> {
    yaml_serde::from_str(input)
}

#[must_use]
pub fn validate_feature_catalog(catalog: &FeatureCatalog) -> Vec<ValidationIssue> {
    let mut issues = Vec::new();
    let mut feature_ids = BTreeSet::new();

    for feature in &catalog.features {
        match split_feature_id(&feature.id) {
            Some((domain, _slug)) if domain != catalog.domain.as_str() => {
                issues.push(ValidationIssue::new(
                    ValidationCode::FeatureDomainMismatch,
                    feature.id.clone(),
                ));
            }
            Some(_) => {}
            None => issues.push(ValidationIssue::new(
                ValidationCode::InvalidFeatureId,
                feature.id.clone(),
            )),
        }

        if !feature_ids.insert(feature.id.as_str()) {
            issues.push(ValidationIssue::new(
                ValidationCode::DuplicateFeatureId,
                feature.id.clone(),
            ));
        }

        if !feature.has_required_metadata() {
            issues.push(ValidationIssue::new(
                ValidationCode::MissingFeatureMetadata,
                feature.id.clone(),
            ));
        }
    }

    issues
}

#[must_use]
pub fn validate_source_index(
    source_index: &SourceIndex,
    feature_ids: &BTreeSet<String>,
    expected_release_tag: &str,
    expected_release_commit: &str,
) -> Vec<ValidationIssue> {
    let mut issues = Vec::new();
    let mut source_ids = BTreeSet::new();

    if source_index.release.tag != expected_release_tag
        || source_index.release.tag_commit != expected_release_commit
    {
        issues.push(ValidationIssue::new(
            ValidationCode::ReleaseMismatch,
            "source-index.release",
        ));
    }

    for source in &source_index.sources {
        let expected_id = format!("{}:{}", source.kind.prefix(), source.evidence.symbol);
        if source.id != expected_id || !is_valid_source_symbol(&source.evidence.symbol) {
            issues.push(ValidationIssue::new(
                ValidationCode::InvalidSourceId,
                source.id.clone(),
            ));
        }

        if !source_ids.insert(source.id.as_str()) {
            issues.push(ValidationIssue::new(
                ValidationCode::DuplicateSourceId,
                source.id.clone(),
            ));
        }

        if !is_locked_upstream_path(source.evidence.path()) {
            issues.push(ValidationIssue::new(
                ValidationCode::InvalidEvidencePath,
                source.evidence.path.clone(),
            ));
        }

        if !source.has_required_metadata() {
            issues.push(ValidationIssue::new(
                ValidationCode::MissingSourceMetadata,
                source.id.clone(),
            ));
        }

        match source.disposition.as_ref() {
            Some(disposition) => {
                if let Some(feature_id) = disposition.feature_id()
                    && !feature_ids.contains(feature_id)
                {
                    issues.push(ValidationIssue::new(
                        ValidationCode::DanglingFeatureReference,
                        format!("{}:{feature_id}", source.id),
                    ));
                }
            }
            None => issues.push(ValidationIssue::new(
                ValidationCode::UnmappedSourceEntry,
                source.id.clone(),
            )),
        }
    }

    issues
}

pub(crate) fn is_locked_upstream_path(path: &str) -> bool {
    if path.is_empty() || path.contains('\\') || path.chars().any(char::is_control) {
        return false;
    }

    let path_value = Path::new(path);
    path.starts_with("upstream/CodexPlusPlus/")
        && !path_value.is_absolute()
        && !path_value.components().any(|component| {
            matches!(
                component,
                Component::ParentDir | Component::RootDir | Component::Prefix(_)
            )
        })
}

fn split_feature_id(id: &str) -> Option<(&str, &str)> {
    let (domain, slug) = id.strip_prefix("feature.")?.split_once('.')?;
    let valid_domain = matches!(
        domain,
        "foundation-platform"
            | "provider-network"
            | "session-data"
            | "plugin-script"
            | "remote-install"
    );
    let valid_slug = !slug.is_empty()
        && !slug.starts_with('-')
        && !slug.ends_with('-')
        && slug
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-');

    (valid_domain && valid_slug).then_some((domain, slug))
}

fn is_valid_source_symbol(symbol: &str) -> bool {
    !symbol.is_empty()
        && !symbol.starts_with('_')
        && !symbol.ends_with('_')
        && symbol
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_')
}
