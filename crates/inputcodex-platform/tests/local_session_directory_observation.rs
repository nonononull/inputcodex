use std::{
    collections::HashMap,
    ffi::OsString,
    fs, io,
    mem::size_of,
    path::{Path, PathBuf},
    sync::atomic::{AtomicU64, Ordering},
    time::Duration,
};

use inputcodex_application::{
    ErrorKind, LocalSessionDirectoryCancellation, LocalSessionDirectoryObservationPort,
    LocalSessionDirectoryRequest,
};
use inputcodex_domain::LocalSessionSourceCoverage;
use inputcodex_platform::SystemLocalSessionDirectoryObservation;
use rusqlite::{Connection, OpenFlags};

pub(crate) use inputcodex_platform::SystemPlatformPaths;

#[allow(dead_code)]
#[path = "../src/local_session_directory_observation.rs"]
mod subject;

static NEXT_TEMP_ID: AtomicU64 = AtomicU64::new(1);

struct TestDirectory {
    path: PathBuf,
}

impl TestDirectory {
    fn new(label: &str) -> Self {
        let sequence = NEXT_TEMP_ID.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "inputcodex-issue-104-{label}-{}-{sequence}",
            std::process::id()
        ));
        if path.exists() {
            fs::remove_dir_all(&path).expect("清理旧测试目录应成功");
        }
        fs::create_dir_all(&path).expect("创建测试目录应成功");
        Self { path }
    }

    fn path(&self) -> &Path {
        &self.path
    }
}

impl Drop for TestDirectory {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}

fn database(path: &Path) -> Connection {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("创建数据库父目录应成功");
    }
    Connection::open_with_flags(
        path,
        OpenFlags::SQLITE_OPEN_READ_WRITE
            | OpenFlags::SQLITE_OPEN_CREATE
            | OpenFlags::SQLITE_OPEN_NO_MUTEX,
    )
    .expect("创建合成 SQLite 应成功")
}

fn observe(
    root: &Path,
    request: LocalSessionDirectoryRequest,
) -> Result<
    Option<inputcodex_domain::LocalSessionDirectoryPage>,
    inputcodex_application::ApplicationError,
> {
    subject::observe_local_session_directory_at_root(
        root,
        &request,
        &LocalSessionDirectoryCancellation::default(),
        subject::LocalSessionDirectoryObservationPolicy::default(),
    )
}

fn title(entry: &inputcodex_domain::LocalSessionDirectoryEntry) -> Option<(&str, bool)> {
    entry
        .display_title()
        .map(|value| (value.as_str(), value.was_truncated()))
}

fn assert_error(error: inputcodex_application::ApplicationError, kind: ErrorKind, code: &str) {
    assert_eq!(error.kind(), kind);
    assert_eq!(error.code().as_str(), code);
    let debug = format!("{error:?}");
    assert!(!debug.contains("inputcodex-issue-104"));
    assert!(!debug.contains("sqlite"));
}

#[test]
fn 系统观察器保持零字段并实现应用端口() {
    fn assert_port<T: LocalSessionDirectoryObservationPort>() {}

    assert_eq!(size_of::<SystemLocalSessionDirectoryObservation>(), 0);
    assert_port::<SystemLocalSessionDirectoryObservation>();
    assert_eq!(
        format!("{:?}", SystemLocalSessionDirectoryObservation),
        "SystemLocalSessionDirectoryObservation"
    );
}

#[test]
fn 无数据库返回_empty_且不创建任何文件() {
    let temp = TestDirectory::new("empty");
    let before = fs::read_dir(temp.path())
        .expect("读取空测试目录应成功")
        .count();

    let result = observe(temp.path(), LocalSessionDirectoryRequest::default())
        .expect("无数据库应是成功空结果");

    assert!(result.is_none());
    assert_eq!(
        fs::read_dir(temp.path())
            .expect("再次读取测试目录应成功")
            .count(),
        before
    );
    assert!(!temp.path().join("state_5.sqlite").exists());
}

