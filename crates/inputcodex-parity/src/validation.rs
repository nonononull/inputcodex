use std::{
    collections::{BTreeMap, BTreeSet},
    error::Error,
    fmt, fs,
    path::Path,
};

use serde::Deserialize;

use crate::{
    FeatureDomain, ParityStatus, SourceDisposition, SourceKind, parse_contract_catalog,
    parse_feature_catalog, parse_source_index, validate_feature_catalog, validate_source_index,
};

const SOURCE_INDEX_PATH: &str = "parity/features/source-index.yml";
const SOURCE_LOCK_PATH: &str = "upstream/source-lock.json";
const COMMANDS_PATH: &str =
    "upstream/CodexPlusPlus/apps/codex-plus-manager/src-tauri/src/commands.rs";
const CORE_LIB_PATH: &str = "upstream/CodexPlusPlus/crates/codex-plus-core/src/lib.rs";
const DATA_LIB_PATH: &str = "upstream/CodexPlusPlus/crates/codex-plus-data/src/lib.rs";
const FEATURE_CATALOG_SCHEMA: &str = "inputcodex.feature-catalog.v1";
const SOURCE_INDEX_SCHEMA: &str = "inputcodex.source-index.v1";

const DOMAIN_FILES: [(FeatureDomain, &str); 5] = [
    (FeatureDomain::FoundationPlatform, "foundation-platform.yml"),
    (FeatureDomain::ProviderNetwork, "provider-network.yml"),
    (FeatureDomain::SessionData, "session-data.yml"),
    (FeatureDomain::PluginScript, "plugin-script.yml"),
    (FeatureDomain::RemoteInstall, "remote-install.yml"),
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationCode {
    InvalidFeatureId,
    DuplicateFeatureId,
    FeatureDomainMismatch,
    MissingFeatureMetadata,
    InvalidSourceId,
    DuplicateSourceId,
    UnmappedSourceEntry,
    ReleaseMismatch,
    SchemaVersionMismatch,
    InvalidEvidencePath,
    MissingSourceMetadata,
    MissingSourceEntry,
    UnexpectedSourceEntry,
    SourceEvidenceMismatch,
    SourceFeatureMappingMismatch,
    InvalidInitialParityStatus,
    DuplicateContractId,
    ContractIdentityMismatch,
    MissingLoadingState,
    DanglingFeatureReference,
    DanglingFixtureReference,
    CrossFeatureFixtureReference,
    DuplicateFixtureId,
    FixtureFeatureMismatch,
    InvalidFixturePath,
    InvalidFixturePayload,
    PrivateAbsolutePath,
    SensitiveFixtureValue,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationIssue {
    code: ValidationCode,
    location: String,
}

impl ValidationIssue {
    pub(crate) fn new(code: ValidationCode, location: impl Into<String>) -> Self {
        Self {
            code,
            location: location.into(),
        }
    }

    #[must_use]
    pub const fn code(&self) -> ValidationCode {
        self.code
    }

    #[must_use]
    pub fn location(&self) -> &str {
        &self.location
    }
}

#[derive(Debug, Default)]
pub struct RepositorySummary {
    source_entry_count: usize,
    feature_count: usize,
    contract_count: usize,
    fixture_count: usize,
    excluded_entry_count: usize,
    exception_pending_count: usize,
    coverage_gap_count: usize,
}

impl RepositorySummary {
    #[must_use]
    pub const fn source_entry_count(&self) -> usize {
        self.source_entry_count
    }

    #[must_use]
    pub const fn feature_count(&self) -> usize {
        self.feature_count
    }

    #[must_use]
    pub const fn contract_count(&self) -> usize {
        self.contract_count
    }

    #[must_use]
    pub const fn fixture_count(&self) -> usize {
        self.fixture_count
    }

    #[must_use]
    pub const fn excluded_entry_count(&self) -> usize {
        self.excluded_entry_count
    }

    #[must_use]
    pub const fn exception_pending_count(&self) -> usize {
        self.exception_pending_count
    }

    #[must_use]
    pub const fn coverage_gap_count(&self) -> usize {
        self.coverage_gap_count
    }
}

#[derive(Debug)]
pub struct RepositoryValidationError {
    message: String,
    issues: Vec<ValidationIssue>,
}

impl RepositoryValidationError {
    fn message(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            issues: Vec::new(),
        }
    }

    fn validation(issues: Vec<ValidationIssue>) -> Self {
        Self {
            message: format!("仓库一致性验证失败，共 {} 项", issues.len()),
            issues,
        }
    }

    #[must_use]
    pub fn issues(&self) -> &[ValidationIssue] {
        &self.issues
    }
}

impl fmt::Display for RepositoryValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.message)?;
        for issue in &self.issues {
            write!(formatter, "; {:?}@{}", issue.code, issue.location)?;
        }
        Ok(())
    }
}

