use inputcodex_parity::{
    FeatureDomain, ParityStatus, ValidationCode, parse_feature_catalog, validate_feature_catalog,
};

const VALID_CATALOG: &str = r#"
schema_version: inputcodex.feature-catalog.v1
release:
  tag: v1.2.41
  tag_commit: 3dafffcafb2566a1e8bce4b35671656d6adb3eda
domain: foundation-platform
features:
  - id: feature.foundation-platform.application-detection
    name: 应用检测
    status: unassessed
    evidence:
      - path: upstream/CodexPlusPlus/apps/codex-plus-manager/src-tauri/src/commands.rs
        symbol: detect_installed_apps
    entry_points:
      - tauri:detect_installed_apps
    platforms:
      windows:
        applicability: supported
        semantics: 返回可识别的本机应用安装状态。
      macos:
        applicability: supported
        semantics: 返回可识别的本机应用安装状态。
    decision_refs:
      - issue:26
"#;

#[test]
fn 合法功能目录可解析为稳定类型() {
    let catalog = parse_feature_catalog(VALID_CATALOG).expect("合法目录应可解析");

    assert_eq!(catalog.domain(), FeatureDomain::FoundationPlatform);
    assert_eq!(catalog.features().len(), 1);
    assert_eq!(
        catalog.features()[0].id(),
        "feature.foundation-platform.application-detection"
    );
    assert_eq!(catalog.features()[0].status(), ParityStatus::Unassessed);
    assert!(validate_feature_catalog(&catalog).is_empty());
}

#[test]
fn 功能_id_必须与文件_domain_一致() {
    let invalid = VALID_CATALOG.replace(
        "feature.foundation-platform.application-detection",
        "feature.provider-network.application-detection",
    );
    let catalog = parse_feature_catalog(&invalid).expect("结构仍应可解析");
    let issues = validate_feature_catalog(&catalog);

    assert!(issues.iter().any(|issue| {
        issue.code() == ValidationCode::FeatureDomainMismatch
            && issue.location().contains("application-detection")
    }));
}

#[test]
fn 未批准的一致性状态在解析阶段被拒绝() {
    let invalid = VALID_CATALOG.replace("status: unassessed", "status: skipped");

    assert!(parse_feature_catalog(&invalid).is_err());
}

#[test]
fn 功能_id_必须使用完整格式() {
    let invalid = VALID_CATALOG.replace(
        "feature.foundation-platform.application-detection",
        "foundation-platform.application-detection",
    );
    let catalog = parse_feature_catalog(&invalid).expect("结构仍应可解析");

    assert!(
        validate_feature_catalog(&catalog)
            .iter()
            .any(|issue| issue.code() == ValidationCode::InvalidFeatureId)
    );
}

#[test]
fn 重复功能_id_被拒绝() {
    let entry = VALID_CATALOG
        .split_once("features:\n")
        .expect("测试目录必须包含 features")
        .1;
    let invalid = format!("{VALID_CATALOG}{entry}");
    let catalog = parse_feature_catalog(&invalid).expect("重复项仍应可解析");

    assert!(
        validate_feature_catalog(&catalog)
            .iter()
            .any(|issue| issue.code() == ValidationCode::DuplicateFeatureId)
    );
}

#[test]
fn 缺少功能名称时解析失败() {
    let invalid = VALID_CATALOG.replace("    name: 应用检测\n", "");

    assert!(parse_feature_catalog(&invalid).is_err());
}

#[test]
fn 缺少_macos_平台语义时解析失败() {
    let invalid = VALID_CATALOG.replace(
        "      macos:\n        applicability: supported\n        semantics: 返回可识别的本机应用安装状态。\n",
        "",
    );

    assert!(parse_feature_catalog(&invalid).is_err());
}

#[test]
fn 功能目录必填元数据不可缺失() {
    let required_fragments = [
        (
            "schema_version",
            "schema_version: inputcodex.feature-catalog.v1\n",
        ),
        (
            "release",
            "release:\n  tag: v1.2.41\n  tag_commit: 3dafffcafb2566a1e8bce4b35671656d6adb3eda\n",
        ),
        (
            "evidence",
            "    evidence:\n      - path: upstream/CodexPlusPlus/apps/codex-plus-manager/src-tauri/src/commands.rs\n        symbol: detect_installed_apps\n",
        ),
        (
            "entry_points",
            "    entry_points:\n      - tauri:detect_installed_apps\n",
        ),
        ("decision_refs", "    decision_refs:\n      - issue:26\n"),
    ];

    for (field, fragment) in required_fragments {
        assert!(
            VALID_CATALOG.contains(fragment),
            "测试片段 {field} 必须存在"
        );
        let invalid = VALID_CATALOG.replace(fragment, "");
        assert!(
            parse_feature_catalog(&invalid).is_err(),
            "缺少 {field} 时必须拒绝解析"
        );
    }
}
