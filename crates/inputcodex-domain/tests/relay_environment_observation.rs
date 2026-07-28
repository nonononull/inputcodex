use inputcodex_domain::{
    ClashConfigSource, ClashTunCandidateStatus, ClashTunObservation, CodexDotenvStatus,
    ObservationCoverageStatus, ProxyEnvironmentCoverage, ProxyEnvironmentSource,
    ProxyEnvironmentVariableName, ProxyEnvironmentVariableObservation, RelayEnvironmentObservation,
};

#[test]
fn 代理环境变量名称大小写不敏感并输出固定规范名() {
    let cases = [
        ("http_proxy", "HTTP_PROXY"),
        ("HTTPS_PROXY", "HTTPS_PROXY"),
        ("All_Proxy", "ALL_PROXY"),
        ("no_proxy", "NO_PROXY"),
        ("FtP_pRoXy", "FTP_PROXY"),
    ];

    for (input, expected) in cases {
        let name =
            ProxyEnvironmentVariableName::from_name(input).expect("标准代理环境变量名称应被识别");
        assert_eq!(name.as_str(), expected);
    }
}

#[test]
fn 非代理环境变量名称被拒绝() {
    assert_eq!(
        ProxyEnvironmentVariableName::from_name("OPENAI_API_KEY"),
        None
    );
    assert_eq!(ProxyEnvironmentVariableName::from_name("HTTP_PROXY "), None);
    assert_eq!(ProxyEnvironmentVariableName::from_name(""), None);
}

#[test]
fn 来源覆盖明确区分已观察未观察和不可用() {
    let coverage = ProxyEnvironmentCoverage::new(
        ObservationCoverageStatus::Observed,
        ObservationCoverageStatus::NotObserved,
        ObservationCoverageStatus::Unavailable,
    );

    assert_eq!(
        coverage.runtime_process(),
        ObservationCoverageStatus::Observed
    );
    assert_eq!(
        coverage.persistent_user(),
        ObservationCoverageStatus::NotObserved
    );
    assert_eq!(
        coverage.persistent_system(),
        ObservationCoverageStatus::Unavailable
    );
}

#[test]
fn 单个代理变量的多个来源稳定排序并去重() {
    let variable = ProxyEnvironmentVariableObservation::new(
        ProxyEnvironmentVariableName::HttpsProxy,
        vec![
            ProxyEnvironmentSource::PersistentSystem,
            ProxyEnvironmentSource::RuntimeProcess,
            ProxyEnvironmentSource::RuntimeProcess,
            ProxyEnvironmentSource::PersistentUser,
        ],
    );

    assert_eq!(variable.name(), ProxyEnvironmentVariableName::HttpsProxy);
    assert_eq!(
        variable.sources(),
        &[
            ProxyEnvironmentSource::RuntimeProcess,
            ProxyEnvironmentSource::PersistentUser,
            ProxyEnvironmentSource::PersistentSystem,
        ]
    );
}

#[test]
fn clash_tun_固定四个逻辑来源并区分五种状态() {
    let observation = ClashTunObservation::new(
        ClashTunCandidateStatus::Absent,
        ClashTunCandidateStatus::Disabled,
        ClashTunCandidateStatus::Enabled,
        ClashTunCandidateStatus::Invalid,
    );

    assert_eq!(
        observation.status(ClashConfigSource::PlatformData),
        ClashTunCandidateStatus::Absent
    );
    assert_eq!(
        observation.status(ClashConfigSource::PlatformConfig),
        ClashTunCandidateStatus::Disabled
    );
    assert_eq!(
        observation.status(ClashConfigSource::HomeAppConfig),
        ClashTunCandidateStatus::Enabled
    );
    assert_eq!(
        observation.status(ClashConfigSource::HomeLegacyConfig),
        ClashTunCandidateStatus::Invalid
    );
    assert!(observation.has_enabled_tun());
    assert!(observation.has_incomplete_candidate());
}

#[test]
fn 聚合结果合并同名变量并派生风险与观察缺口() {
    let observation = RelayEnvironmentObservation::new(
        vec![
            ProxyEnvironmentVariableObservation::new(
                ProxyEnvironmentVariableName::HttpsProxy,
                vec![ProxyEnvironmentSource::PersistentSystem],
            ),
            ProxyEnvironmentVariableObservation::new(
                ProxyEnvironmentVariableName::HttpProxy,
                vec![ProxyEnvironmentSource::RuntimeProcess],
            ),
            ProxyEnvironmentVariableObservation::new(
                ProxyEnvironmentVariableName::HttpsProxy,
                vec![ProxyEnvironmentSource::PersistentUser],
            ),
        ],
        ProxyEnvironmentCoverage::new(
            ObservationCoverageStatus::Observed,
            ObservationCoverageStatus::Observed,
            ObservationCoverageStatus::Unavailable,
        ),
        CodexDotenvStatus::Present,
        ClashTunObservation::new(
            ClashTunCandidateStatus::Absent,
            ClashTunCandidateStatus::Unreadable,
            ClashTunCandidateStatus::Enabled,
            ClashTunCandidateStatus::Disabled,
        ),
    );

    assert_eq!(observation.proxy_variables().len(), 2);
    assert_eq!(
        observation.proxy_variables()[0].name(),
        ProxyEnvironmentVariableName::HttpProxy
    );
    assert_eq!(
        observation.proxy_variables()[1].sources(),
        &[
            ProxyEnvironmentSource::PersistentUser,
            ProxyEnvironmentSource::PersistentSystem,
        ]
    );
    assert_eq!(observation.codex_dotenv(), CodexDotenvStatus::Present);
    assert!(observation.has_detected_risk());
    assert!(observation.has_observation_gap());
}

#[test]
fn 零风险完整观察仍形成有效报告() {
    let observation = RelayEnvironmentObservation::new(
        Vec::new(),
        ProxyEnvironmentCoverage::new(
            ObservationCoverageStatus::Observed,
            ObservationCoverageStatus::Observed,
            ObservationCoverageStatus::Observed,
        ),
        CodexDotenvStatus::Absent,
        ClashTunObservation::new(
            ClashTunCandidateStatus::Absent,
            ClashTunCandidateStatus::Absent,
            ClashTunCandidateStatus::Absent,
            ClashTunCandidateStatus::Absent,
        ),
    );

    assert!(observation.proxy_variables().is_empty());
    assert!(!observation.has_detected_risk());
    assert!(!observation.has_observation_gap());
}
