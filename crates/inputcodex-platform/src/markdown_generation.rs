use std::{
    collections::HashSet,
    ffi::OsStr,
    fs::{self, File, Metadata},
    io::{self, BufRead, BufReader, Read},
    path::{Component, Path, PathBuf},
    sync::{
        Arc,
        atomic::{AtomicU8, Ordering},
    },
    time::{Duration, Instant},
};

use inputcodex_application::{
    ApplicationError, MarkdownGenerationCancellation, MarkdownGenerationPort,
    MarkdownGenerationRequest, PlatformPathsPort, PlatformPathsRequest,
};
use inputcodex_domain::{
    LocalSessionTitle, MAX_MARKDOWN_MESSAGE_COUNT, MarkdownGenerationError, MarkdownMessage,
    MarkdownMessageRole, MarkdownUtcTimestamp, SessionMarkdownDocument,
};
use rusqlite::{Connection, ErrorCode, OpenFlags, OptionalExtension, types::ValueRef};
use serde_json::Value;

use crate::{
    SystemPlatformPaths,
    local_session_directory_observation::{
        MAX_LOCAL_SESSION_DATABASES, SystemLocalSessionDirectoryFileProbe,
        resolve_local_session_sqlite_root_with_probe,
    },
};

const SQLITE_HOME_ENV: &str = "CODEX_SQLITE_HOME";
const INVALID_SQLITE_HOME_CODE: &str = "MARKDOWN_GENERATION_INVALID_SQLITE_HOME";
const TOO_MANY_DATABASES_CODE: &str = "MARKDOWN_GENERATION_TOO_MANY_DATABASES";
const UNSUPPORTED_SCHEMA_CODE: &str = "MARKDOWN_GENERATION_UNSUPPORTED_SCHEMA";
const UNAVAILABLE_CODE: &str = "MARKDOWN_GENERATION_UNAVAILABLE";
const INVALID_ROLLOUT_CODE: &str = "MARKDOWN_GENERATION_INVALID_ROLLOUT";
const INVALID_CONTENT_CODE: &str = "MARKDOWN_GENERATION_INVALID_CONTENT";
const RESOURCE_LIMIT_CODE: &str = "MARKDOWN_GENERATION_RESOURCE_LIMIT";
const MARKDOWN_GENERATION_TIMEOUT: &str = "MARKDOWN_GENERATION_TIMEOUT";
const MARKDOWN_GENERATION_CANCELLED: &str = "MARKDOWN_GENERATION_CANCELLED";
const SQLITE_DIRECTORY: &str = "sqlite";
const LEGACY_DATABASE: &str = "state_5.sqlite";
const SESSIONS_DIRECTORY: &str = "sessions";
const ARCHIVED_SESSIONS_DIRECTORY: &str = "archived_sessions";
const IMAGE_ATTACHMENT_OMITTED: &str = "> Image attachment omitted";
const INTERRUPTION_NONE: u8 = 0;
const INTERRUPTION_CANCELLED: u8 = 1;
const INTERRUPTION_TIMEOUT: u8 = 2;

pub(super) const MAX_ROLLOUT_DEPTH: usize = 4;
pub(super) const MAX_ROLLOUT_DISCOVERY_ENTRIES: usize = 8192;
pub(super) const MAX_ROLLOUT_CANDIDATES: usize = 4096;
pub(super) const MAX_ROLLOUT_METADATA_BYTES: usize = 8 * 1024;
pub(super) const MAX_ROLLOUT_DISCOVERY_BYTES: usize = 32 * 1024 * 1024;
pub(super) const MAX_ROLLOUT_BYTES: usize = 16 * 1024 * 1024;
pub(super) const MAX_ROLLOUT_RECORDS: usize = 100_000;
pub(super) const DEFAULT_BUSY_TIMEOUT: Duration = Duration::from_millis(50);
pub(super) const DEFAULT_PROGRESS_STEPS: i32 = 1_000;
pub(super) const DEFAULT_GENERATION_TIMEOUT: Duration = Duration::from_secs(2);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum MarkdownGenerationPathKind {
    Missing,
    File,
    Directory,
    Symlink,
    Other,
}

pub(super) trait MarkdownGenerationFileProbe {
    fn kind(&self, path: &Path) -> io::Result<MarkdownGenerationPathKind>;
    fn direct_entries(&self, path: &Path, max_entries: usize) -> io::Result<Vec<PathBuf>>;
}

#[derive(Debug, Clone, Copy, Default)]
struct SystemMarkdownGenerationFileProbe;

impl MarkdownGenerationFileProbe for SystemMarkdownGenerationFileProbe {
    fn kind(&self, path: &Path) -> io::Result<MarkdownGenerationPathKind> {
        match fs::symlink_metadata(path) {
            Ok(metadata) => {
                let file_type = metadata.file_type();
                if file_type.is_symlink() {
                    Ok(MarkdownGenerationPathKind::Symlink)
                } else if metadata.is_file() {
                    Ok(MarkdownGenerationPathKind::File)
                } else if metadata.is_dir() {
                    Ok(MarkdownGenerationPathKind::Directory)
                } else {
                    Ok(MarkdownGenerationPathKind::Other)
                }
            }
            Err(error) if error.kind() == io::ErrorKind::NotFound => {
                Ok(MarkdownGenerationPathKind::Missing)
            }
            Err(error) => Err(error),
        }
    }

    fn direct_entries(&self, path: &Path, max_entries: usize) -> io::Result<Vec<PathBuf>> {
        fs::read_dir(path)?
            .take(max_entries.saturating_add(1))
            .map(|entry| entry.map(|value| value.path()))
            .collect()
    }
}

#[derive(Debug, Clone, Copy)]
pub(super) struct MarkdownGenerationPolicy {
    generation_timeout: Duration,
    busy_timeout: Duration,
    progress_steps: i32,
    max_rollout_depth: usize,
    max_discovery_entries: usize,
    max_rollout_candidates: usize,
    max_metadata_bytes: usize,
    max_discovery_bytes: usize,
    max_rollout_bytes: usize,
    max_rollout_records: usize,
    max_messages: usize,
}

