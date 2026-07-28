use std::{
    collections::BTreeSet,
    fs,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

use inputcodex_parity::{
    ValidationCode, parse_source_index, validate_feature_repository, validate_repository,
    validate_source_index,
};

const RELEASE_TAG: &str = "v1.2.43";
const RELEASE_COMMIT: &str = "5036ff056b5c629f19356396b17d6eeb70da664c";
const PREVIOUS_RELEASE_TAG: &str = "v1.2.42";
const PREVIOUS_RELEASE_COMMIT: &str = "657cd33e009ad02515d30db6492cd4e669b06318";
const RE_AUDIT_ISSUE_URL: &str = "https://github.com/nonononull/inputcodex/issues/65";

const VALID_SOURCE_INDEX: &str = r#"
schema_version: inputcodex.source-index.v1
release:
  tag: v1.2.43
  tag_commit: 5036ff056b5c629f19356396b17d6eeb70da664c
sources:
  - id: tauri-command:load_overview
    kind: tauri-command
    evidence:
      path: upstream/CodexPlusPlus/apps/codex-plus-manager/src-tauri/src/commands.rs
      symbol: load_overview
    platforms: [windows, macos]
    side_effects: [filesystem-read]
    disposition:
      type: feature
      feature_id: feature.foundation-platform.application-overview
"#;

fn known_feature_ids() -> BTreeSet<String> {
    BTreeSet::from(["feature.foundation-platform.application-overview".to_owned()])
}

fn repository_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|path| path.parent())
        .expect("parity crate 应位于仓库 crates 目录")
        .to_path_buf()
}

fn read_repository_text(relative_path: &str) -> String {
    fs::read_to_string(repository_root().join(relative_path))
        .unwrap_or_else(|error| panic!("应能读取 {relative_path}: {error}"))
}

fn assert_repository_text_contains(relative_path: &str, expected: &[&str]) {
    let text = read_repository_text(relative_path);
    for value in expected {
        assert!(
            text.contains(value),
            "{relative_path} 应包含目录重新审计证据：{value}"
        );
    }
}

fn yaml_list_item_block<'a>(text: &'a str, id: &str) -> &'a str {
    let marker = format!("  - id: {id}");
    let start = text
        .find(&marker)
        .unwrap_or_else(|| panic!("YAML 应包含条目：{id}"));
    let tail = &text[start..];
    let end = tail[marker.len()..]
        .find("\n  - id: ")
        .map_or(tail.len(), |offset| marker.len() + offset);
    &tail[..end]
}

struct FeatureRepositoryFixture {
    root: PathBuf,
}

struct SourceLockState<'a> {
    snapshot_tag: &'a str,
    snapshot_commit: &'a str,
    catalog_tag: &'a str,
    catalog_commit: &'a str,
    status: &'a str,
    stale_reason: Option<&'a str>,
    re_audit_issue_ref: Option<&'a str>,
}

impl FeatureRepositoryFixture {
    fn new() -> Self {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("系统时间必须晚于 Unix epoch")
            .as_nanos();
        let root = repository_root().join("target").join(format!(
            "inputcodex-release-audit-{}-{nonce}",
            std::process::id()
        ));

        fs::create_dir_all(&root).expect("应能创建临时功能目录夹具");
        copy_tree(
            &repository_root().join("parity/features"),
            &root.join("parity/features"),
        );
        copy_tree(
            &repository_root().join("upstream/CodexPlusPlus"),
            &root.join("upstream/CodexPlusPlus"),
        );

        Self { root }
    }

    fn root(&self) -> &Path {
        &self.root
    }

