use std::{
    collections::HashSet,
    ffi::{OsStr, OsString},
    fs, io,
    path::{Path, PathBuf},
    sync::{
        Arc,
        atomic::{AtomicU8, Ordering},
    },
    time::{Duration, Instant},
};

use inputcodex_application::{
    ApplicationError, ErrorKind, LocalSessionDirectoryCancellation,
    LocalSessionDirectoryObservationPort, LocalSessionDirectoryRequest,
};
#[cfg(any(target_os = "windows", target_os = "macos"))]
use inputcodex_application::{PlatformPathsPort, PlatformPathsRequest};
use inputcodex_domain::{
    LocalSessionDirectoryEntry, LocalSessionDirectoryPage, LocalSessionSourceSummary,
    LocalSessionTitle,
};
use rusqlite::{Connection, ErrorCode, OpenFlags, OptionalExtension};

#[cfg(any(target_os = "windows", target_os = "macos"))]
use crate::SystemPlatformPaths;

const INVALID_PAGINATION_CODE: &str = "LOCAL_SESSION_DIRECTORY_INVALID_PAGINATION";
const INVALID_SQLITE_HOME_CODE: &str = "LOCAL_SESSION_DIRECTORY_INVALID_SQLITE_HOME";
const TOO_MANY_DATABASES_CODE: &str = "LOCAL_SESSION_DIRECTORY_TOO_MANY_DATABASES";
const UNSUPPORTED_SCHEMA_CODE: &str = "LOCAL_SESSION_DIRECTORY_UNSUPPORTED_SCHEMA";
const UNAVAILABLE_CODE: &str = "LOCAL_SESSION_DIRECTORY_UNAVAILABLE";
const TIMEOUT_CODE: &str = "LOCAL_SESSION_DIRECTORY_TIMEOUT";
const CANCELLED_CODE: &str = "LOCAL_SESSION_DIRECTORY_CANCELLED";
const SQLITE_HOME_ENV: &str = "CODEX_SQLITE_HOME";
const SQLITE_DIRECTORY: &str = "sqlite";
const LEGACY_DATABASE: &str = "state_5.sqlite";
const DEFAULT_OBSERVATION_TIMEOUT: Duration = Duration::from_secs(2);
const DEFAULT_BUSY_TIMEOUT: Duration = Duration::from_millis(50);
const DEFAULT_PROGRESS_STEPS: i32 = 1_000;
const INTERRUPTION_NONE: u8 = 0;
const INTERRUPTION_CANCELLED: u8 = 1;
const INTERRUPTION_TIMEOUT: u8 = 2;

pub(super) const MAX_LOCAL_SESSION_DATABASES: usize = 32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum LocalSessionPathKind {
    Missing,
    File,
    Directory,
    Symlink,
    Other,
}

pub(super) trait LocalSessionDirectoryFileProbe {
    fn kind(&self, path: &Path) -> io::Result<LocalSessionPathKind>;
    fn direct_entries(&self, path: &Path) -> io::Result<Vec<PathBuf>>;
}

#[derive(Debug, Clone, Copy, Default)]
pub(super) struct SystemLocalSessionDirectoryFileProbe;

impl LocalSessionDirectoryFileProbe for SystemLocalSessionDirectoryFileProbe {
    fn kind(&self, path: &Path) -> io::Result<LocalSessionPathKind> {
        match fs::symlink_metadata(path) {
            Ok(metadata) => {
                let file_type = metadata.file_type();
                if file_type.is_symlink() {
                    Ok(LocalSessionPathKind::Symlink)
                } else if metadata.is_file() {
                    Ok(LocalSessionPathKind::File)
                } else if metadata.is_dir() {
                    Ok(LocalSessionPathKind::Directory)
                } else {
                    Ok(LocalSessionPathKind::Other)
                }
            }
            Err(error) if error.kind() == io::ErrorKind::NotFound => {
                Ok(LocalSessionPathKind::Missing)
            }
            Err(error) => Err(error),
        }
    }