impl Default for MarkdownGenerationPolicy {
    fn default() -> Self {
        Self {
            generation_timeout: DEFAULT_GENERATION_TIMEOUT,
            busy_timeout: DEFAULT_BUSY_TIMEOUT,
            progress_steps: DEFAULT_PROGRESS_STEPS,
            max_rollout_depth: MAX_ROLLOUT_DEPTH,
            max_discovery_entries: MAX_ROLLOUT_DISCOVERY_ENTRIES,
            max_rollout_candidates: MAX_ROLLOUT_CANDIDATES,
            max_metadata_bytes: MAX_ROLLOUT_METADATA_BYTES,
            max_discovery_bytes: MAX_ROLLOUT_DISCOVERY_BYTES,
            max_rollout_bytes: MAX_ROLLOUT_BYTES,
            max_rollout_records: MAX_ROLLOUT_RECORDS,
            max_messages: MAX_MARKDOWN_MESSAGE_COUNT,
        }
    }
}

// 集成测试以源码模块方式调用这些缩小版策略构造器。
#[cfg(test)]
#[allow(dead_code)]
impl MarkdownGenerationPolicy {
    #[must_use]
    pub(super) fn new(
        generation_timeout: Duration,
        busy_timeout: Duration,
        progress_steps: i32,
    ) -> Self {
        Self {
            generation_timeout,
            busy_timeout,
            progress_steps,
            ..Self::default()
        }
    }

    #[must_use]
    pub(super) const fn with_discovery_limits(
        mut self,
        max_rollout_depth: usize,
        max_discovery_entries: usize,
        max_rollout_candidates: usize,
        max_metadata_bytes: usize,
        max_discovery_bytes: usize,
    ) -> Self {
        self.max_rollout_depth = max_rollout_depth;
        self.max_discovery_entries = max_discovery_entries;
        self.max_rollout_candidates = max_rollout_candidates;
        self.max_metadata_bytes = max_metadata_bytes;
        self.max_discovery_bytes = max_discovery_bytes;
        self
    }

