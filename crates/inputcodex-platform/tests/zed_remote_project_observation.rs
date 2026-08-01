use std::{
    ffi::OsString,
    fs,
    path::{Path, PathBuf},
    sync::atomic::{AtomicU64, Ordering},
    time::Duration,
};

#[cfg(not(any(target_os = "windows", target_os = "macos")))]
use inputcodex_application::ZedRemoteProjectObservationRequest;
use inputcodex_application::{
    ErrorKind, ZedRemoteProjectObservationCancellation, ZedRemoteProjectObservationPort,
};
use inputcodex_domain::{
    ZedRemoteProjectOrigin, ZedRemoteProjectSelectionHint, ZedRemoteProjectSourceCoverage,
};
use inputcodex_platform::SystemZedRemoteProjectObservation;
use rusqlite::{Connection, OpenFlags};
use serde_json::{Value, json};

#[cfg(any(target_os = "windows", target_os = "macos"))]
pub(crate) fn resolve_codex_home_for_observation()
-> Result<PathBuf, inputcodex_application::ApplicationError> {
    Err(inputcodex_application::ApplicationError::unavailable(
        "ZED_REMOTE_PROJECT_TEST_PATH_OWNER_NOT_INVOKED",
    ))
}

#[allow(dead_code)]
#[path = "../src/zed_remote_project_observation.rs"]
mod subject;

static NEXT_TEMP_ID: AtomicU64 = AtomicU64::new(1);

struct TestDirectory {
    path: PathBuf,
}

impl TestDirectory {
    fn new(label: &str) -> Self {
        let sequence = NEXT_TEMP_ID.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "inputcodex-issue-132-{label}-{}-{sequence}",
            std::process::id()
        ));
        if path.exists() {
            fs::remove_dir_all(&path).expect("清理旧测试目录应成功");
        }
        fs::create_dir_all(&path).expect("创建测试目录应成功");
        #[cfg(target_os = "macos")]
        let path = fs::canonicalize(path).expect("规范化 macOS 测试目录应成功");
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

