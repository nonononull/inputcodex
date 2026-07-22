use std::collections::BTreeSet;

use inputcodex_parity::{
    FeatureDomain, LoadingState, ValidationCode, parse_contract_catalog, validate_contract_catalog,
    validate_contract_catalog_domain,
};

const VALID_CONTRACT: &str = r#"
schema_version: inputcodex.behavior-contract.v1
domain: foundation-platform
contracts:
  - id: contract.feature.foundation-platform.application-detection.default
    feature_id: feature.foundation-platform.application-detection
    scenario: default
    preconditions:
      - 本地环境可读取应用安装信息。
    inputs:
      - name: refresh
        data_type: boolean
        required: false
        description: 是否忽略已有只读扫描结果。
    outputs:
      - name: applications
        data_type: application-list
        description: 已识别应用及安装状态。
    side_effects:
      - none
    persistence: 不持久化私人路径或凭据。
    errors:
      - code: PLATFORM_UNSUPPORTED
        kind: unsupported
        semantics: 平台不支持时明确失败，不伪造空结果。
    loading:
      states: [Idle, Loading, Ready, Empty, Failed, Cancelling]
      request_identity: 每次扫描使用单调递增请求标识。
      stale_result_rule: 旧请求结果不得覆盖新请求状态。
    timeout:
      behavior: 超时后进入 Failed 并保留诊断上下文。
      evidence: 记录超时类别和请求标识，不记录私人路径。
    cancellation:
      behavior: 取消后当前结果失效。
      evidence: 记录取消完成与旧请求失效。
    isolation: 单个应用探测失败不得终止其他探测项。
    observability:
      - 请求标识
      - 扫描总数
      - 失败分类
    platforms:
      shared_semantics: Windows 与 macOS 都返回相同字段和错误分类。
      windows: 使用 Windows 平台适配器读取安装事实。
      macos: 使用 macOS 平台适配器读取安装事实。
      differences: []
    fixture_policy:
      mode: none
      reason: 应用扫描由平台适配器产生，不需要结构化 fixture。
    fixture_refs: []
"#;

#[test]
fn 合同覆盖六种加载状态和请求失效语义() {
    let contracts = parse_contract_catalog(VALID_CONTRACT).expect("合法合同应可解析");
    let feature_ids =
        BTreeSet::from(["feature.foundation-platform.application-detection".to_owned()]);
    let fixture_ids = BTreeSet::new();

    assert_eq!(
        contracts.contracts()[0].loading().states(),
        &[
            LoadingState::Idle,
            LoadingState::Loading,
            LoadingState::Ready,
            LoadingState::Empty,
            LoadingState::Failed,
            LoadingState::Cancelling,
        ]
    );
    assert!(validate_contract_catalog(&contracts, &feature_ids, &fixture_ids).is_empty());
}

#[test]
fn 合同文件_domain_必须与目标领域一致() {
    let contracts = parse_contract_catalog(VALID_CONTRACT).expect("合法合同应可解析");

    assert!(
        validate_contract_catalog_domain(&contracts, FeatureDomain::ProviderNetwork)
            .iter()
            .any(|issue| issue.code() == ValidationCode::ContractDomainMismatch)
    );
}

#[test]
fn fixture_policy_必须与引用状态一致() {
    let invalid = VALID_CONTRACT.replace("      mode: none", "      mode: required");
    let contracts = parse_contract_catalog(&invalid).expect("结构仍应可解析");
    let feature_ids =
        BTreeSet::from(["feature.foundation-platform.application-detection".to_owned()]);

    assert!(
        validate_contract_catalog(&contracts, &feature_ids, &BTreeSet::new())
            .iter()
            .any(|issue| issue.code() == ValidationCode::FixturePolicyMismatch)
    );
}

#[test]
fn 缺少_cancelling_状态时合同验证失败() {
    let invalid = VALID_CONTRACT.replace(
        "states: [Idle, Loading, Ready, Empty, Failed, Cancelling]",
        "states: [Idle, Loading, Ready, Empty, Failed]",
    );
    let contracts = parse_contract_catalog(&invalid).expect("结构仍应可解析");
    let feature_ids =
        BTreeSet::from(["feature.foundation-platform.application-detection".to_owned()]);

    assert!(
        validate_contract_catalog(&contracts, &feature_ids, &BTreeSet::new())
            .iter()
            .any(|issue| issue.code() == ValidationCode::MissingLoadingState)
    );
}

#[test]
fn 悬空_feature_引用被拒绝() {
    let contracts = parse_contract_catalog(VALID_CONTRACT).expect("合法合同应可解析");

    assert!(
        validate_contract_catalog(&contracts, &BTreeSet::new(), &BTreeSet::new())
            .iter()
            .any(|issue| issue.code() == ValidationCode::DanglingFeatureReference)
    );
}

#[test]
fn 重复合同_id_被拒绝() {
    let entry = VALID_CONTRACT
        .split_once("contracts:\n")
        .expect("测试合同必须包含 contracts")
        .1;
    let invalid = format!("{VALID_CONTRACT}{entry}");
    let contracts = parse_contract_catalog(&invalid).expect("重复合同仍应可解析");
    let feature_ids =
        BTreeSet::from(["feature.foundation-platform.application-detection".to_owned()]);

    assert!(
        validate_contract_catalog(&contracts, &feature_ids, &BTreeSet::new())
            .iter()
            .any(|issue| issue.code() == ValidationCode::DuplicateContractId)
    );
}

