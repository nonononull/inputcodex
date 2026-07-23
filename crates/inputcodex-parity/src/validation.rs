use std::{
    collections::{BTreeMap, BTreeSet},
    error::Error,
    fmt, fs,
    path::Path,
};

use serde::Deserialize;

use crate::{
    FeatureDomain, ParityStatus, SourceDisposition, SourceKind, parse_contract_catalog,
    parse_feature_catalog, parse_fixture_manifest, parse_source_index, validate_contract_catalog,
    validate_contract_catalog_domain, validate_feature_catalog, validate_fixture_manifest,
    validate_fixture_payload, validate_source_index,
};

const SOURCE_INDEX_PATH: &str = "parity/features/source-index.yml";
const SOURCE_LOCK_PATH: &str = "upstream/source-lock.json";
const COMMANDS_PATH: &str =
    "upstream/CodexPlusPlus/apps/codex-plus-manager/src-tauri/src/commands.rs";
const CORE_LIB_PATH: &str = "upstream/CodexPlusPlus/crates/codex-plus-core/src/lib.rs";
const DATA_LIB_PATH: &str = "upstream/CodexPlusPlus/crates/codex-plus-data/src/lib.rs";
const FEATURE_CATALOG_SCHEMA: &str = "inputcodex.feature-catalog.v1";
const SOURCE_INDEX_SCHEMA: &str = "inputcodex.source-index.v1";
const CONTRACT_CATALOG_SCHEMA: &str = "inputcodex.behavior-contract.v1";
const FIXTURE_MANIFEST_SCHEMA: &str = "inputcodex.fixture-manifest.v1";
const RELEASE_AUDIT_SCHEMA: &str = "inputcodex.release-audit.v1";
const RELEASE_AUDIT_CURRENT: &str = "current";
const RELEASE_AUDIT_STALE: &str = "stale-re-audit-required";
const RE_AUDIT_ISSUE_URL_PREFIX: &str = "https://github.com/nonononull/inputcodex/issues/";

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
    ContractDomainMismatch,
    MissingFeatureContract,
    FixturePolicyMismatch,
    MissingLoadingState,
    DanglingFeatureReference,
    DanglingFixtureReference,
    CrossFeatureFixtureReference,
    DuplicateFixtureId,
    FixtureFeatureMismatch,
    FixtureDirectoryMismatch,
    MissingFixtureFile,
    UnexpectedFixtureFile,
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
    requires_reaudit: bool,
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

    #[must_use]
    pub const fn requires_reaudit(&self) -> bool {
        self.requires_reaudit
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
    release_audit: SourceLockReleaseAudit,
}

#[derive(Debug, Deserialize)]
struct SourceLockSnapshot {
    release_tag: String,
    commit: String,
}

#[derive(Debug, Deserialize)]
struct SourceLockReleaseAudit {
    schema_version: String,
    catalog_release: SourceLockCatalogRelease,
    status: String,
    stale_reason: Option<String>,
    re_audit_issue_ref: Option<String>,
}

#[derive(Debug, Deserialize)]
struct SourceLockCatalogRelease {
    tag: String,
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
    feature_ids: BTreeSet<String>,
}

struct FixtureRepositoryState {
    fixture_ids: BTreeSet<String>,
    fixture_count: usize,
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
    let fixture_state = load_fixture_repository(repository_root, &state.feature_ids)?;
    let contracts_root = repository_root.join("parity/contracts");
    let mut contract_count = 0;
    let mut contract_ids = BTreeSet::new();
    let mut contracted_feature_ids = BTreeSet::new();
    let mut issues = Vec::new();

    for (expected_domain, file_name) in DOMAIN_FILES {
        let contract_path = contracts_root.join(file_name);
        let contract_text = read_utf8(&contract_path)?;
        let contracts = parse_contract_catalog(&contract_text).map_err(|error| {
            RepositoryValidationError::message(format!(
                "无法解析 {}：{error}",
                contract_path.display()
            ))
        })?;

        if contracts.schema_version() != CONTRACT_CATALOG_SCHEMA {
            issues.push(ValidationIssue::new(
                ValidationCode::SchemaVersionMismatch,
                contract_path.display().to_string(),
            ));
        }
        issues.extend(validate_contract_catalog_domain(
            &contracts,
            expected_domain,
        ));
        issues.extend(validate_contract_catalog(
            &contracts,
            &state.feature_ids,
            &fixture_state.fixture_ids,
        ));

        let expected_feature_prefix = format!("feature.{}.", expected_domain.as_str());
        for contract in contracts.contracts() {
            if !contract_ids.insert(contract.id().to_owned()) {
                issues.push(ValidationIssue::new(
                    ValidationCode::DuplicateContractId,
                    contract.id(),
                ));
            }
            if !contract.feature_id().starts_with(&expected_feature_prefix) {
                issues.push(ValidationIssue::new(
                    ValidationCode::ContractDomainMismatch,
                    format!("{}:{}", contract.id(), expected_domain.as_str()),
                ));
            }
            contracted_feature_ids.insert(contract.feature_id().to_owned());
            for fixture_id in contract.fixture_refs() {
                if !fixture_state.fixture_ids.contains(fixture_id) {
                    issues.push(ValidationIssue::new(
                        ValidationCode::DanglingFixtureReference,
                        format!("{}:{fixture_id}", contract.id()),
                    ));
                }
            }
        }
        contract_count += contracts.contracts().len();
    }