    #[must_use]
    pub(super) const fn with_rollout_limits(
        mut self,
        max_rollout_bytes: usize,
        max_rollout_records: usize,
        max_messages: usize,
    ) -> Self {
        self.max_rollout_bytes = max_rollout_bytes;
        self.max_rollout_records = max_rollout_records;
        self.max_messages = max_messages;
        self
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct SystemMarkdownGeneration;

impl MarkdownGenerationPort for SystemMarkdownGeneration {
    fn generate(
        &self,
        request: &MarkdownGenerationRequest,
        cancellation: &MarkdownGenerationCancellation,
    ) -> Result<Option<SessionMarkdownDocument>, ApplicationError> {
        let paths = SystemPlatformPaths.resolve(&PlatformPathsRequest::default())?;
        let codex_home = paths.codex_home().as_path();
        let sqlite_root = resolve_local_session_sqlite_root_with_probe(
            codex_home,
            std::env::var_os(SQLITE_HOME_ENV),
            &SystemLocalSessionDirectoryFileProbe,
        )
        .map_err(map_local_session_discovery_error)?;

        generate_session_markdown_at_roots(
            codex_home,
            &sqlite_root,
            request,
            cancellation,
            MarkdownGenerationPolicy::default(),
        )
    }
}

pub(super) fn generate_session_markdown_at_roots(
    codex_home: &Path,
    sqlite_root: &Path,
    request: &MarkdownGenerationRequest,
    cancellation: &MarkdownGenerationCancellation,
    policy: MarkdownGenerationPolicy,
) -> Result<Option<SessionMarkdownDocument>, ApplicationError> {
    let deadline = Instant::now()
        .checked_add(policy.generation_timeout)
        .unwrap_or_else(Instant::now);
    let interruption = InterruptionControl::new(cancellation.clone(), deadline);
    interruption.ensure_active()?;

    let candidates = discover_markdown_databases(sqlite_root, policy, &interruption)?;
    if candidates.is_empty() {
        return Ok(None);
    }

    let mut records = Vec::new();
    for candidate in &candidates {
        interruption.ensure_active()?;
        if let Some(record) = read_session_record(candidate, request, policy, &interruption)? {
            records.push(record);
        }
    }
    if records.is_empty() {
        return Ok(None);
    }

    records.sort_by(|left, right| {
        right
            .updated_at_ms
            .cmp(&left.updated_at_ms)
            .then_with(|| left.priority.cmp(&right.priority))
    });
    let record = records.remove(0);
    let rollout_path = match record.rollout_path {
        Some(path) => {
            validate_rollout_path_with_probe(
                codex_home,
                &path,
                &SystemMarkdownGenerationFileProbe,
            )?;
            path
        }
        None => discover_rollout_path(codex_home, request.session_id(), policy, &interruption)?
            .ok_or_else(invalid_rollout)?,
    };

    let messages = load_rollout_messages(
        codex_home,
        &rollout_path,
        request.session_id(),
        policy,
        &interruption,
    )?;
    if messages.is_empty() {
        return Ok(None);
    }

    interruption.ensure_active()?;
    let document = SessionMarkdownDocument::generate(record.title.as_ref(), messages)
        .map_err(map_domain_error)?;
    interruption.ensure_active()?;
    Ok(Some(document))
}

#[derive(Debug)]
struct SessionRecord {
    title: Option<LocalSessionTitle>,
    rollout_path: Option<PathBuf>,
    updated_at_ms: Option<i64>,
    priority: usize,
}

#[derive(Debug)]
struct MarkdownDatabaseCandidate {
    path: PathBuf,
    priority: usize,
}

impl MarkdownDatabaseCandidate {
    fn path(&self) -> &Path {
        &self.path
    }

    const fn priority(&self) -> usize {
        self.priority
    }
}

fn discover_markdown_databases(
    root: &Path,
    policy: MarkdownGenerationPolicy,
    interruption: &InterruptionControl,
) -> Result<Vec<MarkdownDatabaseCandidate>, ApplicationError> {
    let probe = SystemMarkdownGenerationFileProbe;
    let sqlite_directory = root.join(SQLITE_DIRECTORY);
    let mut current = Vec::new();

    match probe.kind(&sqlite_directory).map_err(|_| unavailable())? {
        MarkdownGenerationPathKind::Missing => {}
        MarkdownGenerationPathKind::Directory => {
            let entries = fs::read_dir(&sqlite_directory).map_err(|_| unavailable())?;
            for (index, entry) in entries.enumerate() {
                interruption.ensure_active()?;
                if index >= policy.max_discovery_entries {
                    return Err(resource_limit());
                }
                let path = entry.map_err(|_| unavailable())?.path();
                if is_sqlite_candidate(&path)
                    && probe.kind(&path).map_err(|_| unavailable())?
                        == MarkdownGenerationPathKind::File
                {
                    current.push(path);
                    if current.len() > MAX_LOCAL_SESSION_DATABASES {
                        return Err(ApplicationError::unavailable(TOO_MANY_DATABASES_CODE));
                    }
                }
            }
        }
        MarkdownGenerationPathKind::File
        | MarkdownGenerationPathKind::Symlink
        | MarkdownGenerationPathKind::Other => return Err(unavailable()),
    }

    current.sort_by(|left, right| left.file_name().cmp(&right.file_name()));
    let legacy = root.join(LEGACY_DATABASE);
    if probe.kind(&legacy).map_err(|_| unavailable())? == MarkdownGenerationPathKind::File
        && !current.iter().any(|candidate| candidate == &legacy)
    {
        current.push(legacy);
    }
    if current.len() > MAX_LOCAL_SESSION_DATABASES {
        return Err(ApplicationError::unavailable(TOO_MANY_DATABASES_CODE));
    }

    Ok(current
        .into_iter()
        .enumerate()
        .map(|(priority, path)| MarkdownDatabaseCandidate { path, priority })
        .collect())
}

fn read_session_record(
    candidate: &MarkdownDatabaseCandidate,
    request: &MarkdownGenerationRequest,
    policy: MarkdownGenerationPolicy,
    interruption: &InterruptionControl,
) -> Result<Option<SessionRecord>, ApplicationError> {
    let before_open = fs::symlink_metadata(candidate.path()).map_err(|_| unavailable())?;
    if !before_open.is_file() {
        return Err(unavailable());
    }
    let flags = OpenFlags::SQLITE_OPEN_READ_ONLY
        | OpenFlags::SQLITE_OPEN_NO_MUTEX
        | OpenFlags::SQLITE_OPEN_NOFOLLOW;
    let connection = Connection::open_with_flags(candidate.path(), flags)
        .map_err(|error| map_sqlite_error(&error, interruption, unavailable()))?;
    let after_open = fs::symlink_metadata(candidate.path()).map_err(|_| unavailable())?;
    if !after_open.is_file() || !same_file_identity(&before_open, &after_open) {
        return Err(unavailable());
    }
    connection
        .busy_timeout(policy.busy_timeout)
        .map_err(|error| map_sqlite_error(&error, interruption, unavailable()))?;

    let progress_control = interruption.clone();
    connection
        .progress_handler(
            policy.progress_steps.max(1),
            Some(move || progress_control.should_interrupt()),
        )
        .map_err(|error| map_sqlite_error(&error, interruption, unavailable()))?;
    connection
        .pragma_update(None, "query_only", true)
        .map_err(|error| map_sqlite_error(&error, interruption, unavailable()))?;
    let query_only = connection
        .pragma_query_value(None, "query_only", |row| row.get::<_, i64>(0))
        .map_err(|error| map_sqlite_error(&error, interruption, unavailable()))?;
    if query_only != 1 {
        return Err(unavailable());
    }

    let mut supported = false;
    if has_table(&connection, "threads", interruption)? {
        let columns = table_columns(&connection, SupportedTable::Threads, interruption)?;
        if columns.contains("id") {
            supported = true;
            if let Some(record) = query_threads(
                &connection,
                &columns,
                request.session_id(),
                candidate.priority(),
                policy.max_metadata_bytes,
                interruption,
            )? {
                return Ok(Some(record));
            }
        }
    }

    if has_table(&connection, "automation_runs", interruption)? {
        let columns = table_columns(&connection, SupportedTable::AutomationRuns, interruption)?;
        if columns.contains("thread_id") {
            supported = true;
            if let Some(record) = query_automation_runs(
                &connection,
                &columns,
                request.session_id(),
                candidate.priority(),
                policy.max_metadata_bytes,
                interruption,
            )? {
                return Ok(Some(record));
            }
        }
    }

    if supported {
        Ok(None)
    } else {
        Err(unsupported_schema())
    }
}

fn has_table(
    connection: &Connection,
    table: &'static str,
    interruption: &InterruptionControl,
) -> Result<bool, ApplicationError> {
    connection
        .query_row(
            "SELECT 1 FROM sqlite_master WHERE type = 'table' AND name = ?1 LIMIT 1",
            [table],
            |_| Ok(()),
        )
        .optional()
        .map(|value| value.is_some())
        .map_err(|error| map_sqlite_error(&error, interruption, unavailable()))
}

#[derive(Debug, Clone, Copy)]
enum SupportedTable {
    Threads,
    AutomationRuns,
}

fn table_columns(
    connection: &Connection,
    table: SupportedTable,
    interruption: &InterruptionControl,
) -> Result<HashSet<String>, ApplicationError> {
    let sql = match table {
        SupportedTable::Threads => "PRAGMA table_info('threads')",
        SupportedTable::AutomationRuns => "PRAGMA table_info('automation_runs')",
    };
    let mut statement = connection
        .prepare(sql)
        .map_err(|error| map_sqlite_error(&error, interruption, unsupported_schema()))?;
    let rows = statement
        .query_map([], |row| row.get::<_, String>(1))
        .map_err(|error| map_sqlite_error(&error, interruption, unsupported_schema()))?;
    let mut columns = HashSet::new();
    for row in rows {
        columns.insert(
            row.map_err(|error| map_sqlite_error(&error, interruption, unsupported_schema()))?,
        );
    }
    Ok(columns)
}

fn query_threads(
    connection: &Connection,
    columns: &HashSet<String>,
    session_id: &str,
    priority: usize,
    max_text_bytes: usize,
    interruption: &InterruptionControl,
) -> Result<Option<SessionRecord>, ApplicationError> {
    let title = optional_expression(columns, "title", "title", "NULL");
    let rollout_path = optional_expression(columns, "rollout_path", "rollout_path", "NULL");
    let updated_at = if columns.contains("updated_at_ms") {
        "updated_at_ms"
    } else if columns.contains("updated_at") {
        "updated_at * 1000"
    } else if columns.contains("created_at_ms") {
        "created_at_ms"
    } else if columns.contains("created_at") {
        "created_at * 1000"
    } else {
        "NULL"
    };
    let sql = format!(
        "SELECT {title} AS display_title, {rollout_path} AS rollout_path, \
         {updated_at} AS updated_at_ms FROM threads WHERE id = ?1 LIMIT 1"
    );
    query_record(
        connection,
        &sql,
        session_id,
        priority,
        max_text_bytes,
        interruption,
    )
}

fn query_automation_runs(
    connection: &Connection,
    columns: &HashSet<String>,
    session_id: &str,
    priority: usize,
    max_text_bytes: usize,
    interruption: &InterruptionControl,
) -> Result<Option<SessionRecord>, ApplicationError> {
    let title = if columns.contains("thread_title") {
        "thread_title"
    } else if columns.contains("title") {
        "title"
    } else {
        "NULL"
    };
    let rollout_path = optional_expression(columns, "rollout_path", "rollout_path", "NULL");
    let updated_at = if columns.contains("updated_at") && columns.contains("created_at") {
        "COALESCE(updated_at, created_at)"
    } else if columns.contains("updated_at") {
        "updated_at"
    } else if columns.contains("created_at") {
        "created_at"
    } else {
        "NULL"
    };
    let sql = format!(
        "SELECT {title} AS display_title, {rollout_path} AS rollout_path, \
         {updated_at} AS updated_at_ms FROM automation_runs WHERE thread_id = ?1 LIMIT 1"
    );
    query_record(
        connection,
        &sql,
        session_id,
        priority,
        max_text_bytes,
        interruption,
    )
}

enum QueriedSessionRecord {
    Record(SessionRecord),
    ResourceLimit,
    InvalidContent,
}

enum BoundedOptionalText {
    Value(Option<String>),
    ResourceLimit,
    InvalidContent,
}

fn query_record(
    connection: &Connection,
    sql: &str,
    session_id: &str,
    priority: usize,
    max_text_bytes: usize,
    interruption: &InterruptionControl,
) -> Result<Option<SessionRecord>, ApplicationError> {
    let result = connection
        .query_row(sql, [session_id], |row| {
            let raw_title = match read_bounded_optional_text(row.get_ref(0)?, max_text_bytes) {
                BoundedOptionalText::Value(value) => value,
                BoundedOptionalText::ResourceLimit => {
                    return Ok(QueriedSessionRecord::ResourceLimit);
                }
                BoundedOptionalText::InvalidContent => {
                    return Ok(QueriedSessionRecord::InvalidContent);
                }
            };
            let raw_rollout_path = match read_bounded_optional_text(row.get_ref(1)?, max_text_bytes)
            {
                BoundedOptionalText::Value(value) => value,
                BoundedOptionalText::ResourceLimit => {
                    return Ok(QueriedSessionRecord::ResourceLimit);
                }
                BoundedOptionalText::InvalidContent => {
                    return Ok(QueriedSessionRecord::InvalidContent);
                }
            };
            let updated_at_ms = row.get::<_, Option<i64>>(2)?;
            Ok(QueriedSessionRecord::Record(SessionRecord {
                title: raw_title.as_deref().and_then(LocalSessionTitle::from_raw),
                rollout_path: raw_rollout_path
                    .filter(|value| !value.trim().is_empty())
                    .map(PathBuf::from),
                updated_at_ms,
                priority,
            }))
        })
        .optional()
        .map_err(|error| map_sqlite_error(&error, interruption, unsupported_schema()))?;

    match result {
        None => Ok(None),
        Some(QueriedSessionRecord::Record(record)) => Ok(Some(record)),
        Some(QueriedSessionRecord::ResourceLimit) => Err(resource_limit()),
        Some(QueriedSessionRecord::InvalidContent) => Err(invalid_content()),
    }
}

fn read_bounded_optional_text(value: ValueRef<'_>, max_bytes: usize) -> BoundedOptionalText {
    match value {
        ValueRef::Null => BoundedOptionalText::Value(None),
        ValueRef::Text(bytes) if bytes.len() > max_bytes => BoundedOptionalText::ResourceLimit,
        ValueRef::Text(bytes) => match std::str::from_utf8(bytes) {
            Ok(value) => BoundedOptionalText::Value(Some(value.to_owned())),
            Err(_) => BoundedOptionalText::InvalidContent,
        },
        ValueRef::Integer(_) | ValueRef::Real(_) | ValueRef::Blob(_) => {
            BoundedOptionalText::InvalidContent
        }
    }
}

fn optional_expression<'a>(
    columns: &HashSet<String>,
    column: &str,
    present: &'a str,
    absent: &'a str,
) -> &'a str {
    if columns.contains(column) {
        present
    } else {
        absent
    }
}