    fn write_source_lock(&self, state: SourceLockState<'_>) {
        let stale_reason = state
            .stale_reason
            .map_or_else(|| "null".to_owned(), |value| format!("\"{value}\""));
        let re_audit_issue_ref = state
            .re_audit_issue_ref
            .map_or_else(|| "null".to_owned(), |value| format!("\"{value}\""));
        let source_lock = format!(
            r#"{{
  "snapshot": {{
    "release_tag": "{}",
    "commit": "{}"
  }},
  "release_audit": {{
    "schema_version": "inputcodex.release-audit.v1",
    "catalog_release": {{
      "tag": "{}",
      "commit": "{}"
    }},
    "status": "{}",
    "stale_reason": {},
    "re_audit_issue_ref": {}
  }}
}}"#,
            state.snapshot_tag,
            state.snapshot_commit,
            state.catalog_tag,
            state.catalog_commit,
            state.status,
            stale_reason,
            re_audit_issue_ref,
        );

        let source_lock_path = self.root.join("upstream/source-lock.json");
        fs::create_dir_all(
            source_lock_path
                .parent()
                .expect("source-lock 必须位于上游目录"),
        )
        .expect("应能创建 source-lock 父目录");
        fs::write(source_lock_path, source_lock).expect("应能写入临时 source-lock");
    }

    fn write_catalog_release(&self, tag: &str, commit: &str) {
        let feature_directory = self.root.join("parity/features");
        for entry in fs::read_dir(feature_directory).expect("应能枚举临时功能目录") {
            let path = entry.expect("应能读取临时功能目录项").path();
            if path.extension().and_then(|extension| extension.to_str()) != Some("yml") {
                continue;
            }

            let text = fs::read_to_string(&path).expect("应能读取临时功能目录文件");
            let updated = text
                .replace(&format!("  tag: {RELEASE_TAG}"), &format!("  tag: {tag}"))
                .replace(
                    &format!("  tag: {PREVIOUS_RELEASE_TAG}"),
                    &format!("  tag: {tag}"),
                )
                .replace(
                    &format!("  tag_commit: {RELEASE_COMMIT}"),
                    &format!("  tag_commit: {commit}"),
                )
                .replace(
                    &format!("  tag_commit: {PREVIOUS_RELEASE_COMMIT}"),
                    &format!("  tag_commit: {commit}"),
                );
            fs::write(path, updated).expect("应能更新临时功能目录 Release");
        }
    }
}

impl Drop for FeatureRepositoryFixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

fn copy_tree(source: &Path, destination: &Path) {
    fs::create_dir_all(destination).expect("应能创建临时目录");

    for entry in fs::read_dir(source).expect("应能读取源目录") {
        let entry = entry.expect("应能读取源目录项");
        let destination_path = destination.join(entry.file_name());
        let file_type = entry.file_type().expect("应能读取源目录项类型");

        if file_type.is_dir() {
            copy_tree(&entry.path(), &destination_path);
        } else if file_type.is_file() {
            fs::copy(entry.path(), destination_path).expect("应能复制临时夹具文件");
        }
    }
}

#[test]
fn 合法_source_index_可解析并通过引用验证() {
    let source_index = parse_source_index(VALID_SOURCE_INDEX).expect("合法 source-index 应可解析");

    assert_eq!(source_index.sources().len(), 1);
    assert!(
        validate_source_index(
            &source_index,
            &known_feature_ids(),
            RELEASE_TAG,
            RELEASE_COMMIT,
        )
        .is_empty()
    );
}

#[test]
fn 未映射的上游入口被拒绝() {
    let invalid = VALID_SOURCE_INDEX.replace(
        "    disposition:\n      type: feature\n      feature_id: feature.foundation-platform.application-overview\n",
        "",
    );
    let source_index = parse_source_index(&invalid).expect("缺少映射仍应可解析以报告覆盖缺口");

    assert!(
        validate_source_index(
            &source_index,
            &known_feature_ids(),
            RELEASE_TAG,
            RELEASE_COMMIT,
        )
        .iter()
        .any(|issue| issue.code() == ValidationCode::UnmappedSourceEntry)
    );
}

#[test]
fn 重复_source_id_被拒绝() {
    let entry = VALID_SOURCE_INDEX
        .split_once("sources:\n")
        .expect("测试 source-index 必须包含 sources")
        .1;
    let invalid = format!("{VALID_SOURCE_INDEX}{entry}");
    let source_index = parse_source_index(&invalid).expect("重复入口仍应可解析");

    assert!(
        validate_source_index(
            &source_index,
            &known_feature_ids(),
            RELEASE_TAG,
            RELEASE_COMMIT,
        )
        .iter()
        .any(|issue| issue.code() == ValidationCode::DuplicateSourceId)
    );
}

#[test]
fn source_index_release_必须与锁定版本一致() {
    let invalid = VALID_SOURCE_INDEX.replace(
        &format!("tag: {RELEASE_TAG}"),
        &format!("tag: {PREVIOUS_RELEASE_TAG}"),
    );
    let source_index = parse_source_index(&invalid).expect("结构仍应可解析");

    assert!(
        validate_source_index(
            &source_index,
            &known_feature_ids(),
            RELEASE_TAG,
            RELEASE_COMMIT,
        )
        .iter()
        .any(|issue| issue.code() == ValidationCode::ReleaseMismatch)
    );
}

#[test]
fn source_index_悬空_feature_引用被拒绝() {
    let source_index = parse_source_index(VALID_SOURCE_INDEX).expect("合法 source-index 应可解析");

    assert!(
        validate_source_index(&source_index, &BTreeSet::new(), RELEASE_TAG, RELEASE_COMMIT,)
            .iter()
            .any(|issue| issue.code() == ValidationCode::DanglingFeatureReference)
    );
}

