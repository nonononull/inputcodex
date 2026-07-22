use std::{
    collections::BTreeSet,
    path::{Component, Path},
};

use serde::Deserialize;
use yaml_serde::Value;

use crate::{ValidationCode, ValidationIssue};

#[derive(Debug, Deserialize)]
pub struct FixtureManifest {
    schema_version: String,
    feature_id: String,
    fixtures: Vec<FixtureDefinition>,
}

impl FixtureManifest {
    pub(crate) fn schema_version(&self) -> &str {
        &self.schema_version
    }

    pub(crate) fn feature_id(&self) -> &str {
        &self.feature_id
    }

    pub(crate) fn fixtures(&self) -> &[FixtureDefinition] {
        &self.fixtures
    }
}

#[derive(Debug, Deserialize)]
pub(crate) struct FixtureDefinition {
    id: String,
    scenario: String,
    #[serde(rename = "kind")]
    _kind: FixtureKind,
    files: Vec<FixtureFile>,
}

impl FixtureDefinition {
    pub(crate) fn id(&self) -> &str {
        &self.id
    }

    pub(crate) fn files(&self) -> &[FixtureFile] {
        &self.files
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
enum FixtureKind {
    Synthetic,
    Redacted,
}

#[derive(Debug, Deserialize)]
pub(crate) struct FixtureFile {
    path: String,
    #[serde(rename = "format")]
    _format: String,
    #[serde(rename = "description")]
    _description: String,
}

impl FixtureFile {
    pub(crate) fn path(&self) -> &str {
        &self.path
    }
}

pub fn parse_fixture_manifest(input: &str) -> yaml_serde::Result<FixtureManifest> {
    yaml_serde::from_str(input)
}

#[must_use]
pub fn validate_fixture_manifest(
    manifest: &FixtureManifest,
    manifest_root: &Path,
) -> Vec<ValidationIssue> {
    let mut issues = Vec::new();
    let mut fixture_ids = BTreeSet::new();

    if manifest_root.file_name().and_then(|name| name.to_str()) != Some(&manifest.feature_id) {
        issues.push(ValidationIssue::new(
            ValidationCode::FixtureDirectoryMismatch,
            manifest_root.display().to_string(),
        ));
    }

    for fixture in &manifest.fixtures {
        if !fixture_ids.insert(fixture.id.as_str()) {
            issues.push(ValidationIssue::new(
                ValidationCode::DuplicateFixtureId,
                fixture.id.clone(),
            ));
        }

        let expected_id = format!("fixture.{}.{}", manifest.feature_id, fixture.scenario);
        if fixture.id != expected_id {
            issues.push(ValidationIssue::new(
                ValidationCode::FixtureFeatureMismatch,
                fixture.id.clone(),
            ));
        }

        for file in &fixture.files {
            if !is_repository_relative_path(&file.path) {
                issues.push(ValidationIssue::new(
                    ValidationCode::InvalidFixturePath,
                    format!("{}:{}", manifest_root.display(), file.path),
                ));
            }
        }
    }

    issues
}

#[must_use]
pub fn validate_fixture_payload(file_name: &str, payload: &[u8]) -> Vec<ValidationIssue> {
    let text = match std::str::from_utf8(payload) {
        Ok(text) => text,
        Err(_) => {
            return vec![ValidationIssue::new(
                ValidationCode::InvalidFixturePayload,
                file_name,
            )];
        }
    };
    let value = match yaml_serde::from_str::<Value>(text) {
        Ok(value) => value,
        Err(_) => {
            return vec![ValidationIssue::new(
                ValidationCode::InvalidFixturePayload,
                file_name,
            )];
        }
    };
    let mut issues = Vec::new();

    inspect_value(file_name, None, &value, &mut issues);
    issues
}

fn is_repository_relative_path(path: &str) -> bool {
    if path.is_empty() || path.contains('\\') || path.chars().any(char::is_control) {
        return false;
    }

    let path = Path::new(path);
    !path.is_absolute()
        && !path.components().any(|component| {
            matches!(
                component,
                Component::ParentDir | Component::RootDir | Component::Prefix(_)
            )
        })
}

fn inspect_value(
    file_name: &str,
    key: Option<&str>,
    value: &Value,
    issues: &mut Vec<ValidationIssue>,
) {
    match value {
        Value::String(value) => {
            if Path::new(value).is_absolute()
                || value.starts_with('/')
                || looks_like_windows_absolute_path(value)
            {
                issues.push(ValidationIssue::new(
                    ValidationCode::PrivateAbsolutePath,
                    file_name,
                ));
            }
            if key.is_some_and(is_sensitive_key)
                && !is_explicit_placeholder(value)
                && looks_like_sensitive_value(value)
            {
                issues.push(ValidationIssue::new(
                    ValidationCode::SensitiveFixtureValue,
                    file_name,
                ));
            }
        }
        Value::Sequence(values) => {
            for value in values {
                inspect_value(file_name, key, value, issues);
            }
        }
        Value::Mapping(values) => {
            for (mapping_key, mapping_value) in values {
                let mapping_key = match mapping_key {
                    Value::String(mapping_key) => Some(mapping_key.as_str()),
                    _ => None,
                };
                inspect_value(file_name, mapping_key, mapping_value, issues);
            }
        }
        Value::Tagged(tagged) => inspect_value(file_name, key, &tagged.value, issues),
        Value::Null | Value::Bool(_) | Value::Number(_) => {}
    }
}

fn looks_like_windows_absolute_path(value: &str) -> bool {
    let bytes = value.as_bytes();
    (bytes.len() >= 3
        && bytes[0].is_ascii_alphabetic()
        && bytes[1] == b':'
        && matches!(bytes[2], b'\\' | b'/'))
        || value.starts_with("\\\\")
}

fn is_sensitive_key(key: &str) -> bool {
    let key = key.to_ascii_lowercase().replace('-', "_");
    key.contains("token")
        || key.contains("secret")
        || key.contains("password")
        || key.contains("cookie")
        || key.contains("private_key")
        || key.contains("signing_key")
}

fn is_explicit_placeholder(value: &str) -> bool {
    let value = value.to_ascii_lowercase();
    value.contains("synthetic")
        || value.contains("redacted")
        || value.contains("example")
        || value.contains("placeholder")
}

fn looks_like_sensitive_value(value: &str) -> bool {
    value.starts_with("sk-") || value.len() >= 12
}
