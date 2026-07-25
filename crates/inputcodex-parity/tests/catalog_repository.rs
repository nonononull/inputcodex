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

const RELEASE_TAG: &str = "v1.2.41";
const RELEASE_COMMIT: &str = "3dafffcafb2566a1e8bce4b35671656d6adb3eda";
const RELEASE_42_TAG: &str = "v1.2.42";
const RELEASE_42_COMMIT: &str = "657cd33e009ad02515d30db6492cd4e669b06318";
const RE_AUDIT_ISSUE_URL: &str = "https://github.com/nonononull/inputcodex/issues/34";

const VALID_SOURCE_INDEX: &str = r#"
schema_version: inputcodex.source-index.v1
release:
  tag: v1.2.41
  tag_commit: 3dafffcafb2566a1e8bce4b35671656d6adb3eda
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
    let invalid = VALID_SOURCE_INDEX.replace("tag: v1.2.41", "tag: v1.2.42");
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

    fixture.write_source_lock(SourceLockState {
        snapshot_tag: RELEASE_42_TAG,
        snapshot_commit: RELEASE_42_COMMIT,
        catalog_tag: RELEASE_TAG,
        catalog_commit: RELEASE_COMMIT,
        status: "stale-re-audit-required",
        stale_reason: Some("上游 v1.2.42 已缓存，功能目录尚未完成复审"),
        re_audit_issue_ref: Some(RE_AUDIT_ISSUE_URL),
    });
    let summary = validate_feature_repository(fixture.root())
        .expect("显式 stale 审计基线必须允许同步与目录复审继续进行");
    assert!(summary.requires_reaudit());

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

    fixture.write_source_lock(SourceLockState {
        snapshot_tag: RELEASE_42_TAG,
        snapshot_commit: RELEASE_42_COMMIT,
        catalog_tag: RELEASE_TAG,
        catalog_commit: RELEASE_COMMIT,
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
        snapshot_tag: RELEASE_42_TAG,
        snapshot_commit: RELEASE_42_COMMIT,
        catalog_tag: RELEASE_TAG,
        catalog_commit: RELEASE_COMMIT,
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
fn 仓库v1_2_42目录重新审计恢复current() {
    let summary = validate_repository(&repository_root()).expect("v1.2.42 功能目录应通过完整验证");
    assert!(
        !summary.requires_reaudit(),
        "完成重新审计后不得继续标记 stale"
    );

    for relative_path in [
        "parity/features/foundation-platform.yml",
        "parity/features/plugin-script.yml",
        "parity/features/provider-network.yml",
        "parity/features/remote-install.yml",
        "parity/features/session-data.yml",
        "parity/features/source-index.yml",
    ] {
        assert_repository_text_contains(relative_path, &[RELEASE_42_TAG, RELEASE_42_COMMIT]);
        assert!(
            !read_repository_text(relative_path).contains(RELEASE_TAG),
            "{relative_path} 不得保留 v1.2.41 Release 元数据"
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
        assert_repository_text_contains(relative_path, &[RELEASE_42_TAG]);
        assert!(
            !read_repository_text(relative_path).contains(RELEASE_TAG),
            "{relative_path} 不得保留 v1.2.41 合同描述"
        );
    }

    assert_repository_text_contains(
        "upstream/source-lock.json",
        &[
            r#""tag": "v1.2.42""#,
            r#""commit": "657cd33e009ad02515d30db6492cd4e669b06318""#,
            r#""status": "current""#,
            r#""stale_reason": null"#,
            r#""re_audit_issue_ref": null"#,
        ],
    );
}

#[test]
fn 仓库v1_2_42受影响行为证据被固定() {
    assert_repository_text_contains(
        "parity/features/foundation-platform.yml",
        &["OpenAI.ChatGPT-Desktop", "issue:38"],
    );
    assert_repository_text_contains(
        "parity/contracts/foundation-platform.yml",
        &["OpenAI.ChatGPT-Desktop"],
    );

    assert_repository_text_contains(
        "parity/features/session-data.yml",
        &["CODEX_SQLITE_HOME", "grouped-undo-token", "issue:38"],
    );
    assert_repository_text_contains(
        "parity/contracts/session-data.yml",
        &[
            "CODEX_SQLITE_HOME",
            "grouped-undo-token",
            "LOCAL_SESSION_UNDO_PREFLIGHT_FAILED",
            "LOCAL_SESSION_UNDO_PATH_REJECTED",
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
        &["companion", "renderer-inject.js", "issue:38"],
    );
    assert_repository_text_contains(
        "parity/contracts/plugin-script.yml",
        &[
            "data:image/png;base64",
            "DREAM_SKIN_COMPANION_INVALID",
            "companion 显示仍依赖 renderer 注入",
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
}

#[test]
fn 仓库source_index_覆盖锁定上游公开入口() {
    let summary =
        validate_feature_repository(&repository_root()).expect("功能目录应通过仓库级验证");

    assert_eq!(summary.source_entry_count(), 133);
    assert_eq!(summary.feature_count(), 36);
    assert_eq!(summary.excluded_entry_count(), 3);
    assert_eq!(summary.exception_pending_count(), 10);
    assert_eq!(summary.coverage_gap_count(), 0);
}

#[test]
fn 仓库功能目录通过完整引用与安全验证() {
    let summary = validate_repository(&repository_root()).expect("仓库功能目录应通过验证");

    assert_eq!(summary.source_entry_count(), 133);
    assert_eq!(summary.feature_count(), 36);
    assert_eq!(summary.contract_count(), 36);
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
