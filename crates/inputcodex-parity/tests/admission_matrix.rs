use std::collections::BTreeMap;

use inputcodex_parity::{
    AdmissionDecision, ParityStatus, SideEffectBucket, TypedOwnerState, ValidationCode,
    parse_side_effect_admission_matrix, parse_source_index, validate_side_effect_admission_matrix,
};

const VALID_SOURCE_INDEX: &str = r#"
schema_version: inputcodex.source-index.v1
release:
  tag: v1.2.44
  tag_commit: 77091ccaee4423f35a1b2c51c4ecd703e6201092
sources:
  - id: tauri-command:save_settings
    kind: tauri-command
    evidence:
      path: upstream/CodexPlusPlus/apps/codex-plus-manager/src-tauri/src/commands.rs
      symbol: save_settings
    platforms: [windows, macos]
    side_effects: [filesystem-read, filesystem-write]
    disposition:
      type: feature
      feature_id: feature.foundation-platform.settings-management
"#;

const VALID_MATRIX: &str = r#"
schema_version: inputcodex.side-effect-admission-matrix.v1
baseline_commit: 5a7465252b56f7e90673e72d3e02881ac9238141
release:
  tag: v1.2.44
  tag_commit: 77091ccaee4423f35a1b2c51c4ecd703e6201092
bucket_precedence: [write, process, network]
sources:
  - source_id: tauri-command:save_settings
    feature_id: feature.foundation-platform.settings-management
    bucket: write
    typed_owner:
      state: partial
      kinds: [filesystem-mutation]
    blocker_refs: ['issue:94']
    admission: blocked
    implementation_authorized: false
"#;

fn feature_statuses() -> BTreeMap<String, ParityStatus> {
    BTreeMap::from([(
        "feature.foundation-platform.settings-management".to_owned(),
        ParityStatus::Unassessed,
    )])
}

#[test]
fn 合法矩阵解析为严格领域类型() {
    let matrix = parse_side_effect_admission_matrix(VALID_MATRIX).expect("合法矩阵应可解析");
    let source = &matrix.sources()[0];

    assert_eq!(matrix.sources().len(), 1);
    assert_eq!(source.source_id(), "tauri-command:save_settings");
    assert_eq!(source.bucket(), SideEffectBucket::Write);
    assert_eq!(source.typed_owner().state(), TypedOwnerState::Partial);
    assert_eq!(source.admission(), AdmissionDecision::Blocked);
    assert!(!source.implementation_authorized());
}

#[test]
fn 矩阵拒绝未知字段和类型伪装() {
    for invalid in [
        VALID_MATRIX.replace("sources:\n", "candidate: forbidden\nsources:\n"),
        VALID_MATRIX.replace(
            "implementation_authorized: false",
            "implementation_authorized: 'false'",
        ),
        VALID_MATRIX.replace("bucket: write", "bucket: Write"),
        VALID_MATRIX.replace("state: partial", "state: [partial]"),
    ] {
        assert!(
            parse_side_effect_admission_matrix(&invalid).is_err(),
            "字段、类型和大小写漂移必须在解析阶段拒绝"
        );
    }
}

#[test]
fn 矩阵与未评估_source_集合和能力桶一对一匹配() {
    let matrix = parse_side_effect_admission_matrix(VALID_MATRIX).expect("合法矩阵应可解析");
    let source_index = parse_source_index(VALID_SOURCE_INDEX).expect("测试 source index 应可解析");

    assert!(
        validate_side_effect_admission_matrix(&matrix, &source_index, &feature_statuses())
            .is_empty()
    );
}

#[test]
fn 矩阵拒绝_typed_owner_和_blocker_合法值漂移() {
    let source_index = parse_source_index(VALID_SOURCE_INDEX).expect("测试 source index 应可解析");
    for (input, expected_code) in [
        (
            VALID_MATRIX.replace("state: partial", "state: missing"),
            ValidationCode::AdmissionOwnerMismatch,
        ),
        (
            VALID_MATRIX.replace("filesystem-mutation", "clipboard-mutation"),
            ValidationCode::AdmissionOwnerMismatch,
        ),
        (
            VALID_MATRIX.replace("'issue:94'", "'issue:999'"),
            ValidationCode::AdmissionBlockerMismatch,
        ),
    ] {
        let matrix = parse_side_effect_admission_matrix(&input).expect("合法枚举值变异仍应可解析");
        let issues =
            validate_side_effect_admission_matrix(&matrix, &source_index, &feature_statuses());
        assert!(
            issues.iter().any(|issue| issue.code() == expected_code),
            "owner/blocker 漂移必须报告 {expected_code:?}，实际={issues:?}"
        );
    }
}

#[test]
fn 矩阵拒绝重复遗漏未知映射漂移与实现授权() {
    let source_index = parse_source_index(VALID_SOURCE_INDEX).expect("测试 source index 应可解析");
    let cases = [
        (
            format!(
                "{VALID_MATRIX}{}",
                VALID_MATRIX.split_once("sources:\n").unwrap().1
            ),
            ValidationCode::DuplicateAdmissionSource,
        ),
        (
            VALID_MATRIX.replace(
                "  - source_id: tauri-command:save_settings\n",
                "  - source_id: tauri-command:unknown\n",
            ),
            ValidationCode::UnexpectedAdmissionSource,
        ),
        (
            VALID_MATRIX.replace(
                "feature.foundation-platform.settings-management",
                "feature.foundation-platform.diagnostics",
            ),
            ValidationCode::AdmissionFeatureMismatch,
        ),
        (
            VALID_MATRIX.replace("bucket: write", "bucket: network"),
            ValidationCode::AdmissionBucketMismatch,
        ),
        (
            VALID_MATRIX.replace(
                "implementation_authorized: false",
                "implementation_authorized: true",
            ),
            ValidationCode::AdmissionUnauthorized,
        ),
        (
            format!(
                "{}sources: []\n",
                VALID_MATRIX.split_once("sources:\n").unwrap().0
            ),
            ValidationCode::MissingAdmissionSource,
        ),
    ];

    for (input, expected_code) in cases {
        let matrix = parse_side_effect_admission_matrix(&input).expect("结构变异仍应可解析");
        let issues =
            validate_side_effect_admission_matrix(&matrix, &source_index, &feature_statuses());
        assert!(
            issues.iter().any(|issue| issue.code() == expected_code),
            "矩阵变异必须报告 {expected_code:?}，实际={issues:?}"
        );
    }
}

#[test]
fn 矩阵拒绝非未评估_feature_与不完整阻断元数据() {
    let source_index = parse_source_index(VALID_SOURCE_INDEX).expect("测试 source index 应可解析");
    let matrix = parse_side_effect_admission_matrix(
        &VALID_MATRIX
            .replace("kinds: [filesystem-mutation]", "kinds: []")
            .replace("blocker_refs: ['issue:94']", "blocker_refs: []"),
    )
    .expect("空集合结构仍应可解析");
    let statuses = BTreeMap::from([(
        "feature.foundation-platform.settings-management".to_owned(),
        ParityStatus::Implemented,
    )]);
    let issues = validate_side_effect_admission_matrix(&matrix, &source_index, &statuses);

    assert!(
        issues
            .iter()
            .any(|issue| issue.code() == ValidationCode::AdmissionTargetStatusMismatch)
    );
    assert!(
        issues
            .iter()
            .any(|issue| issue.code() == ValidationCode::AdmissionMetadataInvalid)
    );
}