#[test]
fn source_index_证据路径必须位于锁定上游快照() {
    let invalid = VALID_SOURCE_INDEX.replace(
        "upstream/CodexPlusPlus/apps/codex-plus-manager/src-tauri/src/commands.rs",
        "../CodexPlusPlus/commands.rs",
    );
    let source_index = parse_source_index(&invalid).expect("结构仍应可解析");

    assert!(
        validate_source_index(
            &source_index,
            &known_feature_ids(),
            RELEASE_TAG,
            RELEASE_COMMIT,
        )
        .iter()
        .any(|issue| issue.code() == ValidationCode::InvalidEvidencePath)
    );
}

#[test]
fn release_audit_显式解耦快照与功能目录审计基线() {
    let fixture = FeatureRepositoryFixture::new();

    fixture.write_source_lock(SourceLockState {
        snapshot_tag: RELEASE_TAG,
        snapshot_commit: RELEASE_COMMIT,
        catalog_tag: RELEASE_TAG,
        catalog_commit: RELEASE_COMMIT,
        status: "current",
        stale_reason: None,
        re_audit_issue_ref: None,
    });
    let summary =
        validate_feature_repository(fixture.root()).expect("current 审计基线必须通过功能目录验证");
    assert!(!summary.requires_reaudit());

    fixture.write_catalog_release(PREVIOUS_RELEASE_TAG, PREVIOUS_RELEASE_COMMIT);
    fixture.write_source_lock(SourceLockState {
        snapshot_tag: RELEASE_TAG,
        snapshot_commit: RELEASE_COMMIT,
        catalog_tag: PREVIOUS_RELEASE_TAG,
        catalog_commit: PREVIOUS_RELEASE_COMMIT,
        status: "stale-re-audit-required",
        stale_reason: Some("上游 v1.2.43 已缓存，功能目录仍为 v1.2.42"),
        re_audit_issue_ref: Some(RE_AUDIT_ISSUE_URL),
    });
    let summary = validate_feature_repository(fixture.root())
        .expect("显式 stale 审计基线必须允许同步与目录复审继续进行");
    assert!(summary.requires_reaudit());

    fixture.write_catalog_release(RELEASE_TAG, RELEASE_COMMIT);
    fixture.write_source_lock(SourceLockState {
        snapshot_tag: RELEASE_TAG,
        snapshot_commit: RELEASE_COMMIT,
        catalog_tag: RELEASE_TAG,
        catalog_commit: RELEASE_COMMIT,
        status: "stale-re-audit-required",
        stale_reason: Some("快照与目录版本相同却被标记为 stale"),
        re_audit_issue_ref: Some(RE_AUDIT_ISSUE_URL),
    });
    let error = validate_feature_repository(fixture.root())
        .expect_err("stale 审计状态必须对应不同的快照与目录版本");
    assert!(
        error
            .issues()
            .iter()
            .any(|issue| issue.code() == ValidationCode::ReleaseMismatch)
    );

    fixture.write_catalog_release(PREVIOUS_RELEASE_TAG, PREVIOUS_RELEASE_COMMIT);
    fixture.write_source_lock(SourceLockState {
        snapshot_tag: RELEASE_TAG,
        snapshot_commit: RELEASE_COMMIT,
        catalog_tag: PREVIOUS_RELEASE_TAG,
        catalog_commit: PREVIOUS_RELEASE_COMMIT,
        status: "stale-re-audit-required",
        stale_reason: None,
        re_audit_issue_ref: Some(RE_AUDIT_ISSUE_URL),
    });
    let error = validate_feature_repository(fixture.root())
        .expect_err("stale 审计状态必须说明重新审计根因");
    assert!(
        error
            .issues()
            .iter()
            .any(|issue| issue.code() == ValidationCode::ReleaseMismatch)
    );

    fixture.write_source_lock(SourceLockState {
        snapshot_tag: RELEASE_TAG,
        snapshot_commit: RELEASE_COMMIT,
        catalog_tag: PREVIOUS_RELEASE_TAG,
        catalog_commit: PREVIOUS_RELEASE_COMMIT,
        status: "stale-re-audit-required",
        stale_reason: Some("上游 v1.2.42 已缓存，功能目录尚未完成复审"),
        re_audit_issue_ref: Some("https://github.com/nonononull/inputcodex/pull/34"),
    });
    let error = validate_feature_repository(fixture.root())
        .expect_err("stale 审计状态必须关联 inputcodex 的重新审计 Issue");
    assert!(
        error
            .issues()
            .iter()
            .any(|issue| issue.code() == ValidationCode::ReleaseMismatch)
    );
}

