use std::{collections::BTreeSet, fs, path::PathBuf};

use inputcodex_parity::{
    ValidationCode, parse_source_index, validate_feature_repository, validate_repository,
    validate_source_index,
};

const RELEASE_TAG: &str = "v1.2.41";
const RELEASE_COMMIT: &str = "3dafffcafb2566a1e8bce4b35671656d6adb3eda";

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