fn write_state(root: &Path, state: &Value) {
    fs::write(
        root.join(".codex-global-state.json"),
        serde_json::to_vec(state).expect("序列化合成状态应成功"),
    )
    .expect("写入合成状态应成功");
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

fn base_state() -> Value {
    json!({
        "selected-remote-host-id": "host-main",
        "codex-managed-remote-connections": [
            {
                "hostId": "host-main",
                "hostname": "alice@example.internal",
                "sshPort": null
            },
            {
                "hostId": "host-other",
                "sshHost": "bob@other.internal",
                "sshPort": 22
            }
        ],
        "remote-projects": [
            {
                "id": "main",
                "hostId": "host-main",
                "remotePath": "/srv/private/repository"
            },
            {
                "id": "other",
                "hostId": "host-other",
                "remotePath": "/opt/other/project"
            }
        ],
        "project-order": ["main", "other"],
        "thread-workspace-root-hints": {
            "thread-132": {
                "hostId": "host-main",
                "remotePath": "/srv/private/repository/worktree"
            }
        }
    })
}

fn observe(
    root: &Path,
) -> Result<
    Option<inputcodex_domain::ZedRemoteProjectObservation>,
    inputcodex_application::ApplicationError,
> {
    subject::observe_zed_remote_projects_at_roots(
        root,
        root,
        &ZedRemoteProjectObservationCancellation::default(),
        subject::ZedRemoteProjectObservationPolicy::default(),
    )
}

fn assert_error(error: inputcodex_application::ApplicationError, kind: ErrorKind, code: &str) {
    assert_eq!(error.kind(), kind);
    assert_eq!(error.code().as_str(), code);
    let debug = format!("{error:?}");
    for forbidden in [
        "alice",
        "example.internal",
        "/srv/private",
        "host-main",
        "inputcodex-issue-132",
    ] {
        assert!(!debug.contains(forbidden), "错误泄漏禁止内容: {forbidden}");
    }
}

fn directory_names(path: &Path) -> Vec<OsString> {
    let mut names = fs::read_dir(path)
        .expect("读取测试目录应成功")
        .map(|entry| entry.expect("读取目录项应成功").file_name())
        .collect::<Vec<_>>();
    names.sort();
    names
}

#[test]
fn 生产源码固定只读_nofollow_与禁止能力边界() {
    let source = include_str!("../src/zed_remote_project_observation.rs");
    for required in [
        "OpenFlags::SQLITE_OPEN_READ_ONLY",
        "OpenFlags::SQLITE_OPEN_NOFOLLOW",
        "progress_handler",
        "\"query_only\"",
        "FILE_FLAG_OPEN_REPARSE_POINT",
        "O_NOFOLLOW",
        "read_bounded_optional_file",
        "Sha256::digest",
    ] {
        assert!(source.contains(required), "生产只读边界缺失: {required}");
    }
    for forbidden in [
        "zed_remote_projects.json",
        "auth.json",
        "Command::new",
        "TcpStream",
        "reqwest",
        "std::fs::write",
        "remove_file",
        "create_dir",
        "unsafe {",
    ] {
        assert!(
            !source.contains(forbidden),
            "生产源码命中禁止能力: {forbidden}"
        );
    }
}

#[test]
fn 系统适配器实现零字段应用端口() {
    fn assert_port<T: ZedRemoteProjectObservationPort + Default>() {}
    assert_port::<SystemZedRemoteProjectObservation>();

    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    {
        let error = SystemZedRemoteProjectObservation
            .observe(
                &ZedRemoteProjectObservationRequest,
                &ZedRemoteProjectObservationCancellation::default(),
            )
            .expect_err("非发布目标必须明确失败");
        assert_error(
            error,
            ErrorKind::Unsupported,
            "ZED_REMOTE_PROJECT_OBSERVATION_UNSUPPORTED",
        );
    }
}

#[test]
fn 稳定假名锁定跨平台向量并区分_none_与显式二十二端口() {
    let none = subject::stable_project_id(
        " alice ",
        " example.internal ",
        None,
        " /srv/private/repository ",
    )
    .expect("合法身份应形成稳定假名");
    let explicit = subject::stable_project_id(
        "alice",
        "example.internal",
        Some(22),
        "/srv/private/repository",
    )
    .expect("显式端口身份应形成稳定假名");

    assert_eq!(
        none.as_str(),
        "zed-remote-project:v1:sha256:d75b0c8cc035934d8078d15553305a9fed6f3a70862bfcab0b55715b2a73b39e"
    );
    assert_eq!(
        explicit.as_str(),
        "zed-remote-project:v1:sha256:b69c8abbcf5a203deba7d16965e202646e786f7d723f9cda30d6e3663788e472"
    );
    assert_ne!(none, explicit);

    let left = subject::stable_project_id("a|b", "c", None, "/d").unwrap();
    let right = subject::stable_project_id("a", "b|c", None, "/d").unwrap();
    assert_ne!(left, right, "长度前缀必须关闭旧 FNV 分隔符歧义");
}

#[test]
fn 全局状态与_sqlite_形成三种来源且只公开稳定假名和选择提示() {
    let temp = TestDirectory::new("complete");
    write_state(temp.path(), &base_state());
    let db_path = temp.path().join("sqlite/state.db");
    let db = database(&db_path);
    db.execute_batch(
        "CREATE TABLE threads (id TEXT PRIMARY KEY, cwd TEXT NOT NULL);\n\
         INSERT INTO threads (id, cwd) VALUES ('thread-db', '/srv/private/repository/db-worktree');",
    )
    .expect("写入合成线程应成功");
    drop(db);

    let observation = observe(temp.path())
        .expect("完整来源应成功")
        .expect("应观察到项目");
    assert_eq!(observation.project_count(), 4);
    assert_eq!(observation.sources().discovered(), 2);
    assert_eq!(observation.sources().readable(), 2);
    assert_eq!(observation.sources().failed(), 0);
    assert_eq!(
        observation.sources().coverage(),
        ZedRemoteProjectSourceCoverage::Complete
    );

    let origins = observation
        .projects()
        .iter()
        .map(|project| project.origin())
        .collect::<Vec<_>>();
    assert_eq!(
        origins
            .iter()
            .filter(|origin| **origin == ZedRemoteProjectOrigin::CodexRemoteProject)
            .count(),
        2
    );
    assert!(origins.contains(&ZedRemoteProjectOrigin::ThreadWorkspaceHint));
    assert!(origins.contains(&ZedRemoteProjectOrigin::SqliteThreadCwd));
    assert_eq!(
        observation
            .projects()
            .iter()
            .filter(|project| {
                project.selection_hint() == ZedRemoteProjectSelectionHint::SelectedHostHint
            })
            .count(),
        3
    );

    let ids = observation
        .projects()
        .iter()
        .map(|project| project.id().as_str())
        .collect::<Vec<_>>();
    assert!(ids.windows(2).all(|pair| pair[0] < pair[1]));

    let debug = format!("{observation:?}");
    for forbidden in [
        "alice",
        "bob",
        "example.internal",
        "other.internal",
        "/srv/private",
        "/opt/other",
        "host-main",
        "host-other",
        "ssh://",
        "label",
    ] {
        assert!(!debug.contains(forbidden), "观察泄漏禁止内容: {forbidden}");
    }
}

#[test]
fn 同一身份跨来源去重并保留高优先级来源和选择提示() {
    let temp = TestDirectory::new("dedupe");
    let mut state = base_state();
    state["thread-workspace-root-hints"] = json!({
        "thread-132": {
            "hostId": "host-main",
            "remotePath": "/srv/private/repository"
        }
    });
    write_state(temp.path(), &state);

    let observation = observe(temp.path())
        .expect("重复来源应成功")
        .expect("应观察到项目");
    assert_eq!(observation.project_count(), 2);
    let selected = observation
        .projects()
        .iter()
        .find(|project| project.selection_hint() == ZedRemoteProjectSelectionHint::SelectedHostHint)
        .expect("应保留选择提示");
    assert_eq!(
        selected.origin(),
        ZedRemoteProjectOrigin::CodexRemoteProject
    );
}

#[test]
fn 缺失来源返回_empty_而损坏来源按有无项目区分_failed_与_partial() {
    let empty = TestDirectory::new("empty");
    assert_eq!(observe(empty.path()).expect("来源全缺失应成功"), None);

    let malformed = TestDirectory::new("malformed");
    fs::write(malformed.path().join(".codex-global-state.json"), b"{").expect("写入损坏状态应成功");
    let error = observe(malformed.path()).expect_err("损坏且无项目必须失败");
    assert_error(
        error,
        ErrorKind::InvalidInput,
        "ZED_REMOTE_PROJECT_OBSERVATION_INVALID_STATE",
    );

    let partial = TestDirectory::new("partial");
    write_state(partial.path(), &base_state());
    fs::create_dir_all(partial.path().join("sqlite")).unwrap();
    fs::write(partial.path().join("sqlite/corrupt.db"), b"not sqlite").unwrap();
    let observation = observe(partial.path())
        .expect("已有全局项目时单库损坏应返回部分结果")
        .expect("应保留全局项目");
    assert_eq!(
        observation.sources().coverage(),
        ZedRemoteProjectSourceCoverage::Partial
    );
    assert_eq!(observation.sources().failed(), 1);

    let failed = TestDirectory::new("failed");
    write_state(
        failed.path(),
        &json!({
            "codex-managed-remote-connections": [],
            "remote-projects": [],
            "thread-workspace-root-hints": {}
        }),
    );
    fs::create_dir_all(failed.path().join("sqlite")).unwrap();
    fs::write(failed.path().join("sqlite/corrupt.db"), b"not sqlite").unwrap();
    let error = observe(failed.path()).expect_err("无项目且存在损坏来源必须失败");
    assert_eq!(error.kind(), ErrorKind::Unavailable);
}

#[test]
fn 全局状态_schema_与资源上限保持_fail_closed() {
    let invalid_schema = TestDirectory::new("invalid-schema");
    write_state(
        invalid_schema.path(),
        &json!({"remote-projects": "not-an-array"}),
    );
    let error = observe(invalid_schema.path()).expect_err("错误 schema 必须失败");
    assert_error(
        error,
        ErrorKind::InvalidInput,
        "ZED_REMOTE_PROJECT_OBSERVATION_INVALID_STATE",
    );

    let oversized = TestDirectory::new("oversized-state");
    fs::write(
        oversized.path().join(".codex-global-state.json"),
        vec![b' '; subject::MAX_GLOBAL_STATE_BYTES + 1],
    )
    .unwrap();
    let error = observe(oversized.path()).expect_err("超大状态必须失败");
    assert_error(
        error,
        ErrorKind::Unavailable,
        "ZED_REMOTE_PROJECT_OBSERVATION_RESOURCE_LIMIT",
    );

    let too_many = TestDirectory::new("too-many-projects");
    let projects = (0..=inputcodex_domain::MAX_ZED_REMOTE_PROJECTS)
        .map(|index| {
            json!({
                "id": format!("project-{index}"),
                "hostId": "host-main",
                "remotePath": format!("/srv/private/repository/{index}")
            })
        })
        .collect::<Vec<_>>();
    let mut state = base_state();
    state["remote-projects"] = Value::Array(projects);
    state["thread-workspace-root-hints"] = json!({});
    write_state(too_many.path(), &state);
    let error = observe(too_many.path()).expect_err("最终项目超限必须失败");
    assert_error(
        error,
        ErrorKind::Unavailable,
        "ZED_REMOTE_PROJECT_OBSERVATION_RESOURCE_LIMIT",
    );
}

#[test]
fn sqlite_候选行和值均有界且错误不发布假候选() {
    let too_many_databases = TestDirectory::new("too-many-databases");
    fs::create_dir_all(too_many_databases.path().join("sqlite")).unwrap();
    for index in 0..=subject::MAX_SQLITE_DATABASES {
        fs::write(
            too_many_databases
                .path()
                .join(format!("sqlite/candidate-{index:02}.db")),
            b"",
        )
        .unwrap();
    }
    let error = observe(too_many_databases.path()).expect_err("数据库候选超限必须失败");
    assert_error(
        error,
        ErrorKind::Unavailable,
        "ZED_REMOTE_PROJECT_OBSERVATION_RESOURCE_LIMIT",
    );

    let too_many_rows = TestDirectory::new("too-many-rows");
    write_state(
        too_many_rows.path(),
        &json!({
            "selected-remote-host-id": "host-main",
            "codex-managed-remote-connections": [{
                "hostId": "host-main",
                "hostname": "alice@example.internal"
            }]
        }),
    );
    let db = database(&too_many_rows.path().join("sqlite/state.db"));
    db.execute("CREATE TABLE threads (cwd TEXT NOT NULL)", [])
        .unwrap();
    for index in 0..=subject::MAX_SQLITE_CWDS_PER_DATABASE {
        db.execute(
            "INSERT INTO threads (cwd) VALUES (?1)",
            [format!("/srv/private/repository/{index}")],
        )
        .unwrap();
    }
    drop(db);
    let error = observe(too_many_rows.path()).expect_err("SQLite 行超限必须失败");
    assert_error(
        error,
        ErrorKind::Unavailable,
        "ZED_REMOTE_PROJECT_OBSERVATION_RESOURCE_LIMIT",
    );

    let oversized_value = TestDirectory::new("oversized-cwd");
    write_state(
        oversized_value.path(),
        &json!({
            "selected-remote-host-id": "host-main",
            "codex-managed-remote-connections": [{
                "hostId": "host-main",
                "hostname": "alice@example.internal"
            }]
        }),
    );
    let db = database(&oversized_value.path().join("sqlite/state.db"));
    db.execute("CREATE TABLE threads (cwd TEXT NOT NULL)", [])
        .unwrap();
    db.execute(
        "INSERT INTO threads (cwd) VALUES (?1)",
        [format!(
            "/{}",
            "x".repeat(subject::MAX_REMOTE_COMPONENT_BYTES)
        )],
    )
    .unwrap();
    drop(db);
    let error = observe(oversized_value.path()).expect_err("超长 cwd 必须失败");
    assert_error(
        error,
        ErrorKind::Unavailable,
        "ZED_REMOTE_PROJECT_OBSERVATION_RESOURCE_LIMIT",
    );
}

struct ConstantDigest;

impl subject::IdentityDigest for ConstantDigest {
    fn digest(&self, _payload: &[u8]) -> [u8; 32] {
        [0x5a; 32]
    }
}

#[test]
fn 不同身份摘要碰撞时整体失败() {
    let temp = TestDirectory::new("collision");
    write_state(temp.path(), &base_state());

    let error = subject::observe_zed_remote_projects_at_roots_with_digest(
        temp.path(),
        temp.path(),
        &ZedRemoteProjectObservationCancellation::default(),
        subject::ZedRemoteProjectObservationPolicy::default(),
        &ConstantDigest,
    )
    .expect_err("不同身份同摘要必须失败");
    assert_error(
        error,
        ErrorKind::Internal,
        "ZED_REMOTE_PROJECT_ID_COLLISION",
    );
}

#[test]
fn 预取消和零时限分别返回稳定_cancelled_与_timeout() {
    let temp = TestDirectory::new("interruption");
    write_state(temp.path(), &base_state());

    let cancellation = ZedRemoteProjectObservationCancellation::default();
    cancellation.cancel();
    let error = subject::observe_zed_remote_projects_at_roots(
        temp.path(),
        temp.path(),
        &cancellation,
        subject::ZedRemoteProjectObservationPolicy::default(),
    )
    .expect_err("预取消必须失败");
    assert_error(
        error,
        ErrorKind::Cancelled,
        "ZED_REMOTE_PROJECT_OBSERVATION_CANCELLED",
    );

    let error = subject::observe_zed_remote_projects_at_roots(
        temp.path(),
        temp.path(),
        &ZedRemoteProjectObservationCancellation::default(),
        subject::ZedRemoteProjectObservationPolicy::new(
            Duration::ZERO,
            Duration::from_millis(50),
            1_000,
        ),
    )
    .expect_err("零时限必须失败");
    assert_error(
        error,
        ErrorKind::Timeout,
        "ZED_REMOTE_PROJECT_OBSERVATION_TIMEOUT",
    );
}

#[test]
fn 显式_sqlite_root_只接受现存绝对普通目录() {
    let temp = TestDirectory::new("sqlite-root");
    assert_eq!(
        subject::resolve_sqlite_root(temp.path(), None).unwrap(),
        temp.path()
    );
    assert_eq!(
        subject::resolve_sqlite_root(temp.path(), Some(OsString::from("  "))).unwrap(),
        temp.path()
    );

    for invalid in [
        OsString::from("relative/sqlite"),
        temp.path().join("missing").into_os_string(),
    ] {
        let error = subject::resolve_sqlite_root(temp.path(), Some(invalid))
            .expect_err("非法显式 SQLite root 必须失败");
        assert_error(
            error,
            ErrorKind::Unavailable,
            "ZED_REMOTE_PROJECT_OBSERVATION_INVALID_SQLITE_HOME",
        );
    }
}

#[test]
fn 严格只读观察不改变状态主库_wal_或目录清单() {
    let temp = TestDirectory::new("read-only");
    write_state(temp.path(), &base_state());
    let state_path = temp.path().join(".codex-global-state.json");
    let db_path = temp.path().join("sqlite/state.db");
    let db = database(&db_path);
    db.pragma_update(None, "journal_mode", "WAL").unwrap();
    db.execute_batch(
        "CREATE TABLE threads (id TEXT PRIMARY KEY, cwd TEXT NOT NULL);\n\
         INSERT INTO threads (id, cwd) VALUES ('thread-db', '/srv/private/repository/db-worktree');",
    )
    .unwrap();

    let wal_path = PathBuf::from(format!("{}-wal", db_path.display()));
    let state_before = fs::read(&state_path).unwrap();
    let database_before = fs::read(&db_path).unwrap();
    let wal_before = fs::read(&wal_path).unwrap();
    let directory_before = directory_names(db_path.parent().unwrap());

    let observation = observe(temp.path())
        .expect("WAL 只读观察应成功")
        .expect("应观察到项目");
    assert_eq!(
        observation.sources().coverage(),
        ZedRemoteProjectSourceCoverage::Complete
    );

    assert_eq!(fs::read(&state_path).unwrap(), state_before);
    assert_eq!(fs::read(&db_path).unwrap(), database_before);
    assert_eq!(fs::read(&wal_path).unwrap(), wal_before);
    assert_eq!(directory_names(db_path.parent().unwrap()), directory_before);
    drop(db);
}
