use inputcodex_parity::{
    FeatureDomain, ParityStatus, ValidationCode, parse_feature_catalog,
    validate_feature_catalog,
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
