#![cfg(any(target_os = "windows", target_os = "macos"))]

use std::{
    collections::HashMap,
    fs, io,
    mem::size_of,
    path::{Path, PathBuf},
    sync::atomic::{AtomicU64, Ordering},
    time::Duration,
};

use inputcodex_application::{
    ErrorKind, MarkdownGenerationCancellation, MarkdownGenerationPort, MarkdownGenerationRequest,
};
use inputcodex_platform::SystemMarkdownGeneration;
use rusqlite::{Connection, OpenFlags};

pub(crate) use inputcodex_platform::SystemPlatformPaths;

#[allow(dead_code)]
#[path = "../src/local_session_directory_observation.rs"]
mod local_session_directory_observation;
#[allow(dead_code)]
#[path = "../src/markdown_generation.rs"]
mod subject;

static NEXT_TEMP_ID: AtomicU64 = AtomicU64::new(1);

struct TestDirectory {
    path: PathBuf,
}

impl TestDirectory {
    fn new(label: &str) -> Self {
        let sequence = NEXT_TEMP_ID.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "inputcodex-issue-109-{label}-{}-{sequence}",
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

fn request(session_id: &str) -> MarkdownGenerationRequest {
    MarkdownGenerationRequest::new(session_id.to_owned()).expect("测试会话标识应合法")
}

fn generate(
    codex_home: &Path,
    sqlite_root: &Path,
    session_id: &str,
) -> Result<
    Option<inputcodex_domain::SessionMarkdownDocument>,
    inputcodex_application::ApplicationError,
> {
    subject::generate_session_markdown_at_roots(
        codex_home,
        sqlite_root,
        &request(session_id),
        &MarkdownGenerationCancellation::default(),
        subject::MarkdownGenerationPolicy::default(),
    )
}

fn write_rollout(path: &Path, lines: &[&str]) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("创建 rollout 父目录应成功");
    }
    let mut body = lines.join("\n");
    body.push('\n');
    fs::write(path, body).expect("写入合成 rollout 应成功");
}

fn create_threads_database(
    path: &Path,
    session_id: &str,
    title: &str,
    rollout_path: Option<&Path>,
    updated_at_ms: i64,
) {
    let db = database(path);
    db.execute_batch(
        "CREATE TABLE threads (
            id TEXT PRIMARY KEY,
            title TEXT,
            rollout_path TEXT,
            updated_at_ms INTEGER
        );",
    )
    .expect("创建 threads schema 应成功");
    db.execute(
        "INSERT INTO threads VALUES (?1, ?2, ?3, ?4)",
        (
            session_id,
            title,
            rollout_path.map(|value| value.to_string_lossy().into_owned()),
            updated_at_ms,
        ),
    )
    .expect("插入 threads 行应成功");
}

fn assert_error(error: inputcodex_application::ApplicationError, kind: ErrorKind, code: &str) {
    assert_eq!(error.kind(), kind);
    assert_eq!(error.code().as_str(), code);
    let debug = format!("{error:?}");
    assert!(!debug.contains("inputcodex-issue-109"));
    assert!(!debug.contains("private-session"));
    assert!(!debug.contains("rollout"));
}

#[test]
fn 系统生成器保持零字段并实现应用端口() {
    fn assert_port<T: MarkdownGenerationPort>() {}

    assert_eq!(size_of::<SystemMarkdownGeneration>(), 0);
    assert_port::<SystemMarkdownGeneration>();
    assert_eq!(
        format!("{:?}", SystemMarkdownGeneration),
        "SystemMarkdownGeneration"
    );
}

#[test]
fn 正式资源常量精确冻结() {
    assert_eq!(
        local_session_directory_observation::MAX_LOCAL_SESSION_DATABASES,
        32
    );
    assert_eq!(subject::MAX_ROLLOUT_DEPTH, 4);
    assert_eq!(subject::MAX_ROLLOUT_DISCOVERY_ENTRIES, 8192);
    assert_eq!(subject::MAX_ROLLOUT_CANDIDATES, 4096);
    assert_eq!(subject::MAX_ROLLOUT_METADATA_BYTES, 8 * 1024);
    assert_eq!(subject::MAX_ROLLOUT_DISCOVERY_BYTES, 32 * 1024 * 1024);
    assert_eq!(subject::MAX_ROLLOUT_BYTES, 16 * 1024 * 1024);
    assert_eq!(subject::MAX_ROLLOUT_RECORDS, 100_000);
    assert_eq!(inputcodex_domain::MAX_MARKDOWN_MESSAGE_COUNT, 20_000);
    assert_eq!(
        inputcodex_domain::MAX_MARKDOWN_OUTPUT_BYTES,
        16 * 1024 * 1024
    );
    assert_eq!(inputcodex_domain::MAX_MARKDOWN_FILENAME_BYTES, 160);
    assert_eq!(subject::DEFAULT_BUSY_TIMEOUT, Duration::from_millis(50));
    assert_eq!(subject::DEFAULT_PROGRESS_STEPS, 1_000);
    assert_eq!(subject::DEFAULT_GENERATION_TIMEOUT, Duration::from_secs(2));
}

