use inputcodex_application::{
    ApplicationError, ApplicationOverviewPort, ApplicationOverviewRequest, LoadApplicationOverview,
    LoadCompletion, LoadCoordinator, LoadState, RequestId, TransitionOutcome,
};
use inputcodex_domain::{
    ApplicationInstallSource, ApplicationOverview, ApplicationVersion, CodexInstallation,
    CollectedAtUnixMs, InstallationState, InstalledVersion, InstalledVersionUnknownReason,
    LiveProcessState, PrivatePath,
};

#[derive(Clone)]
struct StubPort {
    result: Result<ApplicationOverview, ApplicationError>,
}

impl ApplicationOverviewPort for StubPort {
    fn load(
        &self,
        _request: &ApplicationOverviewRequest,
    ) -> Result<ApplicationOverview, ApplicationError> {
        self.result.clone()
    }
}

fn installed(version: InstalledVersion) -> ApplicationOverview {
    let root = std::env::temp_dir().join("inputcodex-application-overview");
    let installation = CodexInstallation::new(
        PrivatePath::new(root.clone()).expect("测试安装根必须是绝对路径"),
        PrivatePath::new(root.join("Codex.exe")).expect("测试可执行文件必须是绝对路径"),
        ApplicationInstallSource::Explicit,
    );
    ApplicationOverview::new(
        InstallationState::Installed {
            installation,
            version,
        },
        ApplicationVersion::new("0.1.0".to_owned()).expect("构建版本合法"),
        LiveProcessState::NotObserved,
        CollectedAtUnixMs::new(100),
    )
}

fn not_installed() -> ApplicationOverview {
    ApplicationOverview::new(
        InstallationState::NotInstalled,
        ApplicationVersion::new("0.1.0".to_owned()).expect("构建版本合法"),
        LiveProcessState::NotObserved,
        CollectedAtUnixMs::new(200),
    )
}

#[test]
fn 已安装已知与未知版本都返回_ready() {
    let known = installed(InstalledVersion::Known(
        ApplicationVersion::new("1.2.43".to_owned()).expect("安装版本合法"),
    ));
    let unknown = installed(InstalledVersion::Unknown(
        InstalledVersionUnknownReason::MetadataMissing,
    ));

    for expected in [known, unknown] {
        let use_case = LoadApplicationOverview::new(StubPort {
            result: Ok(expected.clone()),
        });

        assert_eq!(
            use_case.execute(&ApplicationOverviewRequest::default()),
            LoadCompletion::Ready(expected)
        );
    }
}

#[test]
fn 未安装返回_ready_且本功能永不映射为_empty() {
    let expected = not_installed();
    let use_case = LoadApplicationOverview::new(StubPort {
        result: Ok(expected.clone()),
    });

    let completion = use_case.execute(&ApplicationOverviewRequest::default());

    assert_eq!(completion, LoadCompletion::Ready(expected));
    assert!(!matches!(completion, LoadCompletion::Empty));
}

#[test]
fn 端口失败保持稳定错误且请求_debug_不泄露显式路径() {
    let explicit = std::env::temp_dir().join("private-codex-installation");
    let request = ApplicationOverviewRequest::new(Some(explicit.clone()));
    let error = ApplicationError::internal("APPLICATION_OVERVIEW_DISCOVERY_FAILED");
    let use_case = LoadApplicationOverview::new(StubPort { result: Err(error) });

    assert_eq!(use_case.execute(&request), LoadCompletion::Failed(error));
    let debug = format!("{request:?}");
    assert!(debug.contains("<redacted>"));
    assert!(!debug.contains(&explicit.to_string_lossy().to_string()));
}

#[test]
fn 取消后的同步概览结果按现有协调器规则变为过期() {
    let expected = not_installed();
    let use_case = LoadApplicationOverview::new(StubPort {
        result: Ok(expected.clone()),
    });
    let request_id = RequestId::new(41);
    let mut coordinator = LoadCoordinator::default();
    coordinator.begin(request_id);
    assert_eq!(coordinator.cancel(request_id), TransitionOutcome::Applied);

    assert_eq!(
        coordinator.complete(
            request_id,
            use_case.execute(&ApplicationOverviewRequest::default())
        ),
        TransitionOutcome::Stale
    );
    assert!(matches!(
        coordinator.state(),
        LoadState::Cancelling { request_id: current } if *current == request_id
    ));
}