pub(super) fn validate_rollout_path_with_probe(
    codex_home: &Path,
    rollout_path: &Path,
    probe: &impl MarkdownGenerationFileProbe,
) -> Result<(), ApplicationError> {
    validate_codex_home_with_probe(codex_home, probe)?;
    if !rollout_path.is_absolute()
        || rollout_path
            .components()
            .any(|component| matches!(component, Component::CurDir | Component::ParentDir))
        || !is_jsonl_path(rollout_path)
    {
        return Err(invalid_rollout());
    }

    let roots = [
        codex_home.join(SESSIONS_DIRECTORY),
        codex_home.join(ARCHIVED_SESSIONS_DIRECTORY),
    ];
    let root = roots
        .iter()
        .find(|root| rollout_path.starts_with(root))
        .ok_or_else(invalid_rollout)?;
    if probe.kind(root).map_err(|_| invalid_rollout())? != MarkdownGenerationPathKind::Directory {
        return Err(invalid_rollout());
    }

    let relative = rollout_path
        .strip_prefix(root)
        .map_err(|_| invalid_rollout())?;
    let components = relative.components().collect::<Vec<_>>();
    if components.is_empty() || components.len().saturating_sub(1) > MAX_ROLLOUT_DEPTH {
        return Err(invalid_rollout());
    }
    let mut current = root.to_path_buf();
    for (index, component) in components.iter().enumerate() {
        let Component::Normal(name) = component else {
            return Err(invalid_rollout());
        };
        current.push(name);
        let expected = if index + 1 == components.len() {
            MarkdownGenerationPathKind::File
        } else {
            MarkdownGenerationPathKind::Directory
        };
        if probe.kind(&current).map_err(|_| invalid_rollout())? != expected {
            return Err(invalid_rollout());
        }
    }

    Ok(())
}