#[test]
fn 无数据库返回_empty_且不创建任何来源() {
    let temp = TestDirectory::new("empty");
    let before = fs::read_dir(temp.path())
        .expect("读取测试目录应成功")
        .count();

    assert!(
        generate(temp.path(), temp.path(), "private-session-empty")
            .expect("无来源应是合法空结果")
            .is_none()
    );
    assert_eq!(
        fs::read_dir(temp.path())
            .expect("再次读取测试目录应成功")
            .count(),
        before
    );
}

#[test]
fn sqlite_目录枚举在读取第二项时立即触发资源上限() {
    let temp = TestDirectory::new("sqlite-entry-limit");
    let rollout = temp.path().join("sessions/limited.jsonl");
    write_rollout(
        &rollout,
        &[
            r#"{"type":"response_item","payload":{"type":"message","role":"user","content":[{"type":"input_text","text":"bounded"}]}}"#,
        ],
    );
    create_threads_database(
        &temp.path().join("sqlite/current.db"),
        "private-session-sqlite-entry-limit",
        "Bounded",
        Some(&rollout),
        1,
    );
    fs::write(temp.path().join("sqlite/noise.txt"), "noise").expect("写入干扰文件应成功");
    let policy =
        subject::MarkdownGenerationPolicy::default().with_discovery_limits(4, 1, 10, 1024, 4096);

    let error = subject::generate_session_markdown_at_roots(
        temp.path(),
        temp.path(),
        &request("private-session-sqlite-entry-limit"),
        &MarkdownGenerationCancellation::default(),
        policy,
    )
    .expect_err("SQLite 目录项必须在无界收集前受限");
    assert_error(
        error,
        ErrorKind::Unavailable,
        "MARKDOWN_GENERATION_RESOURCE_LIMIT",
    );
}

#[test]
fn threads_显式_rollout_只生成批准角色文本和不可联网图片占位() {
    let temp = TestDirectory::new("threads-explicit");
    let rollout = temp.path().join("sessions/2026/explicit.jsonl");
    write_rollout(
        &rollout,
        &[
            r#"{"type":"session_meta","payload":{"id":"private-session-explicit"}}"#,
            r#"{"type":"response_item","timestamp":"2026-01-01T00:30:00+01:00","payload":{"type":"message","role":"user","content":[{"type":"input_text","text":"Hello\r\nworld"},{"type":"input_image","image_url":"data:image/png;base64,AAAA"},{"type":"input_image","image_url":"https://example.invalid/private.png"}]}}"#,
            r#"{"type":"response_item","payload":{"type":"message","role":"developer","content":[{"type":"input_text","text":"internal"}]}}"#,
            r#"{"type":"response_item","payload":{"type":"function_call","name":"secret"}}"#,
            r#"{"type":"response_item","timestamp":"invalid","payload":{"type":"message","role":"assistant","content":[{"type":"output_text","text":"Done"},{"type":"unknown","value":"skip"}]}}"#,
        ],
    );
    create_threads_database(
        &temp.path().join("sqlite/current.db"),
        "private-session-explicit",
        "  发布 / 计划  ",
        Some(&rollout),
        100,
    );

    let document = generate(temp.path(), temp.path(), "private-session-explicit")
        .expect("显式 rollout 应成功")
        .expect("应生成文档");

    assert_eq!(document.suggested_filename(), "session-发布-计划.md");
    assert_eq!(document.message_count(), 2);
    assert!(
        document
            .markdown()
            .starts_with("# 发布 / 计划\n\n### User\n_2025-12-31T23:30:00Z_\n\nHello\nworld")
    );
    assert_eq!(
        document
            .markdown()
            .matches("> Image attachment omitted")
            .count(),
        2
    );
    assert!(document.markdown().contains("### Assistant\n\nDone\n"));
    for forbidden in [
        "developer",
        "internal",
        "secret",
        "data:image",
        "https://",
        "example.invalid",
    ] {
        assert!(!document.markdown().contains(forbidden));
    }
}

