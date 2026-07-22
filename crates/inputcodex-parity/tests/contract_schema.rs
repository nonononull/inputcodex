use std::collections::BTreeSet;

use inputcodex_parity::{
    LoadingState, ValidationCode, parse_contract_catalog, validate_contract_catalog,
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
    fixture_refs: []
"#;

#[test]
fn 合同覆盖六种加载状态和请求失效语义() {
    let contracts = parse_contract_catalog(VALID_CONTRACT).expect("合法合同应可解析");
    let feature_ids = BTreeSet::from([
        "feature.foundation-platform.application-detection".to_owned(),
    ]);
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
    assert!(
        validate_contract_catalog(&contracts, &feature_ids, &fixture_ids).is_empty()
    );
}

#[test]
fn 缺少_cancelling_状态时合同验证失败() {
    let invalid = VALID_CONTRACT.replace(
        "states: [Idle, Loading, Ready, Empty, Failed, Cancelling]",
        "states: [Idle, Loading, Ready, Empty, Failed]",
    );
    let contracts = parse_contract_catalog(&invalid).expect("结构仍应可解析");
    let feature_ids = BTreeSet::from([
        "feature.foundation-platform.application-detection".to_owned(),
    ]);

    assert!(validate_contract_catalog(&contracts, &feature_ids, &BTreeSet::new())
        .iter()
        .any(|issue| issue.code() == ValidationCode::MissingLoadingState));
}

#[test]
fn 悬空_feature_引用被拒绝() {
    let contracts = parse_contract_catalog(VALID_CONTRACT).expect("合法合同应可解析");

    assert!(validate_contract_catalog(&contracts, &BTreeSet::new(), &BTreeSet::new())
        .iter()
        .any(|issue| issue.code() == ValidationCode::DanglingFeatureReference));
}