impl Error for RepositoryValidationError {}

#[derive(Debug, Deserialize)]
struct SourceLock {
    snapshot: SourceLockSnapshot,
}

#[derive(Debug, Deserialize)]
struct SourceLockSnapshot {
    release_tag: String,
    commit: String,
}

#[derive(Debug)]
struct ExpectedSource {
    kind: SourceKind,
    path: String,
    symbol: String,
}

struct FeatureRepositoryState {
    summary: RepositorySummary,
}

pub fn validate_feature_repository(
    repository_root: &Path,
) -> Result<RepositorySummary, RepositoryValidationError> {
    load_feature_repository(repository_root).map(|state| state.summary)
}

pub fn validate_repository(
    repository_root: &Path,
) -> Result<RepositorySummary, RepositoryValidationError> {
    let mut state = load_feature_repository(repository_root)?;
    let contracts_root = repository_root.join("parity/contracts");
    let mut contract_count = 0;

    for (_, file_name) in DOMAIN_FILES {
        let contract_path = contracts_root.join(file_name);
        let contract_text = read_utf8(&contract_path)?;
        let contracts = parse_contract_catalog(&contract_text).map_err(|error| {
            RepositoryValidationError::message(format!(
                "无法解析 {}：{error}",
                contract_path.display()
            ))
        })?;
        contract_count += contracts.contracts().len();
    }

    if contract_count == 0 {
        return Err(RepositoryValidationError::message(
            "parity/contracts 尚未包含行为合同",
        ));
    }

    state.summary.contract_count = contract_count;
    Ok(state.summary)
}