#[test]
fn sqlite_标题和_rollout_path_在分配前受文本字节上限约束() {
    let oversized = "x".repeat(subject::MAX_ROLLOUT_METADATA_BYTES + 1);

    let title_temp = TestDirectory::new("oversized-sqlite-title");
    let title_rollout = title_temp.path().join("sessions/title.jsonl");
    write_rollout(
        &title_rollout,
        &[
            r#"{"type":"response_item","payload":{"type":"message","role":"user","content":[{"type":"input_text","text":"bounded"}]}}"#,
        ],
    );
    create_threads_database(
        &title_temp.path().join("sqlite/current.db"),
        "private-session-oversized-title",
        &oversized,
        Some(&title_rollout),
        1,
    );
    let title_error = generate(
        title_temp.path(),
        title_temp.path(),
        "private-session-oversized-title",
    )
    .expect_err("超大 SQLite 标题必须在 String 分配前失败");
    assert_error(
        title_error,
        ErrorKind::Unavailable,
        "MARKDOWN_GENERATION_RESOURCE_LIMIT",
    );

    let path_temp = TestDirectory::new("oversized-sqlite-path");
    create_threads_database(
        &path_temp.path().join("sqlite/current.db"),
        "private-session-oversized-path",
        "Bounded",
        Some(Path::new(&oversized)),
        1,
    );
    let path_error = generate(
        path_temp.path(),
        path_temp.path(),
        "private-session-oversized-path",
    )
    .expect_err("超大 SQLite rollout_path 必须在 PathBuf 分配前失败");
    assert_error(
        path_error,
        ErrorKind::Unavailable,
        "MARKDOWN_GENERATION_RESOURCE_LIMIT",
    );
}

#[test]
fn automation_runs_缺失显式路径时只在固定根发现匹配_rollout() {
    let temp = TestDirectory::new("automation-discovery");
    let rollout = temp.path().join("archived_sessions/2026/07/matched.jsonl");
    write_rollout(
        &rollout,
        &[
            r#"{"type":"session_meta","payload":{"id":"private-session-auto"}}"#,
            r#"{"type":"response_item","payload":{"type":"message","role":"assistant","content":[{"type":"output_text","text":"Automation reply"}]}}"#,
        ],
    );
    let unrelated = temp.path().join("sessions/0000/unrelated.jsonl");
    write_rollout(
        &unrelated,
        &[r#"{"type":"session_meta","payload":{"id":"other-session"}}"#],
    );
    let db = database(&temp.path().join("sqlite/automation.sqlite"));
    db.execute_batch(
        "CREATE TABLE automation_runs (
            thread_id TEXT PRIMARY KEY,
            thread_title TEXT,
            created_at INTEGER,
            updated_at INTEGER
        );",
    )
    .expect("创建 automation_runs schema 应成功");
    db.execute(
        "INSERT INTO automation_runs VALUES (?1, ?2, ?3, ?4)",
        ("private-session-auto", "Automation", 100_i64, 200_i64),
    )
    .expect("插入 automation_runs 行应成功");

    let document = generate(temp.path(), temp.path(), "private-session-auto")
        .expect("发现式 rollout 应成功")
        .expect("应生成文档");
    assert_eq!(document.suggested_filename(), "session-Automation.md");
    assert!(document.markdown().contains("Automation reply"));
}

#[test]
fn 发现到多个同会话_rollout_时拒绝按词法顺序选择旧副本() {
    let temp = TestDirectory::new("duplicate-discovery");
    for (path, body) in [
        ("sessions/2025/old.jsonl", "Old copy"),
        ("archived_sessions/2026/new.jsonl", "New copy"),
    ] {
        write_rollout(
            &temp.path().join(path),
            &[
                r#"{"type":"session_meta","payload":{"id":"private-session-duplicate-discovery"}}"#,
                &format!(
                    r#"{{"type":"response_item","payload":{{"type":"message","role":"assistant","content":[{{"type":"output_text","text":"{body}"}}]}}}}"#
                ),
            ],
        );
    }
    let db = database(&temp.path().join("sqlite/automation.sqlite"));
    db.execute_batch(
        "CREATE TABLE automation_runs (
            thread_id TEXT PRIMARY KEY,
            thread_title TEXT
        );",
    )
    .expect("创建 automation_runs schema 应成功");
    db.execute(
        "INSERT INTO automation_runs VALUES (?1, ?2)",
        ("private-session-duplicate-discovery", "Duplicate"),
    )
    .expect("插入 automation_runs 行应成功");

    let error = generate(
        temp.path(),
        temp.path(),
        "private-session-duplicate-discovery",
    )
    .expect_err("多个匹配 rollout 必须作为权威来源冲突失败");
    assert_error(
        error,
        ErrorKind::Unavailable,
        "MARKDOWN_GENERATION_INVALID_ROLLOUT",
    );
}