#[test]
fn threads_schema_只投影批准字段并处理可选列() {
    let temp = TestDirectory::new("threads");
    let db_path = temp.path().join("sqlite").join("current.db");
    let db = database(&db_path);
    db.execute_batch(
        "CREATE TABLE threads (
            id TEXT PRIMARY KEY,
            title TEXT,
            archived INTEGER,
            updated_at INTEGER
        );",
    )
    .expect("创建 threads schema 应成功");
    db.execute(
        "INSERT INTO threads VALUES (?1, ?2, ?3, ?4)",
        ("thread-new", "  新\t会话\u{0007}  ", 1_i64, 300_i64),
    )
    .expect("插入新会话应成功");
    db.execute(
        "INSERT INTO threads VALUES (?1, ?2, ?3, ?4)",
        ("thread-old", Option::<String>::None, 0_i64, 100_i64),
    )
    .expect("插入旧会话应成功");
    drop(db);

    let page = observe(temp.path(), LocalSessionDirectoryRequest::default())
        .expect("threads 观察应成功")
        .expect("threads 应返回页面");

    assert_eq!(page.entries().len(), 2);
    assert_eq!(page.entries()[0].session_id(), "thread-new");
    assert_eq!(title(&page.entries()[0]), Some(("新 会话", false)));
    assert!(page.entries()[0].is_archived());
    assert_eq!(page.entries()[0].updated_at_ms(), Some(300_000));
    assert_eq!(page.entries()[1].session_id(), "thread-old");
    assert_eq!(title(&page.entries()[1]), None);
    assert!(!page.entries()[1].is_archived());
    assert_eq!(
        page.sources().coverage(),
        LocalSessionSourceCoverage::Complete
    );
}

#[test]
fn threads_仅有必需_id_时其余字段安全降级() {
    let temp = TestDirectory::new("threads-minimal");
    let db_path = temp.path().join("sqlite").join("minimal.sqlite");
    let db = database(&db_path);
    db.execute_batch("CREATE TABLE threads (id TEXT PRIMARY KEY);")
        .expect("创建最小 threads schema 应成功");
    db.execute("INSERT INTO threads VALUES (?1)", ["minimal-thread"])
        .expect("插入最小会话应成功");
    drop(db);

    let page = observe(temp.path(), LocalSessionDirectoryRequest::default())
        .expect("最小 threads 观察应成功")
        .expect("最小 threads 应返回页面");
    let entry = &page.entries()[0];

    assert_eq!(entry.session_id(), "minimal-thread");
    assert_eq!(title(entry), None);
    assert!(!entry.is_archived());
    assert_eq!(entry.updated_at_ms(), None);
}

#[test]
fn automation_runs_schema_映射标题归档和时间回退() {
    let temp = TestDirectory::new("automation");
    let db_path = temp.path().join("sqlite").join("automation.sqlite3");
    let db = database(&db_path);
    db.execute_batch(
        "CREATE TABLE automation_runs (
            thread_id TEXT PRIMARY KEY,
            status TEXT,
            thread_title TEXT,
            created_at INTEGER,
            updated_at INTEGER
        );",
    )
    .expect("创建 automation_runs schema 应成功");
    db.execute(
        "INSERT INTO automation_runs VALUES (?1, ?2, ?3, ?4, ?5)",
        ("auto-1", "running", "First", 100_i64, Option::<i64>::None),
    )
    .expect("插入第一条自动化会话应成功");
    db.execute(
        "INSERT INTO automation_runs VALUES (?1, ?2, ?3, ?4, ?5)",
        ("auto-2", "ARCHIVED", "Second", 200_i64, Some(400_i64)),
    )
    .expect("插入第二条自动化会话应成功");
    drop(db);

    let page = observe(temp.path(), LocalSessionDirectoryRequest::default())
        .expect("automation_runs 观察应成功")
        .expect("automation_runs 应返回页面");

    assert_eq!(page.entries()[0].session_id(), "auto-2");
    assert_eq!(title(&page.entries()[0]), Some(("Second", false)));
    assert!(page.entries()[0].is_archived());
    assert_eq!(page.entries()[0].updated_at_ms(), Some(400));
    assert_eq!(page.entries()[1].updated_at_ms(), Some(100));
}