#[test]
fn 仓库v1_2_43目录重新审计证据保持固定() {
    validate_repository(&repository_root()).expect("v1.2.43 功能目录证据应通过完整验证");

    for relative_path in [
        "parity/features/foundation-platform.yml",
        "parity/features/plugin-script.yml",
        "parity/features/provider-network.yml",
        "parity/features/remote-install.yml",
        "parity/features/session-data.yml",
        "parity/features/source-index.yml",
    ] {
        assert_repository_text_contains(relative_path, &[RELEASE_TAG, RELEASE_COMMIT]);
        assert!(
            !read_repository_text(relative_path).contains(PREVIOUS_RELEASE_TAG),
            "{relative_path} 不得保留 v1.2.42 Release 元数据"
        );
    }

    for relative_path in [
        "parity/contracts/foundation-platform.yml",
        "parity/contracts/plugin-script.yml",
        "parity/contracts/provider-network.yml",
        "parity/contracts/remote-install.yml",
        "parity/contracts/session-data.yml",
        "parity/README.md",
    ] {
        assert_repository_text_contains(relative_path, &[RELEASE_TAG]);
        assert!(
            !read_repository_text(relative_path).contains(PREVIOUS_RELEASE_TAG),
            "{relative_path} 不得保留 v1.2.42 合同描述"
        );
    }
}

#[test]
fn 仓库v1_2_43受影响行为证据被固定() {
    assert_repository_text_contains(
        "parity/features/foundation-platform.yml",
        &[
            "OpenAI.ChatGPT-Desktop",
            "remote-debugging-port",
            "等待目标进程退出",
            "expires_at",
            "issue:65",
        ],
    );
    assert_repository_text_contains(
        "parity/contracts/foundation-platform.yml",
        &[
            "OpenAI.ChatGPT-Desktop",
            "debug port",
            "等待旧进程退出",
            "不得加载、下载或展示 sponsor",
        ],
    );

    assert_repository_text_contains(
        "parity/features/session-data.yml",
        &[
            "CODEX_SQLITE_HOME",
            "grouped-undo-token",
            "local_thread_catalog",
            "sqliteCatalogRowsInserted",
            "issue:65",
        ],
    );
    assert_repository_text_contains(
        "parity/contracts/session-data.yml",
        &[
            "CODEX_SQLITE_HOME",
            "grouped-undo-token",
            "LOCAL_SESSION_UNDO_PREFLIGHT_FAILED",
            "LOCAL_SESSION_UNDO_PATH_REJECTED",
            "sqlite_catalog_rows_inserted",
            "local_thread_catalog",
            "observation_sequence",
        ],
    );
    assert_repository_text_contains(
        "parity/fixtures/feature.session-data.local-session-management/baseline.yml",
        &[
            "database_count: 2",
            "all_databases_checked: true",
            "allowed_paths_only: true",
            "undo_window_retained: true",
        ],
    );

    assert_repository_text_contains(
        "parity/features/plugin-script.yml",
        &[
            "companion",
            "renderer-inject.js",
            "runtime status",
            "status 与 error",
            "issue:65",
        ],
    );
    assert_repository_text_contains(
        "parity/contracts/plugin-script.yml",
        &[
            "data:image/png;base64",
            "DREAM_SKIN_COMPANION_INVALID",
            "companion 显示仍依赖 renderer 注入",
            "运行状态仍来自 renderer 注入链",
            "不得伪造运行成功",
        ],
    );
    assert_repository_text_contains(
        "parity/fixtures/feature.plugin-script.dream-skin-library/baseline.yml",
        &[
            "data_url: data:image/webp;base64,UklGRg==",
            "width: 96",
            "side: right",
        ],
    );

    assert_repository_text_contains(
        "parity/fixtures/feature.session-data.provider-metadata-maintenance/baseline.yml",
        &[
            "sqlite_catalog_rows_inserted: 1",
            "display_title: Thread One",
            "source_created_at: 100.0",
            "source_updated_at: 200.0",
            "initial_build_complete: true",
            "observation_sequence: 1",
        ],
    );
    assert_repository_text_contains(
        "parity/fixtures/feature.session-data.provider-metadata-maintenance/manifest.yml",
        &["SQLite catalog", "同步状态水位"],
    );
}