#[test]
fn 跨库同会话按更新时间选择权威记录且完全相同时当前库优先() {
    let newer = TestDirectory::new("duplicate-newer");
    let current_rollout = newer.path().join("sessions/current.jsonl");
    let legacy_rollout = newer.path().join("sessions/legacy.jsonl");
    write_rollout(
        &current_rollout,
        &[
            r#"{"type":"response_item","payload":{"type":"message","role":"user","content":[{"type":"input_text","text":"Current older"}]}}"#,
        ],
    );
    write_rollout(
        &legacy_rollout,
        &[
            r#"{"type":"response_item","payload":{"type":"message","role":"user","content":[{"type":"input_text","text":"Legacy newer"}]}}"#,
        ],
    );
    create_threads_database(
        &newer.path().join("sqlite/current.db"),
        "private-session-duplicate",
        "Current",
        Some(&current_rollout),
        100,
    );
    create_threads_database(
        &newer.path().join("state_5.sqlite"),
        "private-session-duplicate",
        "Legacy",
        Some(&legacy_rollout),
        200,
    );
    let selected = generate(newer.path(), newer.path(), "private-session-duplicate")
        .expect("跨库选择应成功")
        .expect("应生成文档");
    assert!(selected.markdown().contains("Legacy newer"));
    assert!(!selected.markdown().contains("Current older"));

    let tied = TestDirectory::new("duplicate-tie");
    let current_rollout = tied.path().join("sessions/current.jsonl");
    let legacy_rollout = tied.path().join("sessions/legacy.jsonl");
    write_rollout(
        &current_rollout,
        &[
            r#"{"type":"response_item","payload":{"type":"message","role":"user","content":[{"type":"input_text","text":"Current tie"}]}}"#,
        ],
    );
    write_rollout(
        &legacy_rollout,
        &[
            r#"{"type":"response_item","payload":{"type":"message","role":"user","content":[{"type":"input_text","text":"Legacy tie"}]}}"#,
        ],
    );
    create_threads_database(
        &tied.path().join("sqlite/current.db"),
        "private-session-tie",
        "Current",
        Some(&current_rollout),
        100,
    );
    create_threads_database(
        &tied.path().join("state_5.sqlite"),
        "private-session-tie",
        "Legacy",
        Some(&legacy_rollout),
        100,
    );
    let selected = generate(tied.path(), tied.path(), "private-session-tie")
        .expect("平局选择应成功")
        .expect("应生成文档");
    assert!(selected.markdown().contains("Current tie"));
    assert!(!selected.markdown().contains("Legacy tie"));
}

#[test]
fn 任一候选数据库失败都阻断较旧记录冒充成功() {
    let temp = TestDirectory::new("candidate-failure");
    let rollout = temp.path().join("sessions/valid.jsonl");
    write_rollout(
        &rollout,
        &[
            r#"{"type":"response_item","payload":{"type":"message","role":"user","content":[{"type":"input_text","text":"Must not escape"}]}}"#,
        ],
    );
    create_threads_database(
        &temp.path().join("sqlite/a-valid.db"),
        "private-session-blocked",
        "Valid",
        Some(&rollout),
        100,
    );
    fs::write(temp.path().join("sqlite/z-broken.db"), b"not sqlite").expect("写入损坏数据库应成功");

    let error = generate(temp.path(), temp.path(), "private-session-blocked")
        .expect_err("任一来源失败必须阻断");
    assert_error(
        error,
        ErrorKind::Unavailable,
        "MARKDOWN_GENERATION_UNAVAILABLE",
    );
}

#[derive(Default)]
struct MemoryFileProbe {
    kinds: HashMap<PathBuf, subject::MarkdownGenerationPathKind>,
}