#[test]
fn 跨库先排序去重再分页且完全相同时当前库优先() {
    let temp = TestDirectory::new("merge");
    let current_path = temp.path().join("sqlite").join("current.db");
    let legacy_path = temp.path().join("state_5.sqlite");

    let current = database(&current_path);
    current
        .execute_batch(
            "CREATE TABLE threads (
                id TEXT PRIMARY KEY,
                title TEXT,
                archived INTEGER,
                updated_at_ms INTEGER
            );",
        )
        .expect("创建当前库应成功");
    for row in [
        ("duplicate-new", "Current Older", 300_i64),
        ("tie", "Current Tie", 200_i64),
        ("zed", "Zed", 100_i64),
    ] {
        current
            .execute("INSERT INTO threads VALUES (?1, ?2, 0, ?3)", row)
            .expect("插入当前库会话应成功");
    }
    current
        .execute(
            "INSERT INTO threads VALUES (?1, ?2, 0, NULL)",
            ("no-time", "No Time"),
        )
        .expect("插入无时间会话应成功");
    drop(current);

    let legacy = database(&legacy_path);
    legacy
        .execute_batch(
            "CREATE TABLE threads (
                id TEXT PRIMARY KEY,
                title TEXT,
                archived INTEGER,
                updated_at_ms INTEGER
            );",
        )
        .expect("创建 legacy 库应成功");
    for row in [
        ("duplicate-new", "Legacy Newer", 400_i64),
        ("tie", "Legacy Tie", 200_i64),
        ("alpha", "Alpha", 100_i64),
    ] {
        legacy
            .execute("INSERT INTO threads VALUES (?1, ?2, 0, ?3)", row)
            .expect("插入 legacy 会话应成功");
    }
    drop(legacy);

    let first = observe(
        temp.path(),
        LocalSessionDirectoryRequest::new(0, 3).expect("第一页请求应合法"),
    )
    .expect("第一页观察应成功")
    .expect("第一页应存在");
    assert_eq!(
        first
            .entries()
            .iter()
            .map(|entry| entry.session_id())
            .collect::<Vec<_>>(),
        vec!["duplicate-new", "tie", "zed"]
    );
    assert_eq!(title(&first.entries()[0]), Some(("Legacy Newer", false)));
    assert_eq!(title(&first.entries()[1]), Some(("Current Tie", false)));
    assert!(first.has_more());

    let second = observe(
        temp.path(),
        LocalSessionDirectoryRequest::new(3, 3).expect("第二页请求应合法"),
    )
    .expect("第二页观察应成功")
    .expect("第二页应存在");
    assert_eq!(
        second
            .entries()
            .iter()
            .map(|entry| entry.session_id())
            .collect::<Vec<_>>(),
        vec!["alpha", "no-time"]
    );
    assert!(!second.has_more());
}

#[test]
fn 有条目时单个来源失败返回_ready_partial() {
    let temp = TestDirectory::new("partial");
    let valid_path = temp.path().join("sqlite").join("a-valid.db");
    let valid = database(&valid_path);
    valid
        .execute_batch("CREATE TABLE threads (id TEXT PRIMARY KEY);")
        .expect("创建有效来源应成功");
    valid
        .execute("INSERT INTO threads VALUES (?1)", ["valid-thread"])
        .expect("插入有效来源应成功");
    drop(valid);
    fs::write(
        temp.path().join("sqlite").join("z-broken.db"),
        b"not a sqlite database",
    )
    .expect("写入损坏来源应成功");

    let page = observe(temp.path(), LocalSessionDirectoryRequest::default())
        .expect("部分来源失败但已有条目应返回页面")
        .expect("部分来源应有页面");

    assert_eq!(page.entries()[0].session_id(), "valid-thread");
    assert_eq!(page.sources().discovered(), 2);
    assert_eq!(page.sources().readable(), 1);
    assert_eq!(page.sources().failed(), 1);
    assert_eq!(
        page.sources().coverage(),
        LocalSessionSourceCoverage::Partial
    );
}