#[test]
fn gate5_platform_paths_实现合同固定为已实现且无新增副作用() {
    let feature_text = read_repository_text("parity/features/foundation-platform.yml");
    let feature = yaml_list_item_block(&feature_text, "feature.foundation-platform.platform-paths");
    for expected in ["status: implemented", "- issue:74", "- issue:75"] {
        assert!(
            feature.contains(expected),
            "平台路径功能条目应包含：{expected}"
        );
    }

    let contract_text = read_repository_text("parity/contracts/foundation-platform.yml");
    let contract = yaml_list_item_block(
        &contract_text,
        "contract.feature.foundation-platform.platform-paths.baseline",
    );
    for expected in [
        "- filesystem-read",
        "- process-read",
        "PLATFORM_PATHS_UNSUPPORTED",
        "EXPLICIT_CODEX_PATH_INVALID",
        "CODEX_HOME_INVALID",
        "USER_HOME_UNAVAILABLE",
        "INPUTCODEX_STATE_ROOT_UNAVAILABLE",
        "PLATFORM_PATHS_FAILED",
        "Ready + installation=None",
    ] {
        assert!(
            contract.contains(expected),
            "平台路径合同应包含：{expected}"
        );
    }
    for forbidden in [
        "filesystem-write",
        "network-read",
        "network-write",
        "advertising",
        "remote-recommendation",
    ] {
        assert!(
            !contract.contains(forbidden),
            "平台路径合同禁止副作用：{forbidden}"
        );
    }

    let source_text = read_repository_text("parity/features/source-index.yml");
    for source_id in [
        "core-module:app_paths",
        "core-module:codex_home",
        "core-module:paths",
    ] {
        let source = yaml_list_item_block(&source_text, source_id);
        assert!(
            source.contains("side_effects: [filesystem-read, process-read]"),
            "{source_id} 应固定文件系统与进程环境读取"
        );
        for forbidden in [
            "filesystem-write",
            "network-read",
            "network-write",
            "injection",
        ] {
            assert!(
                !source.contains(forbidden),
                "{source_id} 禁止副作用：{forbidden}"
            );
        }
    }

    assert_repository_text_contains(
        "parity/README.md",
        &["Issue `#75`", "Ready + installation=None"],
    );
}

#[test]
fn gate5_application_overview_实现合同固定为只读事实且不伪造实时状态() {
    let feature_text = read_repository_text("parity/features/foundation-platform.yml");
    let overview = yaml_list_item_block(
        &feature_text,
        "feature.foundation-platform.application-overview",
    );
    for expected in [
        "status: implemented",
        "symbol: codex_app_version",
        "- tauri-command:load_overview",
        "LiveProcessState::NotObserved",
        "- issue:77",
        "- issue:78",
    ] {
        assert!(
            overview.contains(expected),
            "应用概览功能条目应包含：{expected}"
        );
    }
    assert!(
        !overview.contains("core-module:status"),
        "历史状态入口不得继续归属于应用概览"
    );

    let lifecycle = yaml_list_item_block(
        &feature_text,
        "feature.foundation-platform.application-lifecycle",
    );
    assert!(
        lifecycle.contains("core-module:status"),
        "历史状态入口应归入应用生命周期"
    );

    let contract_text = read_repository_text("parity/contracts/foundation-platform.yml");
    let contract = yaml_list_item_block(
        &contract_text,
        "contract.feature.foundation-platform.application-overview.baseline",
    );
    for expected in [
        "Ready(Installed Known)",
        "Ready(Installed Unknown)",
        "Ready(NotInstalled)",
        "InstalledVersion::Unknown",
        "LiveProcessState::NotObserved",
        "APPLICATION_OVERVIEW_UNSUPPORTED",
        "EXPLICIT_CODEX_PATH_INVALID",
        "APPLICATION_OVERVIEW_DISCOVERY_FAILED",
        "APPLICATION_OVERVIEW_TIME_UNAVAILABLE",
        "APPLICATION_OVERVIEW_BUILD_VERSION_INVALID",
        "256 bytes",
        "65536 bytes",
    ] {
        assert!(
            contract.contains(expected),
            "应用概览合同应包含：{expected}"
        );
    }
    for forbidden in [
        "filesystem-write",
        "process-read",
        "network-read",
        "network-write",
        "latest-status.json",
        "LaunchHistoryRecord",
        "advertising",
        "remote-recommendation",
    ] {
        assert!(
            !contract.contains(forbidden),
            "应用概览合同禁止能力或隐藏依赖：{forbidden}"
        );
    }

    let source_text = read_repository_text("parity/features/source-index.yml");
    let status_source = yaml_list_item_block(&source_text, "core-module:status");
    assert!(
        status_source.contains("side_effects: [filesystem-read, filesystem-write]"),
        "历史状态源必须记录真实文件读写副作用"
    );
    assert!(
        status_source.contains("feature.foundation-platform.application-lifecycle"),
        "历史状态源必须归入应用生命周期"
    );

    assert_repository_text_contains(
        "parity/README.md",
        &[
            "Issue `#78`",
            "Ready(Installed Known)",
            "LiveProcessState::NotObserved",
        ],
    );
}

