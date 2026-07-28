use inputcodex_application::{
    ApplicationError, ErrorKind, LoadCompletion, LoadCoordinator, LoadState, LoadVersionStartup,
    RequestId, TransitionOutcome, VersionStartupPort, VersionStartupRequest,
};
use inputcodex_domain::{ApplicationVersion, StartupIntent, VersionStartupSnapshot};

#[derive(Clone)]
struct StubPort(Result<VersionStartupSnapshot, ApplicationError>);

impl VersionStartupPort for StubPort {
    fn load(
        &self,
        _request: &VersionStartupRequest,
    ) -> Result<VersionStartupSnapshot, ApplicationError> {
        self.0.clone()
    }
}

fn snapshot(intent: StartupIntent) -> VersionStartupSnapshot {
    VersionStartupSnapshot::new(
        ApplicationVersion::new("0.1.0".to_owned()).expect("构建版本合法"),
        intent,
    )
}

#[test]
fn 默认与展示更新意图都返回_ready_且永不返回_empty() {
    for intent in [StartupIntent::Default, StartupIntent::ShowUpdate] {
        let expected = snapshot(intent);
        let use_case = LoadVersionStartup::new(StubPort(Ok(expected.clone())));

        let completion = use_case.execute(&VersionStartupRequest);

        assert_eq!(completion, LoadCompletion::Ready(expected));
        assert!(!matches!(completion, LoadCompletion::Empty));
    }
}

#[test]
fn 非法显式启动配置保持_invalid_input_稳定错误() {
    let error = ApplicationError::invalid_input("INVALID_STARTUP_OPTION");
    let use_case = LoadVersionStartup::new(StubPort(Err(error)));

    assert_eq!(
        use_case.execute(&VersionStartupRequest),
        LoadCompletion::Failed(error)
    );
    assert_eq!(error.kind(), ErrorKind::InvalidInput);
    assert_eq!(error.code().as_str(), "INVALID_STARTUP_OPTION");
}

#[test]
fn 取消后的同步版本启动结果按协调器规则变为过期() {
    let request_id = RequestId::new(81);
    let use_case = LoadVersionStartup::new(StubPort(Ok(snapshot(StartupIntent::Default))));
    let mut coordinator = LoadCoordinator::default();
    coordinator.begin(request_id);
    assert_eq!(coordinator.cancel(request_id), TransitionOutcome::Applied);

    assert_eq!(
        coordinator.complete(request_id, use_case.execute(&VersionStartupRequest)),
        TransitionOutcome::Stale
    );
    assert!(matches!(
        coordinator.state(),
        LoadState::Cancelling { request_id: current } if *current == request_id
    ));
}

#[test]
fn 较旧请求的同步结果不会覆盖较新请求() {
    let old_request = RequestId::new(1);
    let current_request = RequestId::new(2);
    let use_case = LoadVersionStartup::new(StubPort(Ok(snapshot(StartupIntent::ShowUpdate))));
    let mut coordinator = LoadCoordinator::default();
    coordinator.begin(current_request);

    assert_eq!(
        coordinator.complete(old_request, use_case.execute(&VersionStartupRequest)),
        TransitionOutcome::Stale
    );
    assert!(matches!(
        coordinator.state(),
        LoadState::Loading { request_id } if *request_id == current_request
    ));
}