#[test]
fn 可读空来源与失败来源并存时不得伪造_empty() {
    let temp = TestDirectory::new("empty-partial");
    let empty_path = temp.path().join("sqlite").join("a-empty.db");
    let empty = database(&empty_path);
    empty
        .execute_batch("CREATE TABLE threads (id TEXT PRIMARY KEY);")
        .expect("创建空来源应成功");
    drop(empty);
    fs::write(
        temp.path().join("sqlite").join("z-broken.db"),
        b"not a sqlite database",
    )
    .expect("写入损坏来源应成功");

    let error = observe(temp.path(), LocalSessionDirectoryRequest::default())
        .expect_err("空来源与失败来源并存必须失败");

    assert_error(
        error,
        ErrorKind::Unavailable,
        "LOCAL_SESSION_DIRECTORY_UNAVAILABLE",
    );
}

#[test]
fn 所有来源失败或_schema_不支持时返回稳定脱敏错误() {
    let broken = TestDirectory::new("all-broken");
    fs::create_dir_all(broken.path().join("sqlite")).expect("创建 sqlite 目录应成功");
    fs::write(
        broken.path().join("sqlite").join("broken.db"),
        b"not a sqlite database",
    )
    .expect("写入损坏库应成功");
    let unavailable = observe(broken.path(), LocalSessionDirectoryRequest::default())
        .expect_err("全部损坏来源必须失败");
    assert_error(
        unavailable,
        ErrorKind::Unavailable,
        "LOCAL_SESSION_DIRECTORY_UNAVAILABLE",
    );

    let unsupported = TestDirectory::new("unsupported");
    let db = database(&unsupported.path().join("sqlite").join("unsupported.db"));
    db.execute_batch("CREATE TABLE unrelated (id TEXT PRIMARY KEY);")
        .expect("创建不支持 schema 应成功");
    drop(db);
    let unsupported_error = observe(unsupported.path(), LocalSessionDirectoryRequest::default())
        .expect_err("不支持 schema 必须失败");
    assert_error(
        unsupported_error,
        ErrorKind::Unsupported,
        "LOCAL_SESSION_DIRECTORY_UNSUPPORTED_SCHEMA",
    );
}

#[derive(Default)]
struct MemoryFileProbe {
    kinds: HashMap<PathBuf, subject::LocalSessionPathKind>,
    entries: HashMap<PathBuf, Vec<PathBuf>>,
}

impl MemoryFileProbe {
    fn with_kind(mut self, path: PathBuf, kind: subject::LocalSessionPathKind) -> Self {
        self.kinds.insert(path, kind);
        self
    }

    fn with_entries(mut self, path: PathBuf, entries: Vec<PathBuf>) -> Self {
        self.entries.insert(path, entries);
        self
    }
}

impl subject::LocalSessionDirectoryFileProbe for MemoryFileProbe {
    fn kind(&self, path: &Path) -> io::Result<subject::LocalSessionPathKind> {
        Ok(self
            .kinds
            .get(path)
            .copied()
            .unwrap_or(subject::LocalSessionPathKind::Missing))
    }

    fn direct_entries(&self, path: &Path) -> io::Result<Vec<PathBuf>> {
        Ok(self.entries.get(path).cloned().unwrap_or_default())
    }
}

#[test]
fn 候选发现只接受直接普通_sqlite_文件并忽略符号链接和非普通文件() {
    let root = std::env::temp_dir().join("inputcodex-issue-104-memory-root");
    let sqlite = root.join("sqlite");
    let accepted = sqlite.join("accepted.DB");
    let nested = sqlite.join("nested");
    let symlink = sqlite.join("linked.sqlite");
    let other = sqlite.join("ignored.txt");
    let legacy = root.join("state_5.sqlite");
    let probe = MemoryFileProbe::default()
        .with_kind(root.clone(), subject::LocalSessionPathKind::Directory)
        .with_kind(sqlite.clone(), subject::LocalSessionPathKind::Directory)
        .with_kind(accepted.clone(), subject::LocalSessionPathKind::File)
        .with_kind(nested.clone(), subject::LocalSessionPathKind::Directory)
        .with_kind(symlink.clone(), subject::LocalSessionPathKind::Symlink)
        .with_kind(other.clone(), subject::LocalSessionPathKind::File)
        .with_kind(legacy, subject::LocalSessionPathKind::Symlink)
        .with_entries(sqlite, vec![nested, symlink, other, accepted.clone()]);

    let candidates = subject::discover_local_session_databases_with_probe(&root, &probe)
        .expect("候选发现应成功");

    assert_eq!(candidates.len(), 1);
    assert_eq!(candidates[0].path(), accepted);
    assert_eq!(candidates[0].priority(), 0);
}