#[test]
fn gate5_version_startup_实现合同固定版本来源与无副作用启动意图() {
    let feature_text = read_repository_text("parity/features/foundation-platform.yml");
    let feature = yaml_list_item_block(
        &feature_text,
        "feature.foundation-platform.version-and-startup",
    );
    for expected in [
        "status: implemented",
        "CARGO_PKG_VERSION",
        "StartupIntent::Default",
        "StartupIntent::ShowUpdate",
        "- issue:80",
        "- issue:81",
    ] {
        assert!(
            feature.contains(expected),
            "版本与启动功能条目应包含：{expected}"
        );
    }

    let contract_text = read_repository_text("parity/contracts/foundation-platform.yml");
    let contract = yaml_list_item_block(
        &contract_text,
        "contract.feature.foundation-platform.version-and-startup.baseline",
    );
    for expected in [
        "Ready(VersionStartupSnapshot)",
        "StartupIntent::Default",
        "StartupIntent::ShowUpdate",
        "CARGO_PKG_VERSION",
        "--show-update",
        "INPUTCODEX_SHOW_UPDATE",
        "INVALID_STARTUP_OPTION",
        "VERSION_AND_STARTUP_UNSUPPORTED",
        "VERSION_AND_STARTUP_BUILD_VERSION_INVALID",
        "本功能不会产生 LoadCompletion::Empty",
    ] {
        assert!(
            contract.contains(expected),
            "版本与启动合同应包含：{expected}"
        );
    }
    assert!(
        !contract.contains("明确空结果"),
        "版本与启动合同不得把 Empty 描述为合法输出"
    );
    for forbidden in [
        "filesystem-read",
        "filesystem-write",
        "network-read",
        "network-write",
        "process-control",
        "advertising",
        "remote-recommendation",
        "CODEX_PLUS_SHOW_UPDATE",
    ] {
        assert!(
            !contract.contains(forbidden),
            "版本与启动合同禁止能力或旧变量：{forbidden}"
        );
    }

    let source_text = read_repository_text("parity/features/source-index.yml");
    for source_id in [
        "core-module:version",
        "tauri-command:backend_version",
        "tauri-command:startup_options",
    ] {
        let source = yaml_list_item_block(&source_text, source_id);
        assert!(
            source.contains("side_effects: [process-read]"),
            "{source_id} 应继续固定为纯进程输入读取"
        );
        assert!(
            source.contains("feature.foundation-platform.version-and-startup"),
            "{source_id} 应继续映射版本与启动功能"
        );
    }

    let production = read_repository_text("crates/inputcodex-platform/src/version_startup.rs");
    for expected in [
        "INPUTCODEX_SHOW_UPDATE",
        "CARGO_PKG_VERSION",
        "--show-update",
    ] {
        assert!(
            production.contains(expected),
            "生产适配器应包含：{expected}"
        );
    }
    for forbidden in [
        "CODEX_PLUS_SHOW_UPDATE",
        "std::fs",
        "std::thread",
        "reqwest",
        "ureq",
    ] {
        assert!(
            !production.contains(forbidden),
            "生产适配器禁止能力或旧变量：{forbidden}"
        );
    }

    assert_repository_text_contains(
        "parity/README.md",
        &[
            "Issue `#81`",
            "StartupIntent::ShowUpdate",
            "INVALID_STARTUP_OPTION",
        ],
    );
}