fn load_feature_repository(
    repository_root: &Path,
) -> Result<FeatureRepositoryState, RepositoryValidationError> {
    let source_lock_path = repository_root.join(SOURCE_LOCK_PATH);
    let source_lock_text = read_utf8(&source_lock_path)?;
    let source_lock: SourceLock = yaml_serde::from_str(&source_lock_text).map_err(|error| {
        RepositoryValidationError::message(format!(
            "无法解析 {}：{error}",
            source_lock_path.display()
        ))
    })?;

    let source_index_path = repository_root.join(SOURCE_INDEX_PATH);
    let source_index_text = read_utf8(&source_index_path)?;
    let source_index = parse_source_index(&source_index_text).map_err(|error| {
        RepositoryValidationError::message(format!(
            "无法解析 {}：{error}",
            source_index_path.display()
        ))
    })?;

    let mut issues = Vec::new();
    let mut feature_ids = BTreeSet::new();
    let mut feature_statuses = BTreeMap::new();
    let mut feature_entry_points = BTreeMap::<String, BTreeSet<String>>::new();
    let mut feature_count = 0;

    if source_index.schema_version() != SOURCE_INDEX_SCHEMA {
        issues.push(ValidationIssue::new(
            ValidationCode::SchemaVersionMismatch,
            SOURCE_INDEX_PATH,
        ));
    }

    for (expected_domain, file_name) in DOMAIN_FILES {
        let relative_path = format!("parity/features/{file_name}");
        let catalog_path = repository_root.join(&relative_path);
        let catalog_text = read_utf8(&catalog_path)?;
        let catalog = parse_feature_catalog(&catalog_text).map_err(|error| {
            RepositoryValidationError::message(format!(
                "无法解析 {}：{error}",
                catalog_path.display()
            ))
        })?;

        if catalog.schema_version() != FEATURE_CATALOG_SCHEMA {
            issues.push(ValidationIssue::new(
                ValidationCode::SchemaVersionMismatch,
                relative_path.clone(),
            ));
        }
        if catalog.domain() != expected_domain {
            issues.push(ValidationIssue::new(
                ValidationCode::FeatureDomainMismatch,
                relative_path.clone(),
            ));
        }
        if catalog.release().tag() != source_lock.snapshot.release_tag
            || catalog.release().tag_commit() != source_lock.snapshot.commit
        {
            issues.push(ValidationIssue::new(
                ValidationCode::ReleaseMismatch,
                relative_path.clone(),
            ));
        }

        issues.extend(validate_feature_catalog(&catalog));

        for feature in catalog.features() {
            feature_count += 1;
            if !feature_ids.insert(feature.id().to_owned()) {
                issues.push(ValidationIssue::new(
                    ValidationCode::DuplicateFeatureId,
                    feature.id(),
                ));
            }
            if !matches!(
                feature.status(),
                ParityStatus::Unassessed | ParityStatus::ExceptionPending
            ) {
                issues.push(ValidationIssue::new(
                    ValidationCode::InvalidInitialParityStatus,
                    feature.id(),
                ));
            }

            feature_statuses.insert(feature.id().to_owned(), feature.status());
            feature_entry_points.insert(
                feature.id().to_owned(),
                feature.entry_points().iter().cloned().collect(),
            );

            for evidence in feature.evidence() {
                if !crate::catalog::is_locked_upstream_path(evidence.path())
                    || evidence.symbol().trim().is_empty()
                    || !repository_root.join(evidence.path()).is_file()
                {
                    issues.push(ValidationIssue::new(
                        ValidationCode::InvalidEvidencePath,
                        format!("{}:{}", feature.id(), evidence.path()),
                    ));
                }
            }
        }
    }

    issues.extend(validate_source_index(
        &source_index,
        &feature_ids,
        &source_lock.snapshot.release_tag,
        &source_lock.snapshot.commit,
    ));

    let expected_sources = enumerate_expected_sources(repository_root)?;
    let source_entries = source_index
        .sources()
        .iter()
        .map(|source| (source.id(), source))
        .collect::<BTreeMap<_, _>>();

    for (source_id, expected) in &expected_sources {
        match source_entries.get(source_id.as_str()) {
            Some(source)
                if source.kind() == expected.kind
                    && source.evidence().path() == expected.path
                    && source.evidence().symbol() == expected.symbol => {}
            Some(_) => issues.push(ValidationIssue::new(
                ValidationCode::SourceEvidenceMismatch,
                source_id,
            )),
            None => issues.push(ValidationIssue::new(
                ValidationCode::MissingSourceEntry,
                source_id,
            )),
        }
    }

    for source in source_index.sources() {
        if !expected_sources.contains_key(source.id()) {
            issues.push(ValidationIssue::new(
                ValidationCode::UnexpectedSourceEntry,
                source.id(),
            ));
        }

        if !repository_root.join(source.evidence().path()).is_file() {
            issues.push(ValidationIssue::new(
                ValidationCode::InvalidEvidencePath,
                format!("{}:{}", source.id(), source.evidence().path()),
            ));
        }

        let Some(disposition) = source.disposition() else {
            continue;
        };
        let Some(feature_id) = disposition.feature_id() else {
            continue;
        };

        if feature_entry_points
            .get(feature_id)
            .is_none_or(|entry_points| !entry_points.contains(source.id()))
        {
            issues.push(ValidationIssue::new(
                ValidationCode::SourceFeatureMappingMismatch,
                format!("{}:{feature_id}", source.id()),
            ));
        }

        if feature_statuses.get(feature_id).is_some_and(|status| {
            disposition.is_exception_pending() != (*status == ParityStatus::ExceptionPending)
        }) {
            issues.push(ValidationIssue::new(
                ValidationCode::SourceFeatureMappingMismatch,
                format!("{}:{feature_id}:status", source.id()),
            ));
        }
    }

    for (feature_id, entry_points) in &feature_entry_points {
        for entry_point in entry_points {
            let mapping_matches = source_entries
                .get(entry_point.as_str())
                .and_then(|source| source.disposition())
                .and_then(SourceDisposition::feature_id)
                .is_some_and(|mapped_feature_id| mapped_feature_id == feature_id);
            if !mapping_matches {
                issues.push(ValidationIssue::new(
                    ValidationCode::SourceFeatureMappingMismatch,
                    format!("{feature_id}:{entry_point}"),
                ));
            }
        }
    }

    if !issues.is_empty() {
        return Err(RepositoryValidationError::validation(issues));
    }

    let excluded_entry_count = source_index
        .sources()
        .iter()
        .filter(|source| {
            matches!(
                source.disposition(),
                Some(SourceDisposition::Excluded { .. })
            )
        })
        .count();
    let exception_pending_count = feature_statuses
        .values()
        .filter(|status| **status == ParityStatus::ExceptionPending)
        .count();

    Ok(FeatureRepositoryState {
        summary: RepositorySummary {
            source_entry_count: source_index.sources().len(),
            feature_count,
            contract_count: 0,
            fixture_count: 0,
            excluded_entry_count,
            exception_pending_count,
            coverage_gap_count: 0,
        },
    })
}