impl MemoryFileProbe {
    fn with_kind(mut self, path: PathBuf, kind: subject::MarkdownGenerationPathKind) -> Self {
        self.kinds.insert(path, kind);
        self
    }
}

impl subject::MarkdownGenerationFileProbe for MemoryFileProbe {
    fn kind(&self, path: &Path) -> io::Result<subject::MarkdownGenerationPathKind> {
        Ok(self
            .kinds
            .get(path)
            .copied()
            .unwrap_or(subject::MarkdownGenerationPathKind::Missing))
    }

    fn direct_entries(&self, _path: &Path, _max_entries: usize) -> io::Result<Vec<PathBuf>> {
        Ok(Vec::new())
    }
}

#[test]
fn 显式_rollout_拒绝相对路径根越界符号链接和非普通文件() {
    let home = std::env::temp_dir().join("inputcodex-issue-109-memory-home");
    let sessions = home.join("sessions");
    let valid = sessions.join("2026/valid.jsonl");
    let linked = sessions.join("linked.jsonl");
    let too_deep = sessions.join("a/b/c/d/e/deep.jsonl");
    let outside = home
        .parent()
        .expect("临时目录应有父目录")
        .join("outside.jsonl");
    let probe = MemoryFileProbe::default()
        .with_kind(home.clone(), subject::MarkdownGenerationPathKind::Directory)
        .with_kind(
            sessions.clone(),
            subject::MarkdownGenerationPathKind::Directory,
        )
        .with_kind(
            sessions.join("2026"),
            subject::MarkdownGenerationPathKind::Directory,
        )
        .with_kind(valid.clone(), subject::MarkdownGenerationPathKind::File)
        .with_kind(linked.clone(), subject::MarkdownGenerationPathKind::Symlink)
        .with_kind(
            sessions.join("a"),
            subject::MarkdownGenerationPathKind::Directory,
        )
        .with_kind(
            sessions.join("a/b"),
            subject::MarkdownGenerationPathKind::Directory,
        )
        .with_kind(
            sessions.join("a/b/c"),
            subject::MarkdownGenerationPathKind::Directory,
        )
        .with_kind(
            sessions.join("a/b/c/d"),
            subject::MarkdownGenerationPathKind::Directory,
        )
        .with_kind(
            sessions.join("a/b/c/d/e"),
            subject::MarkdownGenerationPathKind::Directory,
        )
        .with_kind(too_deep.clone(), subject::MarkdownGenerationPathKind::File)
        .with_kind(outside.clone(), subject::MarkdownGenerationPathKind::File);

    subject::validate_rollout_path_with_probe(&home, &valid, &probe)
        .expect("固定根内普通文件应通过");
    for path in [
        PathBuf::from("sessions/relative.jsonl"),
        sessions.join("../outside.jsonl"),
        linked,
        too_deep,
        outside,
        sessions.join("missing.jsonl"),
        sessions.clone(),
    ] {
        let error = subject::validate_rollout_path_with_probe(&home, &path, &probe)
            .expect_err("不受控路径必须失败");
        assert_error(
            error,
            ErrorKind::Unavailable,
            "MARKDOWN_GENERATION_INVALID_ROLLOUT",
        );
    }

    let linked_home = std::env::temp_dir().join("inputcodex-issue-109-linked-home");
    let linked_sessions = linked_home.join("sessions");
    let linked_file = linked_sessions.join("linked-home.jsonl");
    let linked_home_probe = MemoryFileProbe::default()
        .with_kind(
            linked_home.clone(),
            subject::MarkdownGenerationPathKind::Symlink,
        )
        .with_kind(
            linked_sessions,
            subject::MarkdownGenerationPathKind::Directory,
        )
        .with_kind(
            linked_file.clone(),
            subject::MarkdownGenerationPathKind::File,
        );
    let error =
        subject::validate_rollout_path_with_probe(&linked_home, &linked_file, &linked_home_probe)
            .expect_err("符号链接 CODEX_HOME 必须失败");
    assert_error(
        error,
        ErrorKind::Unavailable,
        "MARKDOWN_GENERATION_INVALID_ROLLOUT",
    );
}