#[test]
fn gate5_运行时环境观察已实现但破坏性总功能仍未评估() {
    let feature_text = read_repository_text("parity/features/foundation-platform.yml");
    let observation = yaml_list_item_block(
        &feature_text,
        "feature.foundation-platform.runtime-environment-conflict-observation",
    );
    for expected in [
        "status: implemented",
        "detected_env_conflicts_from_pairs",
        "tauri-command:check_env_conflicts",
        "- issue:85",
        "- issue:86",
    ] {
        assert!(
            observation.contains(expected),
            "运行时环境观察功能条目应包含：{expected}"
        );
    }
    assert!(!observation.contains("remove_env_conflicts"));

    let umbrella = yaml_list_item_block(
        &feature_text,
        "feature.foundation-platform.environment-conflicts",
    );
    for expected in [
        "status: unassessed",
        "core-module:env_conflicts",
        "tauri-command:remove_env_conflicts",
        "- issue:85",
    ] {
        assert!(
            umbrella.contains(expected),
            "原环境冲突总功能应继续包含：{expected}"
        );
    }
    assert!(!umbrella.contains("status: implemented"));
    assert!(!umbrella.contains("tauri-command:check_env_conflicts"));

    let contract_text = read_repository_text("parity/contracts/foundation-platform.yml");
    let contract = yaml_list_item_block(
        &contract_text,
        "contract.feature.foundation-platform.runtime-environment-conflict-observation.baseline",
    );
    for expected in [
        "environment-read",
        "persistence: 'none'",
        "Ready(empty)",
        "runtime_process: Observed",
        "persistent_user: NotObserved",
        "persistent_system: NotObserved",
        "RUNTIME_ENVIRONMENT_OBSERVATION_UNSUPPORTED",
        "RUNTIME_ENVIRONMENT_OBSERVATION_TIMEOUT",
        "RUNTIME_ENVIRONMENT_NAME_UNREPRESENTABLE",
    ] {
        assert!(
            contract.contains(expected),
            "运行时环境观察合同应包含：{expected}"
        );
    }
    for forbidden in [
        "environment-write",
        "filesystem-read",
        "filesystem-write",
        "network-read",
        "network-write",
        "process-control",
        "advertising",
        "remote-recommendation",
        "变量实际值",
    ] {
        assert!(
            !contract.contains(forbidden),
            "运行时环境观察合同禁止能力或敏感输出：{forbidden}"
        );
    }

    let source_text = read_repository_text("parity/features/source-index.yml");
    let check = yaml_list_item_block(&source_text, "tauri-command:check_env_conflicts");
    assert!(check.contains("side_effects: [environment-read]"));
    assert!(check.contains(
        "feature_id: feature.foundation-platform.runtime-environment-conflict-observation"
    ));
    assert!(!check.contains("environment-write"));

    for source_id in [
        "core-module:env_conflicts",
        "tauri-command:remove_env_conflicts",
    ] {
        let source = yaml_list_item_block(&source_text, source_id);
        assert!(source.contains("side_effects: [environment-read, environment-write]"));
        assert!(source.contains("feature_id: feature.foundation-platform.environment-conflicts"));
    }

    let production =
        read_repository_text("crates/inputcodex-platform/src/runtime_environment_observation.rs");
    assert_eq!(production.matches("std::env::vars_os()").count(), 1);
    for forbidden in [
        "std::env::set_var",
        "std::env::remove_var",
        "std::fs",
        "std::process::Command",
        "std::thread",
        "unsafe {",
    ] {
        assert!(
            !production.contains(forbidden),
            "生产适配器禁止能力：{forbidden}"
        );
    }

    assert_repository_text_contains(
        "parity/README.md",
        &[
            "Issue `#86`",
            "Ready(empty)",
            "persistent_user: NotObserved",
            "RUNTIME_ENVIRONMENT_NAME_UNREPRESENTABLE",
        ],
    );
}