fn validate_codex_home_with_probe(
    codex_home: &Path,
    probe: &impl MarkdownGenerationFileProbe,
) -> Result<(), ApplicationError> {
    if !codex_home.is_absolute()
        || probe.kind(codex_home).map_err(|_| invalid_rollout())?
            != MarkdownGenerationPathKind::Directory
    {
        return Err(invalid_rollout());
    }
    Ok(())
}

fn discover_rollout_path(
    codex_home: &Path,
    session_id: &str,
    policy: MarkdownGenerationPolicy,
    interruption: &InterruptionControl,
) -> Result<Option<PathBuf>, ApplicationError> {
    let probe = SystemMarkdownGenerationFileProbe;
    validate_codex_home_with_probe(codex_home, &probe)?;
    let mut candidates = Vec::new();
    let mut enumerated = 0;
    for root in [
        codex_home.join(SESSIONS_DIRECTORY),
        codex_home.join(ARCHIVED_SESSIONS_DIRECTORY),
    ] {
        interruption.ensure_active()?;
        match probe.kind(&root).map_err(|_| unavailable())? {
            MarkdownGenerationPathKind::Missing => continue,
            MarkdownGenerationPathKind::Directory => collect_rollout_candidates(
                &root,
                0,
                &probe,
                policy,
                interruption,
                &mut enumerated,
                &mut candidates,
            )?,
            MarkdownGenerationPathKind::File
            | MarkdownGenerationPathKind::Symlink
            | MarkdownGenerationPathKind::Other => return Err(invalid_rollout()),
        }
    }

    let mut discovery_bytes = 0;
    let mut matched = None;
    for candidate in candidates {
        interruption.ensure_active()?;
        if rollout_matches_session(
            codex_home,
            &candidate,
            session_id,
            policy,
            interruption,
            &mut discovery_bytes,
        )? {
            if matched.is_some() {
                return Err(invalid_rollout());
            }
            matched = Some(candidate);
        }
    }
    Ok(matched)
}

#[allow(clippy::too_many_arguments)]
fn collect_rollout_candidates(
    directory: &Path,
    depth: usize,
    probe: &impl MarkdownGenerationFileProbe,
    policy: MarkdownGenerationPolicy,
    interruption: &InterruptionControl,
    enumerated: &mut usize,
    candidates: &mut Vec<PathBuf>,
) -> Result<(), ApplicationError> {
    interruption.ensure_active()?;
    let remaining_entries = policy.max_discovery_entries.saturating_sub(*enumerated);
    let mut entries = probe
        .direct_entries(directory, remaining_entries)
        .map_err(|_| unavailable())?;
    if entries.len() > remaining_entries {
        return Err(resource_limit());
    }
    entries.sort();
    for path in entries {
        interruption.ensure_active()?;
        *enumerated = enumerated.saturating_add(1);
        if *enumerated > policy.max_discovery_entries {
            return Err(resource_limit());
        }

        match probe.kind(&path).map_err(|_| unavailable())? {
            MarkdownGenerationPathKind::Directory => {
                if depth >= policy.max_rollout_depth {
                    return Err(resource_limit());
                }
                collect_rollout_candidates(
                    &path,
                    depth + 1,
                    probe,
                    policy,
                    interruption,
                    enumerated,
                    candidates,
                )?;
            }
            MarkdownGenerationPathKind::File if is_jsonl_path(&path) => {
                candidates.push(path);
                if candidates.len() > policy.max_rollout_candidates {
                    return Err(resource_limit());
                }
            }
            MarkdownGenerationPathKind::File => {}
            MarkdownGenerationPathKind::Missing => return Err(unavailable()),
            MarkdownGenerationPathKind::Symlink | MarkdownGenerationPathKind::Other => {
                return Err(invalid_rollout());
            }
        }
    }
    Ok(())
}