fn enumerate_expected_sources(
    repository_root: &Path,
) -> Result<BTreeMap<String, ExpectedSource>, RepositoryValidationError> {
    let mut sources = BTreeMap::new();
    let commands = read_utf8(&repository_root.join(COMMANDS_PATH))?;
    for symbol in parse_tauri_commands(&commands) {
        insert_expected_source(
            &mut sources,
            SourceKind::TauriCommand,
            COMMANDS_PATH,
            symbol,
        );
    }

    let core_modules = read_utf8(&repository_root.join(CORE_LIB_PATH))?;
    for symbol in parse_public_modules(&core_modules) {
        let module_file = format!("upstream/CodexPlusPlus/crates/codex-plus-core/src/{symbol}.rs");
        let module_path = if repository_root.join(&module_file).is_file() {
            module_file
        } else {
            format!("upstream/CodexPlusPlus/crates/codex-plus-core/src/{symbol}/mod.rs")
        };
        insert_expected_source(&mut sources, SourceKind::CoreModule, &module_path, symbol);
    }

    let data_modules = read_utf8(&repository_root.join(DATA_LIB_PATH))?;
    for symbol in parse_public_modules(&data_modules) {
        let module_path = format!("upstream/CodexPlusPlus/crates/codex-plus-data/src/{symbol}.rs");
        insert_expected_source(&mut sources, SourceKind::DataModule, &module_path, symbol);
    }

    Ok(sources)
}

fn insert_expected_source(
    sources: &mut BTreeMap<String, ExpectedSource>,
    kind: SourceKind,
    path: &str,
    symbol: String,
) {
    let prefix = match kind {
        SourceKind::TauriCommand => "tauri-command",
        SourceKind::CoreModule => "core-module",
        SourceKind::DataModule => "data-module",
    };
    sources.insert(
        format!("{prefix}:{symbol}"),
        ExpectedSource {
            kind,
            path: path.to_owned(),
            symbol,
        },
    );
}

fn parse_tauri_commands(input: &str) -> BTreeSet<String> {
    let mut commands = BTreeSet::new();
    let mut command_attribute_seen = false;

    for line in input.lines() {
        let trimmed = line.trim();
        if trimmed == "#[tauri::command]" {
            command_attribute_seen = true;
            continue;
        }
        if command_attribute_seen && let Some(function_name) = parse_public_function_name(trimmed) {
            commands.insert(function_name.to_owned());
            command_attribute_seen = false;
        }
    }

    commands
}

fn parse_public_function_name(line: &str) -> Option<&str> {
    let declaration = line.strip_prefix("pub ")?;
    let declaration = declaration.strip_prefix("async ").unwrap_or(declaration);
    let declaration = declaration.strip_prefix("fn ")?;
    let name_end = declaration
        .find(|character: char| !(character.is_ascii_alphanumeric() || character == '_'))
        .unwrap_or(declaration.len());
    (name_end > 0).then_some(&declaration[..name_end])
}

fn parse_public_modules(input: &str) -> BTreeSet<String> {
    input
        .lines()
        .filter_map(|line| {
            line.trim()
                .strip_prefix("pub mod ")?
                .strip_suffix(';')
                .map(str::to_owned)
        })
        .collect()
}

fn read_utf8(path: &Path) -> Result<String, RepositoryValidationError> {
    fs::read_to_string(path).map_err(|error| {
        RepositoryValidationError::message(format!("无法读取 {}：{error}", path.display()))
    })
}