#[test]
fn gate5_relay_环境只读观察已实现但网络环境总功能仍未评估() {
    let feature_text = read_repository_text("parity/features/provider-network.yml");
    let observation = yaml_list_item_block(
        &feature_text,
        "feature.provider-network.relay-environment-observation",
    );
    for expected in [
        "name: 'Relay 环境只读观察'",
        "status: implemented",
        "core-module:relay_environment",
        "tauri-command:check_relay_environment",
        "- issue:88",
        "- issue:89",
    ] {
        assert!(
            observation.contains(expected),
            "Relay 环境观察功能条目应包含：{expected}"
        );
    }
    assert!(!observation.contains("core-module:proxy"));

    let umbrella = yaml_list_item_block(
        &feature_text,
        "feature.provider-network.network-environment",
    );
    for expected in ["status: unassessed", "core-module:proxy", "- issue:88"] {
        assert!(
            umbrella.contains(expected),
            "原网络环境总功能应继续包含：{expected}"
        );
    }
    assert!(!umbrella.contains("status: implemented"));
    assert!(!umbrella.contains("core-module:relay_environment"));
    assert!(!umbrella.contains("tauri-command:check_relay_environment"));

    let contract_text = read_repository_text("parity/contracts/provider-network.yml");
    let contract = yaml_list_item_block(
        &contract_text,
        "contract.feature.provider-network.relay-environment-observation.baseline",
    );
    for expected in [
        "environment-read",
        "filesystem-read",
        "persistence: 'none'",
        "无风险仍返回 Ready",
        "persistent_user: Unavailable",
        "persistent_user: NotObserved",
        "64 KiB",
        "RELAY_ENVIRONMENT_OBSERVATION_UNSUPPORTED",
        "USER_HOME_UNAVAILABLE",
        "CODEX_HOME_INVALID",
        "RELAY_ENVIRONMENT_NAME_UNREPRESENTABLE",
        "RELAY_ENVIRONMENT_OBSERVATION_FAILED",
    ] {
        assert!(
            contract.contains(expected),
            "Relay 环境观察合同应包含：{expected}"
        );
    }
    for forbidden in [
        "environment-write",
        "filesystem-write",
        "network-read",
        "network-write",
        "process-control",
        "advertising",
        "remote-recommendation",
        "环境变量值",
        "实际路径",
    ] {
        assert!(
            !contract.contains(forbidden),
            "Relay 环境观察合同禁止能力或敏感输出：{forbidden}"
        );
    }

    let source_text = read_repository_text("parity/features/source-index.yml");
    for source_id in [
        "core-module:relay_environment",
        "tauri-command:check_relay_environment",
    ] {
        let source = yaml_list_item_block(&source_text, source_id);
        assert!(source.contains("side_effects: [environment-read, filesystem-read]"));
        assert!(
            source.contains("feature_id: feature.provider-network.relay-environment-observation")
        );
        assert!(!source.contains("network-read"));
    }
    let proxy = yaml_list_item_block(&source_text, "core-module:proxy");
    assert!(proxy.contains("side_effects: [environment-read, network-read]"));
    assert!(proxy.contains("feature_id: feature.provider-network.network-environment"));

    let shared =
        read_repository_text("crates/inputcodex-platform/src/relay_environment_observation.rs");
    let windows = read_repository_text(
        "crates/inputcodex-platform/src/relay_environment_observation/windows.rs",
    );
    let macos = read_repository_text(
        "crates/inputcodex-platform/src/relay_environment_observation/macos.rs",
    );
    assert_eq!(windows.matches("std::env::vars_os().collect()").count(), 1);
    assert_eq!(macos.matches("std::env::vars_os().collect()").count(), 1);
    for expected in ["64 * 1024", "fs::metadata", "File::open", "read_to_end"] {
        assert!(shared.contains(expected), "共享平台探针应包含：{expected}");
    }
    for expected in [
        "windows_registry",
        "CURRENT_USER_ENVIRONMENT",
        "LOCAL_MACHINE_ENVIRONMENT",
        "RegistryReadError::Unavailable",
    ] {
        assert!(
            windows.contains(expected),
            "Windows 适配器应包含：{expected}"
        );
    }
    for expected in [
        "PersistentEnvironment::NotObserved",
        "Library/Application Support",
    ] {
        assert!(macos.contains(expected), "macOS 适配器应包含：{expected}");
    }
    let production = format!("{shared}\n{windows}\n{macos}");
    for forbidden in [
        "std::env::set_var",
        "std::env::remove_var",
        "fs::write",
        "OpenOptions",
        "std::process::Command",
        "Command::new",
        "std::thread",
        "reqwest",
        "hyper",
        "TcpStream",
        "UdpSocket",
        "unsafe {",
    ] {
        assert!(
            !production.contains(forbidden),
            "Relay 环境生产适配器禁止能力：{forbidden}"
        );
    }

    assert_repository_text_contains(
        "parity/README.md",
        &[
            "Issue `#89`",
            "feature.provider-network.relay-environment-observation",
            "64 KiB",
            "persistent_user: NotObserved",
            "persistent_user: Unavailable",
        ],
    );
}

#[test]
fn 仓库source_index_覆盖锁定上游公开入口() {
    let summary =
        validate_feature_repository(&repository_root()).expect("功能目录应通过仓库级验证");

    assert_eq!(summary.source_entry_count(), 133);
    assert_eq!(summary.feature_count(), 38);
    assert_eq!(summary.excluded_entry_count(), 3);
    assert_eq!(summary.exception_pending_count(), 10);
    assert_eq!(summary.coverage_gap_count(), 0);
}

#[test]
fn 仓库功能目录通过完整引用与安全验证() {
    let summary = validate_repository(&repository_root()).expect("仓库功能目录应通过验证");

    assert_eq!(summary.source_entry_count(), 133);
    assert_eq!(summary.feature_count(), 38);
    assert_eq!(summary.contract_count(), 38);
    assert_eq!(summary.fixture_count(), 11);
    assert_eq!(summary.coverage_gap_count(), 0);
}

#[test]
fn parity_文本文件不包含非法控制字节() {
    let mut pending = vec![repository_root().join("parity")];

    while let Some(directory) = pending.pop() {
        for entry in fs::read_dir(&directory).expect("应能枚举 parity 目录") {
            let entry = entry.expect("应能读取 parity 目录项");
            let file_type = entry.file_type().expect("应能读取 parity 文件类型");
            if file_type.is_dir() {
                pending.push(entry.path());
                continue;
            }
            if !file_type.is_file()
                || !matches!(
                    entry
                        .path()
                        .extension()
                        .and_then(|extension| extension.to_str()),
                    Some("md" | "yml" | "yaml")
                )
            {
                continue;
            }

            let path = entry.path();
            let bytes = fs::read(&path).expect("应能读取 parity 文本文件");
            assert!(
                bytes.iter().all(|byte| {
                    !((*byte < 0x20 && !matches!(*byte, b'\t' | b'\n' | b'\r')) || *byte == 0x7f)
                }),
                "{} 包含非法控制字节",
                path.display()
            );
        }
    }
}