    for feature_id in &state.feature_ids {
        if !contracted_feature_ids.contains(feature_id) {
            issues.push(ValidationIssue::new(
                ValidationCode::MissingFeatureContract,
                feature_id,
            ));
        }
    }

    if !issues.is_empty() {
        return Err(RepositoryValidationError::validation(issues));
    }

    state.summary.contract_count = contract_count;
    state.summary.fixture_count = fixture_state.fixture_count;
    Ok(state.summary)
}

fn load_fixture_repository(
    repository_root: &Path,
    feature_ids: &BTreeSet<String>,
) -> Result<FixtureRepositoryState, RepositoryValidationError> {
    let fixtures_root = repository_root.join("parity/fixtures");
    let mut fixture_directories = fs::read_dir(&fixtures_root)
        .map_err(|error| {
            RepositoryValidationError::message(format!(
                "无法读取 {}：{error}",
                fixtures_root.display()
            ))
        })?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| {
            RepositoryValidationError::message(format!(
                "无法枚举 {}：{error}",
                fixtures_root.display()
            ))
        })?;
    fixture_directories.sort_by_key(std::fs::DirEntry::file_name);

    let mut fixture_ids = BTreeSet::new();
    let mut fixture_count = 0;
    let mut issues = Vec::new();

    for directory in fixture_directories {
        let metadata = directory.metadata().map_err(|error| {
            RepositoryValidationError::message(format!(
                "无法读取 fixture 元数据 {}：{error}",
                directory.path().display()
            ))
        })?;
        if !metadata.is_dir() || directory.file_type().is_ok_and(|kind| kind.is_symlink()) {
            issues.push(ValidationIssue::new(
                ValidationCode::InvalidFixturePath,
                directory.path().display().to_string(),
            ));
            continue;
        }

        let manifest_root = directory.path();
        let manifest_path = manifest_root.join("manifest.yml");
        let manifest_text = read_utf8(&manifest_path)?;
        let manifest = parse_fixture_manifest(&manifest_text).map_err(|error| {
            RepositoryValidationError::message(format!(
                "无法解析 {}：{error}",
                manifest_path.display()
            ))
        })?;

        if manifest.schema_version() != FIXTURE_MANIFEST_SCHEMA {
            issues.push(ValidationIssue::new(
                ValidationCode::SchemaVersionMismatch,
                manifest_path.display().to_string(),
            ));
        }
        if !feature_ids.contains(manifest.feature_id()) {
            issues.push(ValidationIssue::new(
                ValidationCode::DanglingFeatureReference,
                manifest.feature_id(),
            ));
        }
        issues.extend(validate_fixture_manifest(&manifest, &manifest_root));

        let mut declared_files = BTreeSet::new();
        for fixture in manifest.fixtures() {
            fixture_count += 1;
            if !fixture_ids.insert(fixture.id().to_owned()) {
                issues.push(ValidationIssue::new(
                    ValidationCode::DuplicateFixtureId,
                    fixture.id(),
                ));
            }

            for fixture_file in fixture.files() {
                declared_files.insert(fixture_file.path().to_owned());
                let payload_path = manifest_root.join(fixture_file.path());
                let payload_metadata = match fs::symlink_metadata(&payload_path) {
                    Ok(metadata) if metadata.is_file() && !metadata.file_type().is_symlink() => {
                        metadata
                    }
                    Ok(_) => {
                        issues.push(ValidationIssue::new(
                            ValidationCode::InvalidFixturePath,
                            payload_path.display().to_string(),
                        ));
                        continue;
                    }
                    Err(_) => {
                        issues.push(ValidationIssue::new(
                            ValidationCode::MissingFixtureFile,
                            payload_path.display().to_string(),
                        ));
                        continue;
                    }
                };
                if payload_metadata.len() == 0 {
                    issues.push(ValidationIssue::new(
                        ValidationCode::InvalidFixturePayload,
                        payload_path.display().to_string(),
                    ));
                    continue;
                }

                let canonical_root = fs::canonicalize(&manifest_root).map_err(|error| {
                    RepositoryValidationError::message(format!(
                        "无法规范化 {}：{error}",
                        manifest_root.display()
                    ))
                })?;
                let canonical_payload = fs::canonicalize(&payload_path).map_err(|error| {
                    RepositoryValidationError::message(format!(
                        "无法规范化 {}：{error}",
                        payload_path.display()
                    ))
                })?;
                if !canonical_payload.starts_with(&canonical_root) {
                    issues.push(ValidationIssue::new(
                        ValidationCode::InvalidFixturePath,
                        payload_path.display().to_string(),
                    ));
                    continue;
                }

                let payload = fs::read(&payload_path).map_err(|error| {
                    RepositoryValidationError::message(format!(
                        "无法读取 {}：{error}",
                        payload_path.display()
                    ))
                })?;
                issues.extend(validate_fixture_payload(
                    &payload_path.display().to_string(),
                    &payload,
                ));
            }
        }

        for relative_file in collect_fixture_files(&manifest_root)? {
            if relative_file != "manifest.yml" && !declared_files.contains(&relative_file) {
                issues.push(ValidationIssue::new(
                    ValidationCode::UnexpectedFixtureFile,
                    format!("{}:{relative_file}", manifest_root.display()),
                ));
            }
        }
    }

    if !issues.is_empty() {
        return Err(RepositoryValidationError::validation(issues));
    }

    Ok(FixtureRepositoryState {
        fixture_ids,
        fixture_count,
    })
}