fn open_verified_rollout_file(
    codex_home: &Path,
    path: &Path,
    policy: MarkdownGenerationPolicy,
    interruption: &InterruptionControl,
) -> Result<File, ApplicationError> {
    interruption.ensure_active()?;
    validate_rollout_path_with_probe(codex_home, path, &SystemMarkdownGenerationFileProbe)?;
    let before_open = fs::symlink_metadata(path).map_err(|_| invalid_rollout())?;
    if !before_open.is_file() {
        return Err(invalid_rollout());
    }
    if before_open.len() > u64::try_from(policy.max_rollout_bytes).unwrap_or(u64::MAX) {
        return Err(resource_limit());
    }

    let file = open_file_no_follow(path).map_err(|_| unavailable())?;
    let opened = file.metadata().map_err(|_| unavailable())?;
    interruption.ensure_active()?;
    validate_rollout_path_with_probe(codex_home, path, &SystemMarkdownGenerationFileProbe)?;
    let after_open = fs::symlink_metadata(path).map_err(|_| invalid_rollout())?;
    if !opened.is_file() || !after_open.is_file() {
        return Err(invalid_rollout());
    }
    if opened.len() > u64::try_from(policy.max_rollout_bytes).unwrap_or(u64::MAX)
        || after_open.len() > u64::try_from(policy.max_rollout_bytes).unwrap_or(u64::MAX)
    {
        return Err(resource_limit());
    }
    if !same_file_identity(&before_open, &opened) || !same_file_identity(&opened, &after_open) {
        return Err(invalid_rollout());
    }

    Ok(file)
}

#[cfg(target_os = "windows")]
fn open_file_no_follow(path: &Path) -> io::Result<File> {
    use std::os::windows::fs::*;

    const FILE_FLAG_OPEN_REPARSE_POINT: u32 = 0x0020_0000;
    File::options()
        .read(true)
        .custom_flags(FILE_FLAG_OPEN_REPARSE_POINT)
        .open(path)
}

#[cfg(target_os = "macos")]
fn open_file_no_follow(path: &Path) -> io::Result<File> {
    use std::os::unix::fs::*;

    const O_NOFOLLOW: i32 = 0x0000_0100;
    File::options()
        .read(true)
        .custom_flags(O_NOFOLLOW)
        .open(path)
}

#[cfg(target_os = "windows")]
pub(super) fn same_file_identity(left: &Metadata, right: &Metadata) -> bool {
    use std::os::windows::fs::MetadataExt;

    // Windows file IDs remain unstable in std; no-follow open and component revalidation
    // are paired with all stable metadata fields that can reveal a path replacement.
    left.file_attributes() == right.file_attributes()
        && left.creation_time() == right.creation_time()
        && left.last_write_time() == right.last_write_time()
        && left.file_size() == right.file_size()
}

#[cfg(target_os = "macos")]
pub(super) fn same_file_identity(left: &Metadata, right: &Metadata) -> bool {
    use std::os::unix::fs::MetadataExt;

    left.dev() == right.dev() && left.ino() == right.ino()
}

fn rollout_matches_session(
    codex_home: &Path,
    path: &Path,
    session_id: &str,
    policy: MarkdownGenerationPolicy,
    interruption: &InterruptionControl,
    discovery_bytes: &mut usize,
) -> Result<bool, ApplicationError> {
    let file = open_verified_rollout_file(codex_home, path, policy, interruption)?;
    let take_limit = u64::try_from(policy.max_metadata_bytes)
        .unwrap_or(u64::MAX)
        .saturating_add(1);
    let mut reader = BufReader::new(file.take(take_limit));
    let mut candidate_bytes = 0_usize;
    let mut line = String::new();

    loop {
        interruption.ensure_active()?;
        line.clear();
        let read = reader.read_line(&mut line).map_err(map_line_read_error)?;
        if read == 0 {
            return Ok(false);
        }
        candidate_bytes = candidate_bytes.saturating_add(read);
        *discovery_bytes = discovery_bytes.saturating_add(read);
        if candidate_bytes > policy.max_metadata_bytes
            || *discovery_bytes > policy.max_discovery_bytes
        {
            return Err(resource_limit());
        }

        let raw = line.trim();
        if raw.is_empty() {
            continue;
        }
        let event: Value = serde_json::from_str(raw).map_err(|_| invalid_content())?;
        let Some(event_type) = event.get("type").and_then(Value::as_str) else {
            return Err(invalid_content());
        };
        if event_type != "session_meta" {
            continue;
        }
        let id = event
            .get("payload")
            .and_then(|payload| payload.get("id"))
            .or_else(|| event.get("id"))
            .and_then(Value::as_str)
            .ok_or_else(invalid_content)?;
        return Ok(normalize_session_id(id) == normalize_session_id(session_id));
    }
}

fn load_rollout_messages(
    codex_home: &Path,
    path: &Path,
    session_id: &str,
    policy: MarkdownGenerationPolicy,
    interruption: &InterruptionControl,
) -> Result<Vec<MarkdownMessage>, ApplicationError> {
    let file = open_verified_rollout_file(codex_home, path, policy, interruption)?;
    let take_limit = u64::try_from(policy.max_rollout_bytes)
        .unwrap_or(u64::MAX)
        .saturating_add(1);
    let mut reader = BufReader::new(file.take(take_limit));
    let mut line = String::new();
    let mut bytes_read = 0_usize;
    let mut record_count = 0;
    let mut messages = Vec::new();

    loop {
        interruption.ensure_active()?;
        line.clear();
        let read = reader.read_line(&mut line).map_err(map_line_read_error)?;
        if read == 0 {
            break;
        }
        bytes_read = bytes_read.saturating_add(read);
        if bytes_read > policy.max_rollout_bytes {
            return Err(resource_limit());
        }
        let raw = line.trim();
        if raw.is_empty() {
            continue;
        }
        record_count += 1;
        if record_count > policy.max_rollout_records {
            return Err(resource_limit());
        }

        let event: Value = serde_json::from_str(raw).map_err(|_| invalid_content())?;
        let event_type = event
            .get("type")
            .and_then(Value::as_str)
            .ok_or_else(invalid_content)?;
        if event_type == "session_meta" {
            let id = event
                .get("payload")
                .and_then(|payload| payload.get("id"))
                .or_else(|| event.get("id"));
            if let Some(id) = id {
                let id = id.as_str().ok_or_else(invalid_content)?;
                if normalize_session_id(id) != normalize_session_id(session_id) {
                    return Err(invalid_content());
                }
            }
            continue;
        }
        if event_type != "response_item" {
            continue;
        }
        let payload = event
            .get("payload")
            .and_then(Value::as_object)
            .ok_or_else(invalid_content)?;
        let payload_type = payload
            .get("type")
            .and_then(Value::as_str)
            .ok_or_else(invalid_content)?;
        if payload_type != "message" {
            continue;
        }
        let role = match payload.get("role").and_then(Value::as_str) {
            Some("user") => MarkdownMessageRole::User,
            Some("assistant") => MarkdownMessageRole::Assistant,
            Some(_) => continue,
            None => return Err(invalid_content()),
        };
        let content = payload
            .get("content")
            .and_then(Value::as_array)
            .ok_or_else(invalid_content)?;
        let body = serialize_message_content(content)?;
        if body.trim().is_empty() {
            continue;
        }
        if messages.len() >= policy.max_messages {
            return Err(resource_limit());
        }
        let timestamp = event
            .get("timestamp")
            .and_then(Value::as_str)
            .and_then(normalize_rfc3339_utc)
            .and_then(|value| MarkdownUtcTimestamp::new(value).ok());
        let message = MarkdownMessage::new(role, timestamp, body).map_err(map_domain_error)?;
        messages.push(message);
    }

    Ok(messages)
}