#[test]
fn 文件身份比较检测同一路径在检查后被替换() {
    let temp = TestDirectory::new("file-identity");
    let path = temp.path().join("sessions/current.jsonl");
    let original = temp.path().join("sessions/original.jsonl");
    write_rollout(&path, &[r#"{"type":"session_meta"}"#]);
    let before = fs::metadata(&path).expect("读取原文件身份应成功");
    fs::rename(&path, &original).expect("保留原文件以避免身份复用应成功");
    write_rollout(&path, &[r#"{"type":"response_item"}"#]);
    let replacement = fs::metadata(&path).expect("读取替换文件身份应成功");
    let moved_original = fs::metadata(&original).expect("读取移动后原文件身份应成功");

    assert!(subject::same_file_identity(&before, &moved_original));
    assert!(!subject::same_file_identity(&before, &replacement));
}

#[test]
fn 显式_rollout_声明其他会话时不得泄露其正文() {
    let temp = TestDirectory::new("mismatched-meta");
    let rollout = temp.path().join("sessions/mismatched.jsonl");
    write_rollout(
        &rollout,
        &[
            r#"{"type":"session_meta","payload":{"id":"other-private-session"}}"#,
            r#"{"type":"response_item","payload":{"type":"message","role":"user","content":[{"type":"input_text","text":"Other private body"}]}}"#,
        ],
    );
    create_threads_database(
        &temp.path().join("sqlite/current.db"),
        "private-session-mismatch",
        "Mismatch",
        Some(&rollout),
        1,
    );

    let error = generate(temp.path(), temp.path(), "private-session-mismatch")
        .expect_err("会话元数据不匹配必须失败");
    assert_error(
        error,
        ErrorKind::Unavailable,
        "MARKDOWN_GENERATION_INVALID_CONTENT",
    );
}

#[test]
fn malformed_json_与错误消息结构明确失败且不返回部分文档() {
    for (label, lines) in [
        (
            "malformed",
            vec![
                r#"{"type":"response_item","payload":{"type":"message","role":"user","content":[{"type":"input_text","text":"partial"}]}}"#,
                "{not-json}",
            ],
        ),
        (
            "structure",
            vec![
                r#"{"type":"response_item","payload":{"type":"message","role":"assistant","content":"not-array"}}"#,
            ],
        ),
    ] {
        let temp = TestDirectory::new(label);
        let rollout = temp.path().join("sessions/invalid.jsonl");
        write_rollout(&rollout, &lines);
        create_threads_database(
            &temp.path().join("sqlite/current.db"),
            "private-session-invalid",
            "Invalid",
            Some(&rollout),
            1,
        );

        let error = generate(temp.path(), temp.path(), "private-session-invalid")
            .expect_err("损坏内容必须失败");
        assert_error(
            error,
            ErrorKind::Unavailable,
            "MARKDOWN_GENERATION_INVALID_CONTENT",
        );
    }
}

#[test]
fn 非_utf8_jsonl_明确归类为损坏内容() {
    let temp = TestDirectory::new("invalid-utf8");
    let rollout = temp.path().join("sessions/invalid-utf8.jsonl");
    fs::create_dir_all(rollout.parent().expect("rollout 应有父目录"))
        .expect("创建 rollout 父目录应成功");
    fs::write(&rollout, [0xff, b'\n']).expect("写入非法 UTF-8 应成功");
    create_threads_database(
        &temp.path().join("sqlite/current.db"),
        "private-session-invalid-utf8",
        "Invalid UTF-8",
        Some(&rollout),
        1,
    );

    let error = generate(temp.path(), temp.path(), "private-session-invalid-utf8")
        .expect_err("非法 UTF-8 必须失败");
    assert_error(
        error,
        ErrorKind::Unavailable,
        "MARKDOWN_GENERATION_INVALID_CONTENT",
    );
}

#[test]
fn 合法但没有用户或助手文本时返回_empty() {
    let temp = TestDirectory::new("no-exportable");
    let rollout = temp.path().join("sessions/empty.jsonl");
    write_rollout(
        &rollout,
        &[
            r#"{"type":"session_meta","payload":{"id":"private-session-no-text"}}"#,
            r#"{"type":"response_item","payload":{"type":"message","role":"developer","content":[{"type":"input_text","text":"internal"}]}}"#,
            r#"{"type":"response_item","payload":{"type":"message","role":"assistant","content":[{"type":"unknown","text":"skip"}]}}"#,
        ],
    );
    create_threads_database(
        &temp.path().join("sqlite/current.db"),
        "private-session-no-text",
        "Empty",
        Some(&rollout),
        1,
    );

    assert!(
        generate(temp.path(), temp.path(), "private-session-no-text")
            .expect("合法零消息应成功")
            .is_none()
    );
}

#[test]
fn rfc3339_时间戳不依赖本机时区并覆盖日期边界() {
    for (input, expected) in [
        ("2026-01-01T00:30:00+01:00", "2025-12-31T23:30:00Z"),
        ("2025-12-31T23:30:00-01:00", "2026-01-01T00:30:00Z"),
        ("2024-03-01T00:15:00+01:00", "2024-02-29T23:15:00Z"),
        ("2024-02-29T23:15:00-02:00", "2024-03-01T01:15:00Z"),
        ("2026-07-30T11:14:29.120000000Z", "2026-07-30T11:14:29.12Z"),
        ("2017-01-01T00:59:60+01:00", "2016-12-31T23:59:60Z"),
    ] {
        assert_eq!(
            subject::normalize_rfc3339_utc(input).as_deref(),
            Some(expected),
            "UTC 规范化失败：{input}"
        );
    }
    for invalid in [
        "2026-02-29T00:00:00Z",
        "2026-07-30 11:14:29Z",
        "2026-07-30T11:14:29-00:00",
        "2026-07-30T11:14:60Z",
        "2026-07-30T11:14:61Z",
        "not-a-time",
    ] {
        assert_eq!(subject::normalize_rfc3339_utc(invalid), None);
    }
}

#[test]
fn 发现阶段深度条目候选和读取字节均受策略限制() {
    let cases = [
        ("depth", 0, 10, 10, 1024, 4096),
        ("entries", 4, 1, 10, 1024, 4096),
        ("candidates", 4, 10, 1, 1024, 4096),
        ("metadata", 4, 10, 10, 8, 4096),
        ("discovery-bytes", 4, 10, 10, 1024, 20),
    ];

    for (label, depth, entries, candidates, metadata, discovery_bytes) in cases {
        let temp = TestDirectory::new(label);
        let first = temp.path().join("sessions/a/first.jsonl");
        let second = temp.path().join("sessions/b/second.jsonl");
        write_rollout(
            &first,
            &[r#"{"type":"session_meta","payload":{"id":"other-one"}}"#],
        );
        write_rollout(
            &second,
            &[r#"{"type":"session_meta","payload":{"id":"private-session-limits"}}"#],
        );
        let db = database(&temp.path().join("sqlite/current.db"));
        db.execute_batch("CREATE TABLE threads (id TEXT PRIMARY KEY, title TEXT);")
            .expect("创建最小 threads schema 应成功");
        db.execute(
            "INSERT INTO threads VALUES (?1, ?2)",
            ("private-session-limits", "Limits"),
        )
        .expect("插入测试行应成功");

        let policy = subject::MarkdownGenerationPolicy::default().with_discovery_limits(
            depth,
            entries,
            candidates,
            metadata,
            discovery_bytes,
        );
        let error = subject::generate_session_markdown_at_roots(
            temp.path(),
            temp.path(),
            &request("private-session-limits"),
            &MarkdownGenerationCancellation::default(),
            policy,
        )
        .expect_err("发现资源上限必须阻断");
        assert_error(
            error,
            ErrorKind::Unavailable,
            "MARKDOWN_GENERATION_RESOURCE_LIMIT",
        );
    }
}

#[test]
fn 选中_rollout_字节记录和消息数量均受策略限制() {
    for (label, max_bytes, max_records, max_messages) in [
        ("rollout-bytes", 64, 100, 100),
        ("rollout-records", 4096, 1, 100),
        ("messages", 4096, 100, 1),
    ] {
        let temp = TestDirectory::new(label);
        let rollout = temp.path().join("sessions/limited.jsonl");
        write_rollout(
            &rollout,
            &[
                r#"{"type":"response_item","payload":{"type":"message","role":"user","content":[{"type":"input_text","text":"one"}]}}"#,
                r#"{"type":"response_item","payload":{"type":"message","role":"assistant","content":[{"type":"output_text","text":"two"}]}}"#,
            ],
        );
        create_threads_database(
            &temp.path().join("sqlite/current.db"),
            "private-session-rollout-limits",
            "Limits",
            Some(&rollout),
            1,
        );
        let policy = subject::MarkdownGenerationPolicy::default().with_rollout_limits(
            max_bytes,
            max_records,
            max_messages,
        );

        let error = subject::generate_session_markdown_at_roots(
            temp.path(),
            temp.path(),
            &request("private-session-rollout-limits"),
            &MarkdownGenerationCancellation::default(),
            policy,
        )
        .expect_err("rollout 资源上限必须阻断");
        assert_error(
            error,
            ErrorKind::Unavailable,
            "MARKDOWN_GENERATION_RESOURCE_LIMIT",
        );
    }
}

#[test]
fn 只读生成保持_wal_主库日志和目录内容不变() {
    let temp = TestDirectory::new("readonly-wal");
    let rollout = temp.path().join("sessions/readonly.jsonl");
    write_rollout(
        &rollout,
        &[
            r#"{"type":"response_item","payload":{"type":"message","role":"user","content":[{"type":"input_text","text":"Read only"}]}}"#,
        ],
    );
    let db_path = temp.path().join("sqlite/readonly.db");
    let db = database(&db_path);
    db.execute_batch(
        "PRAGMA journal_mode = WAL;
         PRAGMA wal_autocheckpoint = 0;
         CREATE TABLE threads (
             id TEXT PRIMARY KEY,
             title TEXT,
             rollout_path TEXT,
             updated_at_ms INTEGER
         );",
    )
    .expect("创建 WAL schema 应成功");
    db.execute(
        "INSERT INTO threads VALUES (?1, ?2, ?3, ?4)",
        (
            "private-session-readonly",
            "Readonly",
            rollout.to_string_lossy().into_owned(),
            1_i64,
        ),
    )
    .expect("插入 WAL 行应成功");
    let wal_path = db_path.with_extension("db-wal");
    let shm_path = db_path.with_extension("db-shm");
    assert!(wal_path.is_file());
    assert!(shm_path.is_file());

    let before_main = fs::read(&db_path).expect("读取主库应成功");
    let before_wal = fs::read(&wal_path).expect("读取 WAL 应成功");
    let before_shm_len = fs::metadata(&shm_path)
        .expect("读取 SHM 元数据应成功")
        .len();
    let mut before_names = fs::read_dir(db_path.parent().expect("数据库应有父目录"))
        .expect("读取目录应成功")
        .map(|entry| entry.expect("读取目录项应成功").file_name())
        .collect::<Vec<_>>();
    before_names.sort();

    let document = generate(temp.path(), temp.path(), "private-session-readonly")
        .expect("只读生成应成功")
        .expect("应生成文档");
    assert!(document.markdown().contains("Read only"));

    let mut after_names = fs::read_dir(db_path.parent().expect("数据库应有父目录"))
        .expect("再次读取目录应成功")
        .map(|entry| entry.expect("读取目录项应成功").file_name())
        .collect::<Vec<_>>();
    after_names.sort();
    assert_eq!(fs::read(&db_path).expect("重读主库应成功"), before_main);
    assert_eq!(fs::read(&wal_path).expect("重读 WAL 应成功"), before_wal);
    assert_eq!(
        fs::metadata(&shm_path).expect("重读 SHM 应成功").len(),
        before_shm_len
    );
    assert_eq!(after_names, before_names);
    drop(db);
}

#[test]
fn 取消和整体_deadline_返回稳定脱敏错误() {
    let temp = TestDirectory::new("interrupt");
    let rollout = temp.path().join("sessions/interrupt.jsonl");
    write_rollout(
        &rollout,
        &[
            r#"{"type":"response_item","payload":{"type":"message","role":"user","content":[{"type":"input_text","text":"Never returned"}]}}"#,
        ],
    );
    create_threads_database(
        &temp.path().join("sqlite/current.db"),
        "private-session-interrupt",
        "Interrupt",
        Some(&rollout),
        1,
    );

    let cancellation = MarkdownGenerationCancellation::default();
    cancellation.cancel();
    let cancelled = subject::generate_session_markdown_at_roots(
        temp.path(),
        temp.path(),
        &request("private-session-interrupt"),
        &cancellation,
        subject::MarkdownGenerationPolicy::default(),
    )
    .expect_err("预取消必须失败");
    assert_error(
        cancelled,
        ErrorKind::Cancelled,
        "MARKDOWN_GENERATION_CANCELLED",
    );

    let timeout = subject::generate_session_markdown_at_roots(
        temp.path(),
        temp.path(),
        &request("private-session-interrupt"),
        &MarkdownGenerationCancellation::default(),
        subject::MarkdownGenerationPolicy::new(Duration::ZERO, Duration::from_millis(1), 1),
    )
    .expect_err("零时限必须超时");
    assert_error(timeout, ErrorKind::Timeout, "MARKDOWN_GENERATION_TIMEOUT");
}
