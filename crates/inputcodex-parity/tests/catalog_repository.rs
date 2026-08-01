use std::{
    collections::BTreeSet,
    fs,
    path::{Path, PathBuf},
    sync::atomic::{AtomicU64, Ordering},
};

use inputcodex_parity::{
    ValidationCode, parse_source_index, validate_feature_repository, validate_repository,
    validate_source_index,
};

const RELEASE_TAG: &str = "v1.2.44";
const RELEASE_COMMIT: &str = "77091ccaee4423f35a1b2c51c4ecd703e6201092";
const PREVIOUS_RELEASE_TAG: &str = "v1.2.43";
const PREVIOUS_RELEASE_COMMIT: &str = "5036ff056b5c629f19356396b17d6eeb70da664c";
const RE_AUDIT_ISSUE_URL: &str = "https://github.com/nonononull/inputcodex/issues/115";
static NEXT_FIXTURE_ID: AtomicU64 = AtomicU64::new(0);

const VALID_SOURCE_INDEX: &str = r#"
schema_version: inputcodex.source-index.v1
release:
  tag: v1.2.44
  tag_commit: 77091ccaee4423f35a1b2c51c4ecd703e6201092
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
        let target_root = repository_root().join("target");
        fs::create_dir_all(&target_root).expect("应能创建临时功能目录父目录");
        let root = loop {
            let fixture_id = NEXT_FIXTURE_ID.fetch_add(1, Ordering::Relaxed);
            let candidate = target_root.join(format!(
                "inputcodex-release-audit-{}-{fixture_id}",
                std::process::id()
            ));
            match fs::create_dir(&candidate) {
                Ok(()) => break candidate,
                Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {}
                Err(error) => panic!("应能创建临时功能目录夹具：{error}"),
            }
        };

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

    fn write_current_source_lock(&self) {
        self.write_source_lock(SourceLockState {
            snapshot_tag: RELEASE_TAG,
            snapshot_commit: RELEASE_COMMIT,
            catalog_tag: RELEASE_TAG,
            catalog_commit: RELEASE_COMMIT,
            status: "current",
            stale_reason: None,
            re_audit_issue_ref: None,
        });
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

    fn append_text(&self, relative_path: &str, suffix: &str) {
        let path = self.root.join(relative_path);
        let mut text = fs::read_to_string(&path).expect("应能读取待追加的临时文本");
        text.push_str(suffix);
        fs::write(path, text).expect("应能追加临时文本");
    }

    fn replace_text(&self, relative_path: &str, expected: &str, replacement: &str) {
        let path = self.root.join(relative_path);
        let text = fs::read_to_string(&path)
            .expect("应能读取待替换的临时文本")
            .replace("\r\n", "\n");
        assert!(text.contains(expected), "临时文本必须包含待替换片段");
        fs::write(path, text.replacen(expected, replacement, 1)).expect("应能替换临时文本");
    }

    fn remove_file(&self, relative_path: &str) {
        fs::remove_file(self.root.join(relative_path)).expect("应能删除临时证据文件");
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

    fixture.write_catalog_release(PREVIOUS_RELEASE_TAG, PREVIOUS_RELEASE_COMMIT);
    fixture.write_source_lock(SourceLockState {
        snapshot_tag: RELEASE_TAG,
        snapshot_commit: RELEASE_COMMIT,
        catalog_tag: PREVIOUS_RELEASE_TAG,
        catalog_commit: PREVIOUS_RELEASE_COMMIT,
        status: "stale-re-audit-required",
        stale_reason: Some("上游 v1.2.44 已缓存，功能目录仍为 v1.2.43"),
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
        stale_reason: Some("上游 v1.2.43 已缓存，功能目录尚未完成复审"),
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
fn stale_允许活动新快照入口与旧证据等待独立重审() {
    let fixture = FeatureRepositoryFixture::new();
    fixture.write_catalog_release(PREVIOUS_RELEASE_TAG, PREVIOUS_RELEASE_COMMIT);
    fixture.write_source_lock(SourceLockState {
        snapshot_tag: RELEASE_TAG,
        snapshot_commit: RELEASE_COMMIT,
        catalog_tag: PREVIOUS_RELEASE_TAG,
        catalog_commit: PREVIOUS_RELEASE_COMMIT,
        status: "stale-re-audit-required",
        stale_reason: Some("活动快照已有新入口，功能目录等待独立重审"),
        re_audit_issue_ref: Some(RE_AUDIT_ISSUE_URL),
    });
    fixture.append_text(
        "upstream/CodexPlusPlus/apps/codex-plus-manager/src-tauri/src/commands.rs",
        "\n#[tauri::command]\npub async fn newly_released_command() {}\n",
    );
    fixture.append_text(
        "upstream/CodexPlusPlus/crates/codex-plus-core/src/lib.rs",
        "\npub mod newly_released_module;\n",
    );
    fixture.remove_file("upstream/CodexPlusPlus/crates/codex-plus-core/src/app_paths.rs");

    let summary = validate_feature_repository(fixture.root())
        .expect("合法 stale 只应验证目录内部合同，新快照覆盖等待独立重审");
    assert!(summary.requires_reaudit());
}

#[test]
fn current_继续拒绝活动快照缺失来源入口() {
    let fixture = FeatureRepositoryFixture::new();
    fixture.write_current_source_lock();
    fixture.append_text(
        "upstream/CodexPlusPlus/apps/codex-plus-manager/src-tauri/src/commands.rs",
        "\n#[tauri::command]\npub async fn newly_released_command() {}\n",
    );
    fixture.append_text(
        "upstream/CodexPlusPlus/crates/codex-plus-core/src/lib.rs",
        "\npub mod newly_released_module;\n",
    );

    let error = validate_feature_repository(fixture.root())
        .expect_err("current 必须拒绝活动快照中尚未进入 source-index 的入口");
    assert!(
        error
            .issues()
            .iter()
            .any(|issue| issue.code() == ValidationCode::MissingSourceEntry),
        "实际错误：{error:?}"
    );
}

#[test]
fn current_继续拒绝目录中的意外来源入口() {
    let fixture = FeatureRepositoryFixture::new();
    fixture.write_current_source_lock();
    fixture.replace_text(
        "upstream/CodexPlusPlus/apps/codex-plus-manager/src-tauri/src/commands.rs",
        "#[tauri::command]\npub async fn load_overview",
        "pub async fn load_overview",
    );

    let error = validate_feature_repository(fixture.root())
        .expect_err("current 必须拒绝 source-index 中已不再公开的入口");
    assert!(
        error
            .issues()
            .iter()
            .any(|issue| issue.code() == ValidationCode::UnexpectedSourceEntry),
        "实际错误：{error:?}"
    );
}

#[test]
fn current_继续拒绝错误来源证据() {
    let fixture = FeatureRepositoryFixture::new();
    fixture.write_current_source_lock();
    fixture.replace_text(
        "parity/features/source-index.yml",
        "  - id: tauri-command:load_overview\n    kind: tauri-command\n    evidence:\n      path: upstream/CodexPlusPlus/apps/codex-plus-manager/src-tauri/src/commands.rs",
        "  - id: tauri-command:load_overview\n    kind: tauri-command\n    evidence:\n      path: upstream/CodexPlusPlus/apps/codex-plus-manager/src-tauri/src/lib.rs",
    );

    let error = validate_feature_repository(fixture.root())
        .expect_err("current 必须拒绝与活动快照不一致的来源证据");
    assert!(
        error
            .issues()
            .iter()
            .any(|issue| issue.code() == ValidationCode::SourceEvidenceMismatch),
        "实际错误：{error:?}"
    );
}

#[test]
fn current_继续拒绝缺失证据文件() {
    let fixture = FeatureRepositoryFixture::new();
    fixture.write_current_source_lock();
    fixture.remove_file("upstream/CodexPlusPlus/crates/codex-plus-core/src/app_paths.rs");

    let error = validate_feature_repository(fixture.root())
        .expect_err("current 必须拒绝活动快照中缺失的证据文件");
    assert!(
        error
            .issues()
            .iter()
            .any(|issue| issue.code() == ValidationCode::InvalidEvidencePath),
        "实际错误：{error:?}"
    );
}

#[test]
fn 仓库v1_2_44目录重新审计证据保持固定() {
    validate_repository(&repository_root()).expect("v1.2.44 功能目录证据应通过完整验证");

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
            "{relative_path} 不得保留 v1.2.43 Release 元数据"
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
            "{relative_path} 不得保留 v1.2.43 合同描述"
        );
    }
}

#[test]
fn 仓库v1_2_44受影响行为证据被固定() {
    assert_repository_text_contains(
        "parity/features/foundation-platform.yml",
        &["CDP", "data bridge", "保留端口", "expires_at", "issue:115"],
    );
    assert_repository_text_contains(
        "parity/contracts/foundation-platform.yml",
        &[
            "CDP",
            "data bridge",
            "保留端口",
            "不得加载、下载或展示 sponsor",
        ],
    );

    assert_repository_text_contains(
        "parity/features/session-data.yml",
        &[
            "CODEX_HOME",
            "local_thread_catalog",
            "sqliteCatalogRowsInserted",
            "issue:115",
        ],
    );
    assert_repository_text_contains(
        "parity/contracts/session-data.yml",
        &[
            "CODEX_HOME",
            "sqlite_catalog_rows_inserted",
            "local_thread_catalog",
            "observation_sequence",
        ],
    );
    assert_repository_text_contains(
        "parity/fixtures/feature.session-data.provider-metadata-maintenance/baseline.yml",
        &[
            "codex_home_source: CODEX_HOME",
            "sqlite_catalog_rows_inserted: 1",
            "observation_sequence: 1",
        ],
    );

    assert_repository_text_contains(
        "parity/features/plugin-script.yml",
        &[
            "远端认证失败",
            "本地 fallback",
            "Quick Chat",
            "data bridge",
            "issue:115",
        ],
    );
    assert_repository_text_contains(
        "parity/contracts/plugin-script.yml",
        &[
            "远端认证失败",
            "本地 fallback",
            "Quick Chat",
            "data bridge",
            "renderer 注入",
        ],
    );
    assert_repository_text_contains(
        "parity/features/provider-network.yml",
        &[
            "feature.provider-network.sub2api-billing-observation",
            "core-module:sub2api",
            "tauri-command:fetch_sub2api_billing",
            "Responses compact",
            "zstd",
            "Web Search",
            "Responses Lite",
            "issue:115",
        ],
    );

    assert_repository_text_contains(
        "parity/contracts/provider-network.yml",
        &[
            "SUB2API_BILLING_OBSERVATION_FAILED",
            "network-read",
            "Responses compact",
            "zstd",
            "Web Search",
            "Responses Lite",
            "pending import",
            "common config",
        ],
    );
    assert_repository_text_contains(
        "parity/features/source-index.yml",
        &[
            "core-module:sub2api",
            "tauri-command:fetch_sub2api_billing",
            "feature.provider-network.sub2api-billing-observation",
        ],
    );
    assert_repository_text_contains(
        "parity/fixtures/feature.provider-network.model-catalog/baseline.yml",
        &["supports_search_tool: true", "use_responses_lite: false"],
    );
    assert_repository_text_contains(
        "parity/fixtures/feature.provider-network.provider-import/baseline.yml",
        &[
            "pending_credentials_persisted: false",
            "url_credentials_redacted: true",
        ],
    );
    assert_repository_text_contains(
        "parity/fixtures/feature.provider-network.relay-profile-management/baseline.yml",
        &[
            "common_config_credentials: excluded",
            "profile_credentials: retained",
        ],
    );
    assert_repository_text_contains(
        "parity/fixtures/feature.provider-network.sub2api-billing-observation/baseline.yml",
        &[
            "group_rate_multiplier: 0.8",
            "effective_rate_multiplier: 0.9",
            "observed_at: '2026-07-30T10:00:00Z'",
        ],
    );
    assert_repository_text_contains(
        "parity/fixtures/feature.provider-network.sub2api-billing-observation/manifest.yml",
        &["合成 Sub2API", "不包含真实凭据"],
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
fn gate5_设置只读观察已实现但设置管理总功能仍未评估() {
    let feature_text = read_repository_text("parity/features/foundation-platform.yml");
    let observation = yaml_list_item_block(
        &feature_text,
        "feature.foundation-platform.settings-observation",
    );
    for expected in [
        "name: '设置只读观察'",
        "status: implemented",
        "tauri-command:load_settings",
        "- issue:91",
        "- issue:92",
    ] {
        assert!(
            observation.contains(expected),
            "设置只读观察功能条目应包含：{expected}"
        );
    }
    for forbidden in [
        "core-module:settings",
        "tauri-command:reset_settings",
        "tauri-command:save_settings",
    ] {
        assert!(
            !observation.contains(forbidden),
            "设置只读观察不得接管原总功能入口：{forbidden}"
        );
    }

    let umbrella = yaml_list_item_block(
        &feature_text,
        "feature.foundation-platform.settings-management",
    );
    for expected in [
        "status: unassessed",
        "core-module:settings",
        "tauri-command:reset_settings",
        "tauri-command:save_settings",
        "- issue:91",
    ] {
        assert!(
            umbrella.contains(expected),
            "原设置管理总功能应继续包含：{expected}"
        );
    }
    assert!(!umbrella.contains("status: implemented"));
    assert!(!umbrella.contains("tauri-command:load_settings"));

    let contract_text = read_repository_text("parity/contracts/foundation-platform.yml");
    let contract = yaml_list_item_block(
        &contract_text,
        "contract.feature.foundation-platform.settings-observation.baseline",
    );
    for expected in [
        "filesystem-read",
        "persistence: 'none'",
        "top_level_entry_count",
        "LoadCompletion::Ready",
        "LoadCompletion::Empty",
        "NotConfigured",
        "256 KiB",
        "SETTINGS_OBSERVATION_UNSUPPORTED",
        "SETTINGS_OBSERVATION_UNAVAILABLE",
        "SETTINGS_OBSERVATION_INVALID_FILE_TYPE",
        "SETTINGS_OBSERVATION_TOO_LARGE",
        "SETTINGS_OBSERVATION_INVALID_JSON",
        "SETTINGS_OBSERVATION_INVALID_ROOT",
        "mode: none",
        "fixture_refs: []",
    ] {
        assert!(
            contract.contains(expected),
            "设置只读观察合同应包含：{expected}"
        );
    }
    for forbidden in [
        "filesystem-write",
        "environment-write",
        "network-read",
        "network-write",
        "process-control",
        "advertising",
        "remote-recommendation",
    ] {
        assert!(
            !contract.contains(forbidden),
            "设置只读观察合同禁止能力：{forbidden}"
        );
    }

    let source_text = read_repository_text("parity/features/source-index.yml");
    let load = yaml_list_item_block(&source_text, "tauri-command:load_settings");
    assert!(load.contains("side_effects: [filesystem-read]"));
    assert!(load.contains("feature_id: feature.foundation-platform.settings-observation"));
    assert!(!load.contains("filesystem-write"));

    for source_id in [
        "core-module:settings",
        "tauri-command:reset_settings",
        "tauri-command:save_settings",
    ] {
        let source = yaml_list_item_block(&source_text, source_id);
        assert!(source.contains("side_effects: [filesystem-read, filesystem-write]"));
        assert!(source.contains("feature_id: feature.foundation-platform.settings-management"));
    }

    let production = read_repository_text("crates/inputcodex-platform/src/settings_observation.rs");
    for expected in [
        "SystemPlatformPaths.resolve",
        "fs::symlink_metadata",
        "File::open",
        "file.take(limit as u64 + 1)",
        "serde_json::from_slice::<Value>",
        "object.len()",
    ] {
        assert!(
            production.contains(expected),
            "设置观察适配器应包含：{expected}"
        );
    }
    for forbidden in [
        "pub trait SettingsFileProbe",
        "pub fn observe_settings_file",
        "fs::write",
        "OpenOptions",
        "std::process::Command",
        "Command::new",
        "std::thread",
        "reqwest",
        "hyper",
        "TcpStream",
        "UdpSocket",
        "iced",
        "unsafe {",
    ] {
        assert!(
            !production.contains(forbidden),
            "设置观察生产适配器禁止能力：{forbidden}"
        );
    }
}

#[test]
fn gate5_诊断日志只读结构观察已实现但诊断总功能仍未评估() {
    let feature_text = read_repository_text("parity/features/foundation-platform.yml");
    let observation = yaml_list_item_block(
        &feature_text,
        "feature.foundation-platform.diagnostic-log-observation",
    );
    for expected in [
        "name: '诊断日志只读结构观察'",
        "status: implemented",
        "tauri-command:read_latest_logs",
        "256 KiB",
        "NoDiagnosticLog",
        "JSON object",
        "malformed",
        "- issue:94",
        "- issue:95",
    ] {
        assert!(
            observation.contains(expected),
            "诊断日志只读结构观察应包含：{expected}"
        );
    }

    let umbrella = yaml_list_item_block(&feature_text, "feature.foundation-platform.diagnostics");
    for expected in [
        "status: unassessed",
        "core-module:diagnostic_log",
        "tauri-command:clear_logs",
        "tauri-command:copy_diagnostics",
        "tauri-command:write_diagnostic_event",
        "- issue:94",
        "- issue:95",
    ] {
        assert!(
            umbrella.contains(expected),
            "原诊断总功能应继续包含：{expected}"
        );
    }
    assert!(!umbrella.contains("status: implemented"));
    assert!(!umbrella.contains("tauri-command:read_latest_logs"));

    let contract_text = read_repository_text("parity/contracts/foundation-platform.yml");
    let contract = yaml_list_item_block(
        &contract_text,
        "contract.feature.foundation-platform.diagnostic-log-observation.baseline",
    );
    for expected in [
        "filesystem-read",
        "persistence: 'none'",
        "file_size_bytes",
        "sampled_record_count",
        "valid_object_record_count",
        "malformed_record_count",
        "truncated",
        "partial_record_discarded",
        "LoadCompletion::Ready",
        "LoadCompletion::Empty",
        "NoDiagnosticLog",
        "256 KiB",
        "DIAGNOSTIC_LOG_OBSERVATION_UNSUPPORTED",
        "DIAGNOSTIC_LOG_OBSERVATION_UNAVAILABLE",
        "DIAGNOSTIC_LOG_OBSERVATION_INVALID_FILE_TYPE",
        "mode: none",
        "fixture_refs: []",
    ] {
        assert!(
            contract.contains(expected),
            "诊断日志只读结构观察合同应包含：{expected}"
        );
    }
    for forbidden in [
        "filesystem-write",
        "environment-write",
        "network-read",
        "network-write",
        "process-control",
        "clipboard-write",
        "advertising",
        "remote-recommendation",
    ] {
        assert!(
            !contract.contains(forbidden),
            "诊断日志只读结构观察合同禁止能力：{forbidden}"
        );
    }

    let source_text = read_repository_text("parity/features/source-index.yml");
    let read = yaml_list_item_block(&source_text, "tauri-command:read_latest_logs");
    assert!(read.contains("side_effects: [filesystem-read]"));
    assert!(read.contains("feature_id: feature.foundation-platform.diagnostic-log-observation"));
    assert!(!read.contains("filesystem-write"));
    assert!(!read.contains("clipboard-write"));

    for source_id in [
        "core-module:diagnostic_log",
        "tauri-command:clear_logs",
        "tauri-command:copy_diagnostics",
        "tauri-command:write_diagnostic_event",
    ] {
        let source = yaml_list_item_block(&source_text, source_id);
        assert!(
            source.contains("side_effects: [filesystem-read, filesystem-write, clipboard-write]")
        );
        assert!(source.contains("feature_id: feature.foundation-platform.diagnostics"));
    }

    let source =
        read_repository_text("crates/inputcodex-platform/src/diagnostic_log_observation.rs");
    let production = source.split("#[cfg(test)]").next().unwrap_or(&source);
    for expected in [
        "SystemPlatformPaths.resolve",
        "fs::symlink_metadata",
        "File::open",
        "file.seek(SeekFrom::Start(start))",
        "file.take(limit as u64)",
        "serde_json::from_slice::<Value>",
        "DiagnosticLogObservation::new",
    ] {
        assert!(
            production.contains(expected),
            "诊断日志观察适配器应包含：{expected}"
        );
    }
    for forbidden in [
        "pub trait DiagnosticLogFileProbe",
        "pub fn observe_diagnostic_log_file",
        "read_to_string",
        "fs::write",
        "OpenOptions",
        "std::process::Command",
        "Command::new",
        "std::thread",
        "reqwest",
        "hyper",
        "TcpStream",
        "UdpSocket",
        "iced",
        "unsafe {",
    ] {
        assert!(
            !production.contains(forbidden),
            "诊断日志观察生产适配器禁止能力：{forbidden}"
        );
    }
}

#[test]
fn gate5_relay_认证与配置状态只读观察已实现但配置管理总功能仍未评估() {
    let feature_text = read_repository_text("parity/features/provider-network.yml");
    let observation = yaml_list_item_block(
        &feature_text,
        "feature.provider-network.relay-status-observation",
    );
    for expected in [
        "name: 'Relay 认证与配置状态只读观察'",
        "status: implemented",
        "tauri-command:relay_status",
        "auth.json",
        "config.toml",
        "256 KiB",
        "- issue:97",
        "- issue:98",
    ] {
        assert!(
            observation.contains(expected),
            "Relay 状态观察功能条目应包含：{expected}"
        );
    }
    assert!(!observation.contains("core-module:relay_config"));
    assert!(!observation.contains("tauri-command:read_relay_files"));
    assert!(!observation.contains("tauri-command:save_relay_file"));
    assert!(!observation.contains("tauri-command:switch_relay_profile"));

    let umbrella = yaml_list_item_block(
        &feature_text,
        "feature.provider-network.relay-profile-management",
    );
    for expected in [
        "status: unassessed",
        "core-module:relay_config",
        "tauri-command:backfill_relay_profile_from_live",
        "tauri-command:read_relay_files",
        "tauri-command:save_relay_file",
        "tauri-command:switch_relay_profile",
        "- issue:97",
        "- issue:98",
    ] {
        assert!(
            umbrella.contains(expected),
            "原 Relay 配置管理总功能应继续包含：{expected}"
        );
    }
    assert!(!umbrella.contains("status: implemented"));
    assert!(!umbrella.contains("tauri-command:relay_status"));

    let contract_text = read_repository_text("parity/contracts/provider-network.yml");
    let contract = yaml_list_item_block(
        &contract_text,
        "contract.feature.provider-network.relay-status-observation.baseline",
    );
    for expected in [
        "filesystem-read",
        "persistence: 'none'",
        "auth_document_status",
        "config_document_status",
        "chatgpt_credentials",
        "openai_api_key",
        "relay_configuration",
        "LoadCompletion::Ready",
        "LoadCompletion::Empty",
        "256 KiB",
        "512 KiB",
        "RELAY_STATUS_OBSERVATION_UNSUPPORTED",
        "USER_HOME_UNAVAILABLE",
        "CODEX_HOME_INVALID",
        "mode: none",
        "fixture_refs: []",
    ] {
        assert!(
            contract.contains(expected),
            "Relay 状态观察合同应包含：{expected}"
        );
    }
    for forbidden in [
        "filesystem-write",
        "environment-write",
        "network-read",
        "network-write",
        "process-control",
        "advertising",
        "remote-recommendation",
    ] {
        assert!(
            !contract.contains(forbidden),
            "Relay 状态观察合同禁止能力：{forbidden}"
        );
    }

    let source_text = read_repository_text("parity/features/source-index.yml");
    let relay_status = yaml_list_item_block(&source_text, "tauri-command:relay_status");
    assert!(relay_status.contains("side_effects: [filesystem-read]"));
    assert!(relay_status.contains("feature_id: feature.provider-network.relay-status-observation"));
    assert!(!relay_status.contains("filesystem-write"));

    for source_id in [
        "core-module:relay_config",
        "tauri-command:backfill_relay_profile_from_live",
        "tauri-command:read_relay_files",
        "tauri-command:save_relay_file",
        "tauri-command:switch_relay_profile",
    ] {
        let source = yaml_list_item_block(&source_text, source_id);
        assert!(source.contains("side_effects: [filesystem-read, filesystem-write]"));
        assert!(source.contains("feature_id: feature.provider-network.relay-profile-management"));
    }

    let source = read_repository_text("crates/inputcodex-platform/src/relay_status_observation.rs");
    let production = source.split("#[cfg(test)]").next().unwrap_or(&source);
    for expected in [
        "SystemPlatformPaths.resolve",
        "fs::symlink_metadata",
        "File::open",
        "file.take(limit as u64 + 1)",
        "serde_json::from_slice::<Value>",
        "parse::<DocumentMut>()",
        "RelayStatusObservation::new",
    ] {
        assert!(
            production.contains(expected),
            "Relay 状态观察生产适配器应包含：{expected}"
        );
    }
    for forbidden in [
        "pub trait RelayStatusFileProbe",
        "pub fn observe_relay_status_files",
        "read_to_string",
        "fs::write",
        "OpenOptions",
        "std::process::Command",
        "Command::new",
        "std::thread",
        "reqwest",
        "hyper",
        "TcpStream",
        "UdpSocket",
        "iced",
        "unsafe {",
    ] {
        assert!(
            !production.contains(forbidden),
            "Relay 状态观察生产适配器禁止能力：{forbidden}"
        );
    }
}

#[test]
fn gate5_上下文能力只读目录观察已实现但上下文管理总功能仍未评估() {
    let feature_text = read_repository_text("parity/features/provider-network.yml");
    let observation = yaml_list_item_block(
        &feature_text,
        "feature.provider-network.context-entry-observation",
    );
    for expected in [
        "name: '上下文能力只读目录观察'",
        "status: implemented",
        "tauri-command:read_live_context_entries",
        "config.toml",
        "256 KiB",
        "- issue:100",
        "- issue:101",
    ] {
        assert!(
            observation.contains(expected),
            "上下文目录观察功能条目应包含：{expected}"
        );
    }
    for forbidden in [
        "tauri-command:delete_context_entry",
        "tauri-command:extract_relay_common_config",
        "tauri-command:list_context_entries",
        "tauri-command:sync_live_context_entries",
        "tauri-command:upsert_context_entry",
        "filesystem-write",
    ] {
        assert!(
            !observation.contains(forbidden),
            "上下文目录观察功能禁止能力：{forbidden}"
        );
    }

    let umbrella = yaml_list_item_block(
        &feature_text,
        "feature.provider-network.context-entry-management",
    );
    for expected in [
        "status: unassessed",
        "tauri-command:delete_context_entry",
        "tauri-command:extract_relay_common_config",
        "tauri-command:list_context_entries",
        "tauri-command:sync_live_context_entries",
        "tauri-command:upsert_context_entry",
        "- issue:100",
        "- issue:101",
    ] {
        assert!(
            umbrella.contains(expected),
            "原上下文管理总功能应继续包含：{expected}"
        );
    }
    assert!(!umbrella.contains("status: implemented"));
    assert!(!umbrella.contains("tauri-command:read_live_context_entries"));

    let contract_text = read_repository_text("parity/contracts/provider-network.yml");
    let contract = yaml_list_item_block(
        &contract_text,
        "contract.feature.provider-network.context-entry-observation.baseline",
    );
    for expected in [
        "filesystem-read",
        "persistence: 'none'",
        "LoadCompletion::Ready",
        "LoadCompletion::Empty",
        "LoadCompletion::Failed",
        "McpServer",
        "Skill",
        "Plugin",
        "total",
        "enabled",
        "disabled",
        "256 KiB",
        "CONTEXT_ENTRY_OBSERVATION_UNSUPPORTED",
        "CONTEXT_ENTRY_OBSERVATION_UNAVAILABLE",
        "CONTEXT_ENTRY_OBSERVATION_INVALID_FILE_TYPE",
        "CONTEXT_ENTRY_OBSERVATION_TOO_LARGE",
        "CONTEXT_ENTRY_OBSERVATION_INVALID_UTF8",
        "CONTEXT_ENTRY_OBSERVATION_INVALID_TOML",
        "CONTEXT_ENTRY_OBSERVATION_INVALID_ROOT_TABLE",
        "CONTEXT_ENTRY_OBSERVATION_INVALID_ENTRY_TABLE",
        "CONTEXT_ENTRY_OBSERVATION_EMPTY_ID",
        "CONTEXT_ENTRY_OBSERVATION_INVALID_BOOLEAN",
        "USER_HOME_UNAVAILABLE",
        "CODEX_HOME_INVALID",
        "mode: none",
        "fixture_refs: []",
    ] {
        assert!(
            contract.contains(expected),
            "上下文目录观察合同应包含：{expected}"
        );
    }
    for forbidden in [
        "filesystem-write",
        "environment-write",
        "network-read",
        "network-write",
        "process-control",
        "advertising",
        "remote-recommendation",
    ] {
        assert!(
            !contract.contains(forbidden),
            "上下文目录观察合同禁止能力：{forbidden}"
        );
    }

    let source_text = read_repository_text("parity/features/source-index.yml");
    let live = yaml_list_item_block(&source_text, "tauri-command:read_live_context_entries");
    assert!(live.contains("side_effects: [filesystem-read]"));
    assert!(live.contains("feature_id: feature.provider-network.context-entry-observation"));
    assert!(!live.contains("filesystem-write"));

    for source_id in [
        "tauri-command:delete_context_entry",
        "tauri-command:extract_relay_common_config",
        "tauri-command:list_context_entries",
        "tauri-command:sync_live_context_entries",
        "tauri-command:upsert_context_entry",
    ] {
        let source = yaml_list_item_block(&source_text, source_id);
        assert!(source.contains("side_effects: [filesystem-read, filesystem-write]"));
        assert!(source.contains("feature_id: feature.provider-network.context-entry-management"));
    }

    let source =
        read_repository_text("crates/inputcodex-platform/src/context_entry_observation.rs");
    let production = source.split("#[cfg(test)]").next().unwrap_or(&source);
    for expected in [
        "SystemPlatformPaths.resolve",
        "fs::symlink_metadata",
        "File::open",
        "file.take(limit as u64 + 1)",
        "Document::parse",
        "source_document.into_mut()",
        "ContextEntryObservation::new",
    ] {
        assert!(
            production.contains(expected),
            "上下文目录观察生产适配器应包含：{expected}"
        );
    }
    for forbidden in [
        "pub trait ContextEntryFileProbe",
        "pub fn observe_context_entry_file",
        "read_to_string",
        "fs::write",
        "OpenOptions",
        "std::process::Command",
        "Command::new",
        "std::thread",
        "reqwest",
        "hyper",
        "TcpStream",
        "UdpSocket",
        "iced",
        "unsafe {",
    ] {
        assert!(
            !production.contains(forbidden),
            "上下文目录观察生产适配器禁止能力：{forbidden}"
        );
    }
}

#[test]
fn gate5_本地会话目录只读观察已实现但本地会话管理总功能仍未评估() {
    let feature_text = read_repository_text("parity/features/session-data.yml");
    assert!(
        feature_text.contains("feature.session-data.local-session-directory-observation"),
        "会话目录观察必须拥有独立 feature"
    );
    let observation = yaml_list_item_block(
        &feature_text,
        "feature.session-data.local-session-directory-observation",
    );
    for expected in [
        "name: '本地会话目录只读观察'",
        "status: implemented",
        "tauri-command:list_local_sessions",
        "CODEX_SQLITE_HOME",
        "最多 32",
        "默认 50",
        "最大 100",
        "- issue:103",
        "- issue:104",
    ] {
        assert!(
            observation.contains(expected),
            "本地会话目录观察功能条目应包含：{expected}"
        );
    }
    for forbidden in [
        "tauri-command:delete_local_session",
        "grouped-undo-token",
        "database-write",
        "filesystem-write",
    ] {
        assert!(
            !observation.contains(forbidden),
            "本地会话目录观察功能禁止能力：{forbidden}"
        );
    }

    let umbrella = yaml_list_item_block(
        &feature_text,
        "feature.session-data.local-session-management",
    );
    for expected in [
        "status: unassessed",
        "core-module:codex_sqlite",
        "data-module:storage",
        "tauri-command:delete_local_session",
        "grouped-undo-token",
        "- issue:103",
        "- issue:104",
    ] {
        assert!(
            umbrella.contains(expected),
            "原本地会话管理总功能应继续包含：{expected}"
        );
    }
    assert!(!umbrella.contains("status: implemented"));
    assert!(!umbrella.contains("tauri-command:list_local_sessions"));

    let contract_text = read_repository_text("parity/contracts/session-data.yml");
    let contract = yaml_list_item_block(
        &contract_text,
        "contract.feature.session-data.local-session-directory-observation.baseline",
    );
    for expected in [
        "filesystem-read",
        "database-read",
        "persistence: 'none'",
        "LoadCompletion::Ready",
        "LoadCompletion::Empty",
        "LoadCompletion::Failed",
        "session_id",
        "display_title",
        "title_truncated",
        "archived",
        "updated_at_ms",
        "offset",
        "limit",
        "has_more",
        "Complete",
        "Partial",
        "默认 50",
        "最大 100",
        "LOCAL_SESSION_DIRECTORY_INVALID_PAGINATION",
        "LOCAL_SESSION_DIRECTORY_INVALID_SQLITE_HOME",
        "LOCAL_SESSION_DIRECTORY_TOO_MANY_DATABASES",
        "LOCAL_SESSION_DIRECTORY_UNSUPPORTED_SCHEMA",
        "LOCAL_SESSION_DIRECTORY_UNAVAILABLE",
        "LOCAL_SESSION_DIRECTORY_TIMEOUT",
        "LOCAL_SESSION_DIRECTORY_CANCELLED",
        "LOCAL_SESSION_DIRECTORY_UNSUPPORTED",
        "USER_HOME_UNAVAILABLE",
        "CODEX_HOME_INVALID",
        "mode: required",
        "fixture.feature.session-data.local-session-directory-observation.baseline",
    ] {
        assert!(
            contract.contains(expected),
            "本地会话目录观察合同应包含：{expected}"
        );
    }
    for forbidden in [
        "database-write",
        "filesystem-write",
        "network-read",
        "network-write",
        "process-control",
        "grouped-undo-token",
        "delete",
        "backup",
        "restore",
    ] {
        assert!(
            !contract.contains(forbidden),
            "本地会话目录观察合同禁止能力：{forbidden}"
        );
    }

    assert_repository_text_contains(
        "parity/fixtures/feature.session-data.local-session-directory-observation/manifest.yml",
        &[
            "feature_id: feature.session-data.local-session-directory-observation",
            "fixture.feature.session-data.local-session-directory-observation.baseline",
            "path: baseline.yml",
            "kind: synthetic",
        ],
    );
    assert_repository_text_contains(
        "parity/fixtures/feature.session-data.local-session-directory-observation/baseline.yml",
        &[
            "schema_version: inputcodex.synthetic.local-session-directory.v1",
            "coverage: Partial",
            "session_id: synthetic-session-current",
            "session_id: synthetic-session-legacy",
            "title_truncated: true",
            "database_mode: read-only",
            "query_only: true",
            "database_paths_exposed: false",
        ],
    );

    let source_text = read_repository_text("parity/features/source-index.yml");
    let list = yaml_list_item_block(&source_text, "tauri-command:list_local_sessions");
    assert!(list.contains("side_effects: [filesystem-read, database-read]"));
    assert!(list.contains("feature_id: feature.session-data.local-session-directory-observation"));
    assert!(!list.contains("database-write"));
    assert!(!list.contains("filesystem-write"));

    for source_id in [
        "core-module:codex_sqlite",
        "data-module:storage",
        "tauri-command:delete_local_session",
    ] {
        let source = yaml_list_item_block(&source_text, source_id);
        assert!(source.contains("feature_id: feature.session-data.local-session-management"));
        assert!(source.contains("database-write"));
        assert!(source.contains("filesystem-write"));
    }

    assert_repository_text_contains(
        "parity/README.md",
        &[
            "当前共有 44 个 feature",
            "`44` 份行为合同",
            "`12` 个 fixture manifest",
            "feature.session-data.local-session-directory-observation",
        ],
    );
}

#[test]
fn gate5_会话_markdown_生成已实现但文件保存继续留在_renderer_例外() {
    let feature_text = read_repository_text("parity/features/session-data.yml");
    let generation =
        yaml_list_item_block(&feature_text, "feature.session-data.markdown-generation");
    for expected in [
        "name: '会话 Markdown 生成'",
        "status: implemented",
        "data-module:markdown",
        "严格只读",
        "UTC",
        "LF",
        "最多 32",
        "20,000",
        "16 MiB",
        "- issue:108",
        "- issue:109",
    ] {
        assert!(
            generation.contains(expected),
            "Markdown 生成功能条目应包含：{expected}"
        );
    }
    for forbidden in [
        "status: unassessed",
        "filesystem-write",
        "保存对话框",
        "覆盖确认",
        "原子写入",
    ] {
        assert!(
            !generation.contains(forbidden),
            "Markdown 生成功能禁止能力：{forbidden}"
        );
    }
    assert!(!feature_text.contains("feature.session-data.markdown-export"));

    let contract_text = read_repository_text("parity/contracts/session-data.yml");
    let contract = yaml_list_item_block(
        &contract_text,
        "contract.feature.session-data.markdown-generation.baseline",
    );
    for expected in [
        "markdown-generation-request",
        "SessionMarkdownDocument",
        "suggested_filename",
        "markdown",
        "message_count",
        "database-read",
        "filesystem-read",
        "persistence: 'none'",
        "LoadCompletion::Ready",
        "LoadCompletion::Empty",
        "LoadCompletion::Failed",
        "32",
        "8 KiB",
        "32 MiB",
        "16 MiB",
        "100000",
        "20000",
        "160 UTF-8 bytes",
        "50 ms",
        "1000",
        "2 s",
        "MARKDOWN_GENERATION_INVALID_SESSION_ID",
        "MARKDOWN_GENERATION_INVALID_SQLITE_HOME",
        "MARKDOWN_GENERATION_TOO_MANY_DATABASES",
        "MARKDOWN_GENERATION_UNSUPPORTED_SCHEMA",
        "MARKDOWN_GENERATION_UNAVAILABLE",
        "MARKDOWN_GENERATION_INVALID_ROLLOUT",
        "MARKDOWN_GENERATION_INVALID_CONTENT",
        "MARKDOWN_GENERATION_RESOURCE_LIMIT",
        "MARKDOWN_GENERATION_TIMEOUT",
        "MARKDOWN_GENERATION_CANCELLED",
        "MARKDOWN_GENERATION_UNSUPPORTED",
        "fixture.feature.session-data.markdown-generation.baseline",
    ] {
        assert!(
            contract.contains(expected),
            "Markdown 生成合同应包含：{expected}"
        );
    }
    for forbidden in [
        "filesystem-write",
        "database-write",
        "network-read",
        "network-write",
        "process-control",
        "output_path",
        "save dialog",
        "atomic write",
    ] {
        assert!(
            !contract.contains(forbidden),
            "Markdown 生成合同禁止能力：{forbidden}"
        );
    }

    assert_repository_text_contains(
        "parity/fixtures/feature.session-data.markdown-generation/manifest.yml",
        &[
            "feature_id: feature.session-data.markdown-generation",
            "fixture.feature.session-data.markdown-generation.baseline",
            "path: baseline.yml",
            "kind: synthetic",
        ],
    );
    assert_repository_text_contains(
        "parity/fixtures/feature.session-data.markdown-generation/baseline.yml",
        &[
            "schema_version: inputcodex.synthetic.markdown-generation.v1",
            "database_mode: read-only",
            "query_only: true",
            "timestamp: '2025-12-31T23:30:00Z'",
            "body: '> Image attachment omitted'",
            "suggested_filename: session-Synthetic-session.md",
            "source_paths_exposed: false",
            "session_id_exposed: false",
            "file_saved: false",
        ],
    );

    let source_text = read_repository_text("parity/features/source-index.yml");
    let markdown = yaml_list_item_block(&source_text, "data-module:markdown");
    assert!(markdown.contains("side_effects: [database-read, filesystem-read]"));
    assert!(markdown.contains("feature_id: feature.session-data.markdown-generation"));
    assert!(!markdown.contains("filesystem-write"));

    let plugin_text = read_repository_text("parity/features/plugin-script.yml");
    let renderer =
        yaml_list_item_block(&plugin_text, "feature.plugin-script.renderer-enhancements");
    for expected in [
        "status: exception-pending",
        "symbol: saveMarkdown",
        "renderer 注入",
        "文件保存",
        "- issue:108",
        "- issue:109",
    ] {
        assert!(
            renderer.contains(expected),
            "renderer 例外应继续保存文件写入证据：{expected}"
        );
    }
    assert!(!renderer.contains("status: implemented"));

    let production = read_repository_text("crates/inputcodex-platform/src/markdown_generation.rs");
    for required in [
        "SQLITE_OPEN_READ_ONLY",
        "SQLITE_OPEN_NO_MUTEX",
        "query_only",
        "MAX_ROLLOUT_DISCOVERY_ENTRIES",
        "MAX_ROLLOUT_CANDIDATES",
        "MAX_ROLLOUT_DISCOVERY_BYTES",
        "MAX_ROLLOUT_BYTES",
        "MAX_ROLLOUT_RECORDS",
        "MAX_MARKDOWN_MESSAGE_COUNT",
        "symlink_metadata",
        "BufReader",
        "MARKDOWN_GENERATION_TIMEOUT",
        "MARKDOWN_GENERATION_CANCELLED",
    ] {
        assert!(
            production.contains(required),
            "Markdown 生成生产适配器应包含：{required}"
        );
    }
    for forbidden in [
        "Connection::open(",
        "read_to_string",
        "fs::write",
        "OpenOptions",
        "std::process::Command",
        "Command::new",
        "std::thread",
        "reqwest",
        "hyper",
        "TcpStream",
        "UdpSocket",
        "iced",
        "unsafe {",
    ] {
        assert!(
            !production.contains(forbidden),
            "Markdown 生成生产适配器禁止能力：{forbidden}"
        );
    }
}

#[test]
fn gate5_token_用量首候选必须按真实_cdp_写入语义隔离() {
    let upstream = read_repository_text(
        "upstream/CodexPlusPlus/crates/codex-plus-core/src/codex_local_storage.rs",
    );
    for expected in [
        "localStorage.getItem",
        "localStorage.setItem",
        "evaluate_script_with_await_promise",
        "append_diagnostic_log",
        "sanitize_local_storage_model_suffixes_nonfatal",
    ] {
        assert!(
            upstream.contains(expected),
            "上游 Local Storage 清理证据应包含：{expected}"
        );
    }
    assert!(
        !upstream.contains("codex_thread_usage_history"),
        "codex_local_storage 模块不得冒充 rollout Token 历史读取"
    );

    let launcher =
        read_repository_text("upstream/CodexPlusPlus/crates/codex-plus-core/src/launcher.rs");
    for expected in [
        "if injection_ready",
        "sanitize_local_storage_model_suffixes_nonfatal",
    ] {
        assert!(
            launcher.contains(expected),
            "launcher 应证明清理发生在注入成功后：{expected}"
        );
    }

    let source_text = read_repository_text("parity/features/source-index.yml");
    let source = yaml_list_item_block(&source_text, "core-module:codex_local_storage");
    for expected in [
        "side_effects: [network-read, filesystem-write, injection]",
        "feature_id: feature.session-data.local-storage-model-suffix-sanitization",
    ] {
        assert!(
            source.contains(expected),
            "Local Storage 来源应包含：{expected}"
        );
    }
    for forbidden in [
        "side_effects: [filesystem-read, database-read]",
        "feature.session-data.token-usage-history",
    ] {
        assert!(
            !source.contains(forbidden),
            "Local Storage 来源禁止旧语义：{forbidden}"
        );
    }

    let feature_text = read_repository_text("parity/features/session-data.yml");
    assert!(!feature_text.contains("feature.session-data.token-usage-history"));
    let feature = yaml_list_item_block(
        &feature_text,
        "feature.session-data.local-storage-model-suffix-sanitization",
    );
    for expected in [
        "status: exception-pending",
        "symbol: sanitize_local_storage_model_suffixes",
        "symbol: sanitize_local_storage_model_suffixes_nonfatal",
        "CDP",
        "JavaScript",
        "Local Storage",
        "- issue:123",
    ] {
        assert!(
            feature.contains(expected),
            "Local Storage 例外应包含：{expected}"
        );
    }
    assert!(!feature.contains("status: implemented"));

    let contract_text = read_repository_text("parity/contracts/session-data.yml");
    assert!(!contract_text.contains("contract.feature.session-data.token-usage-history.baseline"));
    let contract = yaml_list_item_block(
        &contract_text,
        "contract.feature.session-data.local-storage-model-suffix-sanitization.baseline",
    );
    for expected in [
        "feature_id: feature.session-data.local-storage-model-suffix-sanitization",
        "PARITY_EXCEPTION_PENDING",
        "side_effects:",
        "- none",
        "mode: none",
        "禁止执行上游实现",
    ] {
        assert!(
            contract.contains(expected),
            "Local Storage 例外合同应包含：{expected}"
        );
    }
    for forbidden in [
        "filesystem-read",
        "database-read",
        "fixture.feature.session-data.token-usage-history.baseline",
    ] {
        assert!(
            !contract.contains(forbidden),
            "Local Storage 例外合同禁止能力：{forbidden}"
        );
    }

    let obsolete_fixture =
        repository_root().join("parity/fixtures/feature.session-data.token-usage-history");
    for file_name in ["manifest.yml", "baseline.yml"] {
        assert!(
            !obsolete_fixture.join(file_name).exists(),
            "错误的 Token 用量 synthetic fixture 文件必须删除：{file_name}"
        );
    }
}

#[test]
fn 仓库source_index_覆盖锁定上游公开入口() {
    let summary =
        validate_feature_repository(&repository_root()).expect("功能目录应通过仓库级验证");

    assert_eq!(summary.source_entry_count(), 135);
    assert_eq!(summary.feature_count(), 44);
    assert_eq!(summary.excluded_entry_count(), 3);
    assert_eq!(summary.exception_pending_count(), 11);
    assert_eq!(summary.coverage_gap_count(), 0);
}

#[test]
fn 仓库功能目录通过完整引用与安全验证() {
    let summary = validate_repository(&repository_root()).expect("仓库功能目录应通过验证");

    assert_eq!(summary.source_entry_count(), 135);
    assert_eq!(summary.feature_count(), 44);
    assert_eq!(summary.contract_count(), 44);
    assert_eq!(summary.fixture_count(), 12);
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