fn serialize_message_content(content: &[Value]) -> Result<String, ApplicationError> {
    let mut blocks = Vec::new();
    for block in content {
        let Some(block_type) = block.get("type").and_then(Value::as_str) else {
            return Err(invalid_content());
        };
        match block_type {
            "input_text" | "output_text" => {
                let text = block
                    .get("text")
                    .and_then(Value::as_str)
                    .ok_or_else(invalid_content)?;
                let normalized = normalize_newlines(text).trim_matches('\n').to_owned();
                if !normalized.trim().is_empty() {
                    blocks.push(normalized);
                }
            }
            "input_image" | "output_image" => {
                blocks.push(IMAGE_ATTACHMENT_OMITTED.to_owned());
            }
            _ => {}
        }
    }
    Ok(blocks.join("\n\n"))
}

pub(super) fn normalize_rfc3339_utc(value: &str) -> Option<String> {
    let bytes = value.as_bytes();
    if !value.is_ascii() || bytes.len() < 20 {
        return None;
    }
    if bytes.get(4) != Some(&b'-')
        || bytes.get(7) != Some(&b'-')
        || bytes.get(10) != Some(&b'T')
        || bytes.get(13) != Some(&b':')
        || bytes.get(16) != Some(&b':')
    {
        return None;
    }
    for index in [0, 1, 2, 3, 5, 6, 8, 9, 11, 12, 14, 15, 17, 18] {
        if !bytes.get(index).is_some_and(u8::is_ascii_digit) {
            return None;
        }
    }

    let (timestamp_end, offset_minutes) = if bytes.last() == Some(&b'Z') {
        (bytes.len() - 1, 0_i32)
    } else {
        if bytes.len() < 25
            || bytes.get(bytes.len() - 3) != Some(&b':')
            || !matches!(bytes.get(bytes.len() - 6), Some(b'+') | Some(b'-'))
        {
            return None;
        }
        let sign_index = bytes.len() - 6;
        let offset_hour = parse_decimal(bytes.get(sign_index + 1..sign_index + 3)?)?;
        let offset_minute = parse_decimal(bytes.get(sign_index + 4..sign_index + 6)?)?;
        if offset_hour > 23 || offset_minute > 59 {
            return None;
        }
        let magnitude = i32::try_from(offset_hour * 60 + offset_minute).ok()?;
        if bytes[sign_index] == b'-' && magnitude == 0 {
            return None;
        }
        let signed = if bytes[sign_index] == b'+' {
            magnitude
        } else {
            -magnitude
        };
        (sign_index, signed)
    };

    if timestamp_end < 19 {
        return None;
    }
    let fraction = match timestamp_end {
        19 => "",
        end if bytes.get(19) == Some(&b'.') && (21..=29).contains(&end) => {
            let digits = bytes.get(20..end)?;
            if digits.is_empty() || !digits.iter().all(u8::is_ascii_digit) {
                return None;
            }
            value.get(20..end)?.trim_end_matches('0')
        }
        _ => return None,
    };

    let mut year = parse_decimal(bytes.get(0..4)?)?;
    let mut month = parse_decimal(bytes.get(5..7)?)?;
    let mut day = parse_decimal(bytes.get(8..10)?)?;
    let hour = parse_decimal(bytes.get(11..13)?)?;
    let minute = parse_decimal(bytes.get(14..16)?)?;
    let second = parse_decimal(bytes.get(17..19)?)?;
    let max_day = days_in_month(year, month);
    if year == 0
        || max_day == 0
        || !(1..=max_day).contains(&day)
        || hour > 23
        || minute > 59
        || second > 60
    {
        return None;
    }

    let local_minutes = i32::try_from(hour * 60 + minute).ok()?;
    let utc_minutes = local_minutes - offset_minutes;
    let day_delta = utc_minutes.div_euclid(24 * 60);
    let minute_of_day = utc_minutes.rem_euclid(24 * 60);
    adjust_date(&mut year, &mut month, &mut day, day_delta)?;
    let utc_hour = u32::try_from(minute_of_day / 60).ok()?;
    let utc_minute = u32::try_from(minute_of_day % 60).ok()?;
    if !is_supported_utc_second(year, month, day, utc_hour, utc_minute, second) {
        return None;
    }

    if fraction.is_empty() {
        Some(format!(
            "{year:04}-{month:02}-{day:02}T{utc_hour:02}:{utc_minute:02}:{second:02}Z"
        ))
    } else {
        Some(format!(
            "{year:04}-{month:02}-{day:02}T{utc_hour:02}:{utc_minute:02}:{second:02}.{fraction}Z"
        ))
    }
}

const fn is_supported_utc_second(
    year: u32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
    second: u32,
) -> bool {
    second <= 59
        || (second == 60
            && hour == 23
            && minute == 59
            && matches!(month, 6 | 12)
            && day == days_in_month(year, month))
}

