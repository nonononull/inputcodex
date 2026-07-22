use std::collections::BTreeSet;

use serde::Deserialize;

use crate::{ValidationCode, ValidationIssue};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum FeatureDomain {
    FoundationPlatform,
    ProviderNetwork,
    SessionData,
    PluginScript,
    RemoteInstall,
}

impl FeatureDomain {
    const fn as_str(self) -> &'static str {
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
    #[serde(rename = "schema_version")]
    _schema_version: String,
    #[serde(rename = "release")]
    _release: ReleaseMetadata,
    domain: FeatureDomain,
    features: Vec<FeatureDefinition>,
}

#[derive(Debug, Deserialize)]
struct ReleaseMetadata {
    #[serde(rename = "tag")]
    _tag: String,
    #[serde(rename = "tag_commit")]
    _tag_commit: String,
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
}

#[derive(Debug, Deserialize)]
pub struct FeatureDefinition {
    id: String,
    #[serde(rename = "name")]
    _name: String,
    status: ParityStatus,
    #[serde(rename = "evidence")]
    _evidence: Vec<FeatureEvidence>,
    #[serde(rename = "entry_points")]
    _entry_points: Vec<String>,
    #[serde(rename = "platforms")]
    _platforms: FeaturePlatforms,
    #[serde(rename = "decision_refs")]
    _decision_refs: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct FeatureEvidence {
    #[serde(rename = "path")]
    _path: String,
    #[serde(rename = "symbol")]
    _symbol: String,
}

#[derive(Debug, Deserialize)]
struct FeaturePlatforms {
    #[serde(rename = "windows")]
    _windows: FeaturePlatform,
    #[serde(rename = "macos")]
    _macos: FeaturePlatform,
}

#[derive(Debug, Deserialize)]
struct FeaturePlatform {
    #[serde(rename = "applicability")]
    _applicability: String,
    #[serde(rename = "semantics")]
    _semantics: String,
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
}

pub fn parse_feature_catalog(input: &str) -> yaml_serde::Result<FeatureCatalog> {
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
    }

    issues
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