    fn direct_entries(&self, path: &Path) -> io::Result<Vec<PathBuf>> {
        fs::read_dir(path)?
            .map(|entry| entry.map(|value| value.path()))
            .collect()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct LocalSessionDatabaseCandidate {
    path: PathBuf,
    priority: usize,
}

impl LocalSessionDatabaseCandidate {
    #[must_use]
    pub(super) fn path(&self) -> &Path {
        &self.path
    }

    #[must_use]
    pub(super) const fn priority(&self) -> usize {
        self.priority
    }
}

#[derive(Debug, Clone, Copy)]
pub(super) struct LocalSessionDirectoryObservationPolicy {
    observation_timeout: Duration,
    busy_timeout: Duration,
    progress_steps: i32,
}

impl Default for LocalSessionDirectoryObservationPolicy {
    fn default() -> Self {
        Self::new(
            DEFAULT_OBSERVATION_TIMEOUT,
            DEFAULT_BUSY_TIMEOUT,
            DEFAULT_PROGRESS_STEPS,
        )
    }
}

impl LocalSessionDirectoryObservationPolicy {
    #[must_use]
    pub(super) const fn new(
        observation_timeout: Duration,
        busy_timeout: Duration,
        progress_steps: i32,
    ) -> Self {
        Self {
            observation_timeout,
            busy_timeout,
            progress_steps,
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct SystemLocalSessionDirectoryObservation;

impl LocalSessionDirectoryObservationPort for SystemLocalSessionDirectoryObservation {
    fn observe(
        &self,
        request: &LocalSessionDirectoryRequest,
        cancellation: &LocalSessionDirectoryCancellation,
    ) -> Result<Option<LocalSessionDirectoryPage>, ApplicationError> {
        #[cfg(any(target_os = "windows", target_os = "macos"))]
        {
            let paths = SystemPlatformPaths.resolve(&PlatformPathsRequest::default())?;
            let root = resolve_local_session_sqlite_root_with_probe(
                paths.codex_home().as_path(),
                std::env::var_os(SQLITE_HOME_ENV),
                &SystemLocalSessionDirectoryFileProbe,
            )?;
            observe_local_session_directory_at_root(
                &root,
                request,
                cancellation,
                LocalSessionDirectoryObservationPolicy::default(),
            )
        }

        #[cfg(not(any(target_os = "windows", target_os = "macos")))]
        {
            let _ = (request, cancellation);
            Err(ApplicationError::unsupported(
                "LOCAL_SESSION_DIRECTORY_UNSUPPORTED",
            ))
        }
    }
}

pub(super) fn resolve_local_session_sqlite_root_with_probe(
    codex_home: &Path,
    override_value: Option<OsString>,
    probe: &impl LocalSessionDirectoryFileProbe,
) -> Result<PathBuf, ApplicationError> {
    let Some(value) = override_value else {
        return Ok(codex_home.to_path_buf());
    };
    if value.to_string_lossy().trim().is_empty() {
        return Ok(codex_home.to_path_buf());
    }

    let path = PathBuf::from(value);
    let is_supported =
        path.is_absolute() && matches!(probe.kind(&path), Ok(LocalSessionPathKind::Directory));
    if !is_supported {
        return Err(ApplicationError::unavailable(INVALID_SQLITE_HOME_CODE));
    }

    Ok(path)
}

pub(super) fn discover_local_session_databases_with_probe(
    root: &Path,
    probe: &impl LocalSessionDirectoryFileProbe,
) -> Result<Vec<LocalSessionDatabaseCandidate>, ApplicationError> {
    let sqlite_directory = root.join(SQLITE_DIRECTORY);
    let mut paths = match probe.kind(&sqlite_directory).map_err(|_| unavailable())? {
        LocalSessionPathKind::Missing => Vec::new(),
        LocalSessionPathKind::Directory => probe
            .direct_entries(&sqlite_directory)
            .map_err(|_| unavailable())?,
        LocalSessionPathKind::File
        | LocalSessionPathKind::Symlink
        | LocalSessionPathKind::Other => return Err(unavailable()),
    };

    paths.retain(|path| is_sqlite_candidate(path));
    let mut current = Vec::new();
    for path in paths {
        if probe.kind(&path).map_err(|_| unavailable())? == LocalSessionPathKind::File {
            current.push(path);
        }
    }
    current.sort_by(|left, right| left.file_name().cmp(&right.file_name()));

    let legacy = root.join(LEGACY_DATABASE);
    if probe.kind(&legacy).map_err(|_| unavailable())? == LocalSessionPathKind::File
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
        .map(|(priority, path)| LocalSessionDatabaseCandidate { path, priority })
        .collect())
}

pub(super) fn observe_local_session_directory_at_root(
    root: &Path,
    request: &LocalSessionDirectoryRequest,
    cancellation: &LocalSessionDirectoryCancellation,
    policy: LocalSessionDirectoryObservationPolicy,
) -> Result<Option<LocalSessionDirectoryPage>, ApplicationError> {
    let deadline = Instant::now()
        .checked_add(policy.observation_timeout)
        .unwrap_or_else(Instant::now);
    let interruption = InterruptionControl::new(cancellation.clone(), deadline);
    interruption.ensure_active()?;

    let candidates =
        discover_local_session_databases_with_probe(root, &SystemLocalSessionDirectoryFileProbe)?;
    if candidates.is_empty() {
        return Ok(None);
    }

    let mut entries = Vec::new();
    let mut readable = 0;
    let mut failed = 0;
    let mut first_error = None;

    for candidate in &candidates {
        interruption.ensure_active()?;
        match read_candidate(candidate, request, policy, &interruption) {
            Ok(mut source_entries) => {
                readable += 1;
                entries.append(&mut source_entries);
            }
            Err(error) if matches!(error.kind(), ErrorKind::Cancelled | ErrorKind::Timeout) => {
                return Err(error);
            }
            Err(error) => {
                failed += 1;
                first_error.get_or_insert(error);
            }
        }
    }

    sort_and_deduplicate(&mut entries);
    let page_end = request.source_row_limit() - 1;
    let has_more = entries.len() > page_end;
    let page_entries = entries
        .into_iter()
        .skip(request.offset())
        .take(request.limit())
        .map(|sourced| sourced.entry)
        .collect::<Vec<_>>();

    if page_entries.is_empty() {
        return if failed == 0 {
            Ok(None)
        } else {
            Err(first_error.unwrap_or_else(unavailable))
        };
    }

    let sources = LocalSessionSourceSummary::new(candidates.len(), readable, failed)
        .map_err(|_| unavailable())?;
    LocalSessionDirectoryPage::new(
        page_entries,
        request.offset(),
        request.limit(),
        has_more,
        sources,
    )
    .map(Some)
    .map_err(|_| unavailable())
}

fn is_sqlite_candidate(path: &Path) -> bool {
    path.extension()
        .and_then(OsStr::to_str)
        .is_some_and(|value| {
            value.eq_ignore_ascii_case("db")
                || value.eq_ignore_ascii_case("sqlite")
                || value.eq_ignore_ascii_case("sqlite3")
        })
}

#[derive(Clone)]
struct InterruptionControl {
    cancellation: LocalSessionDirectoryCancellation,
    deadline: Instant,
    reason: Arc<AtomicU8>,
}

impl InterruptionControl {
    fn new(cancellation: LocalSessionDirectoryCancellation, deadline: Instant) -> Self {
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
            INTERRUPTION_CANCELLED => Some(ApplicationError::cancelled(CANCELLED_CODE)),
            INTERRUPTION_TIMEOUT => Some(ApplicationError::timeout(TIMEOUT_CODE)),
            _ => None,
        }
    }
}

struct SourcedSessionEntry {
    entry: LocalSessionDirectoryEntry,
    priority: usize,
}

fn read_candidate(
    candidate: &LocalSessionDatabaseCandidate,
    request: &LocalSessionDirectoryRequest,
    policy: LocalSessionDirectoryObservationPolicy,
    interruption: &InterruptionControl,
) -> Result<Vec<SourcedSessionEntry>, ApplicationError> {
    let flags = OpenFlags::SQLITE_OPEN_READ_ONLY | OpenFlags::SQLITE_OPEN_NO_MUTEX;
    let connection = Connection::open_with_flags(candidate.path(), flags)
        .map_err(|error| map_sqlite_error(&error, interruption, unavailable()))?;
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

    if has_table(&connection, "threads", interruption)? {
        let columns = table_columns(&connection, SupportedTable::Threads, interruption)?;
        if columns.contains("id") {
            return read_threads(
                &connection,
                &columns,
                request,
                candidate.priority(),
                interruption,
            );
        }
    }

    if has_table(&connection, "automation_runs", interruption)? {
        let columns = table_columns(&connection, SupportedTable::AutomationRuns, interruption)?;
        if columns.contains("thread_id") {
            return read_automation_runs(
                &connection,
                &columns,
                request,
                candidate.priority(),
                interruption,
            );
        }
    }

    Err(ApplicationError::unsupported(UNSUPPORTED_SCHEMA_CODE))
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
        .map_err(|error| map_sqlite_error(&error, interruption, unavailable()))?;
    let rows = statement
        .query_map([], |row| row.get::<_, String>(1))
        .map_err(|error| map_sqlite_error(&error, interruption, unavailable()))?;
    let mut columns = HashSet::new();
    for row in rows {
        columns.insert(row.map_err(|error| map_sqlite_error(&error, interruption, unavailable()))?);
    }
    Ok(columns)
}

fn read_threads(
    connection: &Connection,
    columns: &HashSet<String>,
    request: &LocalSessionDirectoryRequest,
    priority: usize,
    interruption: &InterruptionControl,
) -> Result<Vec<SourcedSessionEntry>, ApplicationError> {
    let title = optional_expression(columns, "title", "title", "NULL");
    let archived = optional_expression(columns, "archived", "archived", "0");
    let updated_at = if columns.contains("updated_at_ms") {
        "updated_at_ms"
    } else if columns.contains("updated_at") {
        "updated_at * 1000"
    } else if columns.contains("created_at_ms") {
        "created_at_ms"
    } else {
        "NULL"
    };
    let sql = format!(
        "SELECT id, {title} AS display_title, {archived} AS archived, \
         {updated_at} AS updated_at_ms FROM threads \
         ORDER BY updated_at_ms IS NULL ASC, updated_at_ms DESC, id DESC LIMIT ?1"
    );
    read_rows(connection, &sql, request, priority, interruption, |row| {
        let session_id = row.get::<_, String>(0)?;
        let raw_title = row.get::<_, Option<String>>(1)?;
        let archived = row.get::<_, Option<i64>>(2)?.unwrap_or_default() != 0;
        let updated_at_ms = row.get::<_, Option<i64>>(3)?;
        Ok((session_id, raw_title, archived, updated_at_ms))
    })
}

fn read_automation_runs(
    connection: &Connection,
    columns: &HashSet<String>,
    request: &LocalSessionDirectoryRequest,
    priority: usize,
    interruption: &InterruptionControl,
) -> Result<Vec<SourcedSessionEntry>, ApplicationError> {
    let title = optional_expression(columns, "thread_title", "thread_title", "NULL");
    let status = optional_expression(columns, "status", "status", "NULL");
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
        "SELECT thread_id, {title} AS display_title, {status} AS status, \
         {updated_at} AS updated_at_ms FROM automation_runs \
         ORDER BY updated_at_ms IS NULL ASC, updated_at_ms DESC, thread_id DESC LIMIT ?1"
    );
    read_rows(connection, &sql, request, priority, interruption, |row| {
        let session_id = row.get::<_, String>(0)?;
        let raw_title = row.get::<_, Option<String>>(1)?;
        let archived = row
            .get::<_, Option<String>>(2)?
            .is_some_and(|status| status.eq_ignore_ascii_case("archived"));
        let updated_at_ms = row.get::<_, Option<i64>>(3)?;
        Ok((session_id, raw_title, archived, updated_at_ms))
    })
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

fn read_rows<F>(
    connection: &Connection,
    sql: &str,
    request: &LocalSessionDirectoryRequest,
    priority: usize,
    interruption: &InterruptionControl,
    mut project: F,
) -> Result<Vec<SourcedSessionEntry>, ApplicationError>
where
    F: FnMut(&rusqlite::Row<'_>) -> rusqlite::Result<(String, Option<String>, bool, Option<i64>)>,
{
    let source_row_limit = i64::try_from(request.source_row_limit())
        .map_err(|_| ApplicationError::invalid_input(INVALID_PAGINATION_CODE))?;
    let mut statement = connection
        .prepare(sql)
        .map_err(|error| map_sqlite_error(&error, interruption, unsupported_schema()))?;
    let mut rows = statement
        .query([source_row_limit])
        .map_err(|error| map_sqlite_error(&error, interruption, unsupported_schema()))?;
    let mut entries = Vec::new();

    loop {
        let row = rows
            .next()
            .map_err(|error| map_sqlite_error(&error, interruption, unsupported_schema()))?;
        let Some(row) = row else {
            break;
        };
        let (session_id, raw_title, archived, updated_at_ms) = project(row)
            .map_err(|error| map_sqlite_error(&error, interruption, unsupported_schema()))?;
        let display_title = raw_title.as_deref().and_then(LocalSessionTitle::from_raw);
        let entry =
            LocalSessionDirectoryEntry::new(session_id, display_title, archived, updated_at_ms)
                .map_err(|_| unsupported_schema())?;
        entries.push(SourcedSessionEntry { entry, priority });
    }

    Ok(entries)
}

fn sort_and_deduplicate(entries: &mut Vec<SourcedSessionEntry>) {
    entries.sort_by(|left, right| {
        right
            .entry
            .updated_at_ms()
            .cmp(&left.entry.updated_at_ms())
            .then_with(|| right.entry.session_id().cmp(left.entry.session_id()))
            .then_with(|| left.priority.cmp(&right.priority))
    });
    let mut seen = HashSet::new();
    entries.retain(|entry| seen.insert(entry.entry.session_id().to_owned()));
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

fn unavailable() -> ApplicationError {
    ApplicationError::unavailable(UNAVAILABLE_CODE)
}

fn unsupported_schema() -> ApplicationError {
    ApplicationError::unsupported(UNSUPPORTED_SCHEMA_CODE)
}