fn adjust_date(year: &mut u32, month: &mut u32, day: &mut u32, delta: i32) -> Option<()> {
    match delta {
        -1 => {
            if *day > 1 {
                *day -= 1;
            } else if *month > 1 {
                *month -= 1;
                *day = days_in_month(*year, *month);
            } else {
                if *year <= 1 {
                    return None;
                }
                *year -= 1;
                *month = 12;
                *day = 31;
            }
        }
        0 => {}
        1 => {
            if *day < days_in_month(*year, *month) {
                *day += 1;
            } else if *month < 12 {
                *month += 1;
                *day = 1;
            } else {
                if *year >= 9999 {
                    return None;
                }
                *year += 1;
                *month = 1;
                *day = 1;
            }
        }
        _ => return None,
    }
    Some(())
}

fn parse_decimal(bytes: &[u8]) -> Option<u32> {
    if bytes.is_empty() || !bytes.iter().all(u8::is_ascii_digit) {
        return None;
    }
    Some(
        bytes
            .iter()
            .fold(0, |value, digit| value * 10 + u32::from(digit - b'0')),
    )
}

const fn days_in_month(year: u32, month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if is_leap_year(year) => 29,
        2 => 28,
        _ => 0,
    }
}

const fn is_leap_year(year: u32) -> bool {
    year.is_multiple_of(4) && (!year.is_multiple_of(100) || year.is_multiple_of(400))
}

fn is_jsonl_path(path: &Path) -> bool {
    path.extension()
        .and_then(OsStr::to_str)
        .is_some_and(|extension| extension.eq_ignore_ascii_case("jsonl"))
}

fn is_sqlite_candidate(path: &Path) -> bool {
    path.extension()
        .and_then(OsStr::to_str)
        .is_some_and(|extension| {
            extension.eq_ignore_ascii_case("db")
                || extension.eq_ignore_ascii_case("sqlite")
                || extension.eq_ignore_ascii_case("sqlite3")
        })
}

fn normalize_session_id(value: &str) -> &str {
    value.strip_prefix("local:").unwrap_or(value)
}

fn normalize_newlines(value: &str) -> String {
    value.replace("\r\n", "\n").replace('\r', "\n")
}

fn map_line_read_error(error: io::Error) -> ApplicationError {
    if error.kind() == io::ErrorKind::InvalidData {
        invalid_content()
    } else {
        unavailable()
    }
}

#[derive(Clone)]
struct InterruptionControl {
    cancellation: MarkdownGenerationCancellation,
    deadline: Instant,
    reason: Arc<AtomicU8>,
}

impl InterruptionControl {
    fn new(cancellation: MarkdownGenerationCancellation, deadline: Instant) -> Self {
        Self {
            cancellation,
            deadline,
            reason: Arc::new(AtomicU8::new(INTERRUPTION_NONE)),
        }
    }

    fn ensure_active(&self) -> Result<(), ApplicationError> {
        match self.current_error() {
            Some(error) => Err(error),
            None => Ok(()),
        }
    }

    fn should_interrupt(&self) -> bool {
        if self.cancellation.is_cancelled() {
            self.reason.store(INTERRUPTION_CANCELLED, Ordering::Release);
        } else if Instant::now() >= self.deadline {
            let _ = self.reason.compare_exchange(
                INTERRUPTION_NONE,
                INTERRUPTION_TIMEOUT,
                Ordering::AcqRel,
                Ordering::Acquire,
            );
        }
        self.reason.load(Ordering::Acquire) != INTERRUPTION_NONE
    }

    fn current_error(&self) -> Option<ApplicationError> {
        let _ = self.should_interrupt();
        match self.reason.load(Ordering::Acquire) {
            INTERRUPTION_CANCELLED => {
                Some(ApplicationError::cancelled(MARKDOWN_GENERATION_CANCELLED))
            }
            INTERRUPTION_TIMEOUT => Some(ApplicationError::timeout(MARKDOWN_GENERATION_TIMEOUT)),
            _ => None,
        }
    }
}

fn map_sqlite_error(
    error: &rusqlite::Error,
    interruption: &InterruptionControl,
    fallback: ApplicationError,
) -> ApplicationError {
    if matches!(
        error,
        rusqlite::Error::SqliteFailure(failure, _)
            if failure.code == ErrorCode::OperationInterrupted
    ) {
        return interruption.current_error().unwrap_or_else(unavailable);
    }
    interruption.current_error().unwrap_or(fallback)
}

fn map_local_session_discovery_error(error: ApplicationError) -> ApplicationError {
    match error.code().as_str() {
        "LOCAL_SESSION_DIRECTORY_INVALID_SQLITE_HOME" => {
            ApplicationError::unavailable(INVALID_SQLITE_HOME_CODE)
        }
        "LOCAL_SESSION_DIRECTORY_TOO_MANY_DATABASES" => {
            ApplicationError::unavailable(TOO_MANY_DATABASES_CODE)
        }
        _ => unavailable(),
    }
}

fn map_domain_error(error: MarkdownGenerationError) -> ApplicationError {
    match error {
        MarkdownGenerationError::TooManyMessages | MarkdownGenerationError::MarkdownTooLarge => {
            resource_limit()
        }
        MarkdownGenerationError::InvalidUtcTimestamp
        | MarkdownGenerationError::EmptyMessageBody
        | MarkdownGenerationError::NoMessages => invalid_content(),
    }
}

fn unavailable() -> ApplicationError {
    ApplicationError::unavailable(UNAVAILABLE_CODE)
}

fn unsupported_schema() -> ApplicationError {
    ApplicationError::unsupported(UNSUPPORTED_SCHEMA_CODE)
}

fn invalid_rollout() -> ApplicationError {
    ApplicationError::unavailable(INVALID_ROLLOUT_CODE)
}

fn invalid_content() -> ApplicationError {
    ApplicationError::unavailable(INVALID_CONTENT_CODE)
}

fn resource_limit() -> ApplicationError {
    ApplicationError::unavailable(RESOURCE_LIMIT_CODE)
}