#[test]
fn 候选超过三十二个时在打开数据库前失败() {
    let temp = TestDirectory::new("candidate-limit");
    let sqlite = temp.path().join("sqlite");
    fs::create_dir_all(&sqlite).expect("创建 sqlite 目录应成功");
    for index in 0..=subject::MAX_LOCAL_SESSION_DATABASES {
        fs::write(sqlite.join(format!("candidate-{index:02}.db")), b"")
            .expect("创建候选文件应成功");
    }

    let error = observe(temp.path(), LocalSessionDirectoryRequest::default())
        .expect_err("候选超限必须失败");

    assert_error(
        error,
        ErrorKind::Unavailable,
        "LOCAL_SESSION_DIRECTORY_TOO_MANY_DATABASES",
    );
}

#[test]
fn 非空显式_sqlite_root_非法时失败且不得回退() {
    let default_root = std::env::temp_dir().join("inputcodex-issue-104-default-root");
    let valid_override = std::env::temp_dir().join("inputcodex-issue-104-override-root");
    let missing = std::env::temp_dir().join("inputcodex-issue-104-missing-root");
    let file = std::env::temp_dir().join("inputcodex-issue-104-file-root");
    let symlink = std::env::temp_dir().join("inputcodex-issue-104-link-root");
    let probe = MemoryFileProbe::default()
        .with_kind(
            default_root.clone(),
            subject::LocalSessionPathKind::Directory,
        )
        .with_kind(
            valid_override.clone(),
            subject::LocalSessionPathKind::Directory,
        )
        .with_kind(file.clone(), subject::LocalSessionPathKind::File)
        .with_kind(symlink.clone(), subject::LocalSessionPathKind::Symlink);

    assert_eq!(
        subject::resolve_local_session_sqlite_root_with_probe(
            &default_root,
            Some(OsString::from("  ")),
            &probe,
        )
        .expect("空白覆盖应视为未配置"),
        default_root
    );
    assert_eq!(
        subject::resolve_local_session_sqlite_root_with_probe(
            &default_root,
            Some(valid_override.clone().into_os_string()),
            &probe,
        )
        .expect("合法绝对目录应成为覆盖根"),
        valid_override
    );

    for value in [
        OsString::from("relative/sqlite-root"),
        missing.into_os_string(),
        file.into_os_string(),
        symlink.into_os_string(),
    ] {
        let error = subject::resolve_local_session_sqlite_root_with_probe(
            &default_root,
            Some(value),
            &probe,
        )
        .expect_err("非法显式根不得回退");
        assert_error(
            error,
            ErrorKind::Unavailable,
            "LOCAL_SESSION_DIRECTORY_INVALID_SQLITE_HOME",
        );
    }
}