fn collect_fixture_files(
    fixture_root: &Path,
) -> Result<BTreeSet<String>, RepositoryValidationError> {
    let mut pending = vec![fixture_root.to_path_buf()];
    let mut files = BTreeSet::new();

    while let Some(directory) = pending.pop() {
        let entries = fs::read_dir(&directory)
            .map_err(|error| {
                RepositoryValidationError::message(format!(
                    "无法枚举 {}：{error}",
                    directory.display()
                ))
            })?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|error| {
                RepositoryValidationError::message(format!(
                    "无法读取 {}：{error}",
                    directory.display()
                ))
            })?;

        for entry in entries {
            let file_type = entry.file_type().map_err(|error| {
                RepositoryValidationError::message(format!(
                    "无法读取 fixture 类型 {}：{error}",
                    entry.path().display()
                ))
            })?;
            if file_type.is_symlink() {
                return Err(RepositoryValidationError::validation(vec![
                    ValidationIssue::new(
                        ValidationCode::InvalidFixturePath,
                        entry.path().display().to_string(),
                    ),
                ]));
            }
            if file_type.is_dir() {
                pending.push(entry.path());
            } else if file_type.is_file() {
                let relative = entry
                    .path()
                    .strip_prefix(fixture_root)
                    .map_err(|error| {
                        RepositoryValidationError::message(format!(
                            "无法计算 fixture 相对路径 {}：{error}",
                            entry.path().display()
                        ))
                    })?
                    .to_string_lossy()
                    .replace('\\', "/");
                files.insert(relative);
            }
        }
    }

    Ok(files)
}

fn validate_release_audit(source_lock: &SourceLock, issues: &mut Vec<ValidationIssue>) -> bool {
    let audit = &source_lock.release_audit;
    let location = "upstream/source-lock.json.release_audit";
    let mut valid = true;

    if audit.schema_version != RELEASE_AUDIT_SCHEMA {
        valid = false;
        issues.push(ValidationIssue::new(
            ValidationCode::SchemaVersionMismatch,
            location,
        ));
    }

    let snapshot_matches_catalog = source_lock.snapshot.release_tag == audit.catalog_release.tag
        && source_lock.snapshot.commit == audit.catalog_release.commit;

    match audit.status.as_str() {
        RELEASE_AUDIT_CURRENT => {
            if !snapshot_matches_catalog
                || audit.stale_reason.is_some()
                || audit.re_audit_issue_ref.is_some()
            {
                issues.push(ValidationIssue::new(
                    ValidationCode::ReleaseMismatch,
                    location,
                ));
            }
            false
        }
        RELEASE_AUDIT_STALE => {
            if snapshot_matches_catalog {
                valid = false;
                issues.push(ValidationIssue::new(
                    ValidationCode::ReleaseMismatch,
                    location,
                ));
            }
            if !audit
                .stale_reason
                .as_deref()
                .is_some_and(|reason| !reason.trim().is_empty())
            {
                valid = false;
                issues.push(ValidationIssue::new(
                    ValidationCode::ReleaseMismatch,
                    location,
                ));
            }
            if !is_valid_reaudit_issue_ref(audit.re_audit_issue_ref.as_deref()) {
                valid = false;
                issues.push(ValidationIssue::new(
                    ValidationCode::ReleaseMismatch,
                    location,
                ));
            }
            valid
        }
        _ => {
            issues.push(ValidationIssue::new(
                ValidationCode::ReleaseMismatch,
                location,
            ));
            false
        }
    }
}

fn is_valid_reaudit_issue_ref(value: Option<&str>) -> bool {
    let Some(value) = value else {
        return false;
    };
    let Some(issue_number) = value.strip_prefix(RE_AUDIT_ISSUE_URL_PREFIX) else {
        return false;
    };

    issue_number
        .as_bytes()
        .first()
        .is_some_and(|byte| *byte != b'0')
        && issue_number.bytes().all(|byte| byte.is_ascii_digit())
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
    let requires_reaudit = validate_release_audit(&source_lock, &mut issues);
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
        if catalog.release().tag() != source_lock.release_audit.catalog_release.tag
            || catalog.release().tag_commit() != source_lock.release_audit.catalog_release.commit
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
        &source_lock.release_audit.catalog_release.tag,
        &source_lock.release_audit.catalog_release.commit,
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
            requires_reaudit,
        },
        feature_ids,
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