#[test]
fn 合同_id_必须与_feature_和_scenario_一致() {
    let invalid = VALID_CONTRACT.replace(
        "contract.feature.foundation-platform.application-detection.default",
        "contract.feature.foundation-platform.application-detection.alternate",
    );
    let contracts = parse_contract_catalog(&invalid).expect("结构仍应可解析");
    let feature_ids =
        BTreeSet::from(["feature.foundation-platform.application-detection".to_owned()]);

    assert!(
        validate_contract_catalog(&contracts, &feature_ids, &BTreeSet::new())
            .iter()
            .any(|issue| issue.code() == ValidationCode::ContractIdentityMismatch)
    );
}

#[test]
fn 缺少请求标识语义时解析失败() {
    let invalid = VALID_CONTRACT.replace(
        "      request_identity: 每次扫描使用单调递增请求标识。\n",
        "",
    );

    assert!(parse_contract_catalog(&invalid).is_err());
}

#[test]
fn 悬空_fixture_引用被拒绝() {
    let invalid = VALID_CONTRACT.replace(
        "    fixture_refs: []",
        "    fixture_refs:\n      - fixture.feature.foundation-platform.application-detection.synthetic-result",
    );
    let contracts = parse_contract_catalog(&invalid).expect("结构仍应可解析");
    let feature_ids =
        BTreeSet::from(["feature.foundation-platform.application-detection".to_owned()]);

    assert!(
        validate_contract_catalog(&contracts, &feature_ids, &BTreeSet::new())
            .iter()
            .any(|issue| issue.code() == ValidationCode::DanglingFixtureReference)
    );
}

#[test]
fn 跨_feature_fixture_引用被拒绝() {
    let fixture_id = "fixture.feature.session-data.session-list.synthetic-page";
    let invalid = VALID_CONTRACT.replace(
        "    fixture_refs: []",
        &format!("    fixture_refs:\n      - {fixture_id}"),
    );
    let contracts = parse_contract_catalog(&invalid).expect("结构仍应可解析");
    let feature_ids =
        BTreeSet::from(["feature.foundation-platform.application-detection".to_owned()]);
    let fixture_ids = BTreeSet::from([fixture_id.to_owned()]);

    assert!(
        validate_contract_catalog(&contracts, &feature_ids, &fixture_ids)
            .iter()
            .any(|issue| issue.code() == ValidationCode::CrossFeatureFixtureReference)
    );
}

#[test]
fn 缺少_macos_合同语义时解析失败() {
    let invalid = VALID_CONTRACT.replace("      macos: 使用 macOS 平台适配器读取安装事实。\n", "");

    assert!(parse_contract_catalog(&invalid).is_err());
}

#[test]
fn 行为合同必填段不可缺失() {
    let required_fragments = [
        (
            "schema_version",
            "schema_version: inputcodex.behavior-contract.v1\n",
        ),
        ("domain", "domain: foundation-platform\n"),
        (
            "id",
            "  - id: contract.feature.foundation-platform.application-detection.default\n",
        ),
        (
            "preconditions",
            "    preconditions:\n      - 本地环境可读取应用安装信息。\n",
        ),
        (
            "inputs",
            "    inputs:\n      - name: refresh\n        data_type: boolean\n        required: false\n        description: 是否忽略已有只读扫描结果。\n",
        ),
        (
            "outputs",
            "    outputs:\n      - name: applications\n        data_type: application-list\n        description: 已识别应用及安装状态。\n",
        ),
        ("side_effects", "    side_effects:\n      - none\n"),
        ("persistence", "    persistence: 不持久化私人路径或凭据。\n"),
        (
            "errors",
            "    errors:\n      - code: PLATFORM_UNSUPPORTED\n        kind: unsupported\n        semantics: 平台不支持时明确失败，不伪造空结果。\n",
        ),
        (
            "timeout",
            "    timeout:\n      behavior: 超时后进入 Failed 并保留诊断上下文。\n      evidence: 记录超时类别和请求标识，不记录私人路径。\n",
        ),
        (
            "cancellation",
            "    cancellation:\n      behavior: 取消后当前结果失效。\n      evidence: 记录取消完成与旧请求失效。\n",
        ),
        (
            "isolation",
            "    isolation: 单个应用探测失败不得终止其他探测项。\n",
        ),
        (
            "observability",
            "    observability:\n      - 请求标识\n      - 扫描总数\n      - 失败分类\n",
        ),
        (
            "fixture_policy",
            "    fixture_policy:\n      mode: none\n      reason: 应用扫描由平台适配器产生，不需要结构化 fixture。\n",
        ),
        ("fixture_refs", "    fixture_refs: []\n"),
    ];

    for (field, fragment) in required_fragments {
        assert!(
            VALID_CONTRACT.contains(fragment),
            "测试片段 {field} 必须存在"
        );
        let invalid = VALID_CONTRACT.replace(fragment, "");
        assert!(
            parse_contract_catalog(&invalid).is_err(),
            "缺少 {field} 时必须拒绝解析"
        );
    }
}