#[test]
fn 观察保持_wal_主库_日志和目录内容不变且_shm_仅作协调() {
    let temp = TestDirectory::new("readonly");
    let db_path = temp.path().join("sqlite").join("readonly.db");
    let db = database(&db_path);
    db.execute_batch(
        "PRAGMA journal_mode = WAL;
         PRAGMA wal_autocheckpoint = 0;
         CREATE TABLE threads (id TEXT PRIMARY KEY);
         INSERT INTO threads VALUES ('read-only');",
    )
    .expect("创建 WAL 只读验证库应成功");
    let wal_path = db_path.with_extension("db-wal");
    let shm_path = db_path.with_extension("db-shm");
    assert!(wal_path.is_file());
    assert!(shm_path.is_file());

    let before_bytes = [
        fs::read(&db_path).expect("读取观察前主库应成功"),
        fs::read(&wal_path).expect("读取观察前 WAL 应成功"),
        fs::read(&shm_path).expect("读取观察前 SHM 应成功"),
    ];
    let mut before_names = fs::read_dir(db_path.parent().expect("数据库应有父目录"))
        .expect("读取观察前目录应成功")
        .map(|entry| entry.expect("读取目录项应成功").file_name())
        .collect::<Vec<_>>();
    before_names.sort();

    let page = observe(temp.path(), LocalSessionDirectoryRequest::default())
        .expect("只读观察应成功")
        .expect("只读观察应返回页面");
    assert_eq!(page.entries()[0].session_id(), "read-only");

    let after_bytes = [
        fs::read(&db_path).expect("读取观察后主库应成功"),
        fs::read(&wal_path).expect("读取观察后 WAL 应成功"),
        fs::read(&shm_path).expect("读取观察后 SHM 应成功"),
    ];
    let mut after_names = fs::read_dir(db_path.parent().expect("数据库应有父目录"))
        .expect("读取观察后目录应成功")
        .map(|entry| entry.expect("读取目录项应成功").file_name())
        .collect::<Vec<_>>();
    after_names.sort();

    assert_eq!(after_bytes[0], before_bytes[0]);
    assert_eq!(after_bytes[1], before_bytes[1]);
    assert_eq!(after_bytes[2].len(), before_bytes[2].len());
    assert_eq!(after_names, before_names);
    drop(db);
}

#[test]
fn 取消和整体_deadline_返回各自稳定错误() {
    let temp = TestDirectory::new("interrupt");
    let db_path = temp.path().join("sqlite").join("interrupt.db");
    let db = database(&db_path);
    db.execute_batch("CREATE TABLE threads (id TEXT PRIMARY KEY);")
        .expect("创建中断验证库应成功");
    db.execute("INSERT INTO threads VALUES (?1)", ["interrupt"])
        .expect("插入中断验证行应成功");
    drop(db);

    let cancellation = LocalSessionDirectoryCancellation::default();
    cancellation.cancel();
    let cancelled = subject::observe_local_session_directory_at_root(
        temp.path(),
        &LocalSessionDirectoryRequest::default(),
        &cancellation,
        subject::LocalSessionDirectoryObservationPolicy::default(),
    )
    .expect_err("预取消观察必须失败");
    assert_error(
        cancelled,
        ErrorKind::Cancelled,
        "LOCAL_SESSION_DIRECTORY_CANCELLED",
    );

    let timeout = subject::observe_local_session_directory_at_root(
        temp.path(),
        &LocalSessionDirectoryRequest::default(),
        &LocalSessionDirectoryCancellation::default(),
        subject::LocalSessionDirectoryObservationPolicy::new(
            Duration::ZERO,
            Duration::from_millis(1),
            1,
        ),
    )
    .expect_err("零时限观察必须超时");
    assert_error(
        timeout,
        ErrorKind::Timeout,
        "LOCAL_SESSION_DIRECTORY_TIMEOUT",
    );
}

#[test]
fn 数据库独占锁不会无限等待并返回脱敏_unavailable() {
    let temp = TestDirectory::new("locked");
    let db_path = temp.path().join("sqlite").join("locked.db");
    let db = database(&db_path);
    db.execute_batch(
        "CREATE TABLE threads (id TEXT PRIMARY KEY);
         INSERT INTO threads VALUES ('locked');
         BEGIN EXCLUSIVE;",
    )
    .expect("建立独占锁应成功");

    let error = subject::observe_local_session_directory_at_root(
        temp.path(),
        &LocalSessionDirectoryRequest::default(),
        &LocalSessionDirectoryCancellation::default(),
        subject::LocalSessionDirectoryObservationPolicy::new(
            Duration::from_millis(200),
            Duration::from_millis(5),
            1,
        ),
    )
    .expect_err("独占锁读取必须有界失败");

    assert_error(
        error,
        ErrorKind::Unavailable,
        "LOCAL_SESSION_DIRECTORY_UNAVAILABLE",
    );
    drop(db);
}
