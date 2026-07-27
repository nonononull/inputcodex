use inputcodex_application::{
    ApplicationError, ErrorKind, LoadCoordinator, LoadState, PlatformPathsPort,
    PlatformPathsRequest, RequestId, ResolvePlatformPaths, TransitionOutcome,
};
use inputcodex_domain::{PlatformPathsSnapshot, PrivatePath};

#[derive(Clone)]
struct StubPort(Result<PlatformPathsSnapshot, ApplicationError>);

impl PlatformPathsPort for StubPort {
    fn resolve(
        &self,
        _request: &PlatformPathsRequest,
    ) -> Result<PlatformPathsSnapshot, ApplicationError> {
        self.0.clone()
    }
}

fn empty_snapshot() -> PlatformPathsSnapshot {
    let root = std::env::temp_dir().join("inputcodex-application-paths");
    let private = |name: &str| PrivatePath::new(root.join(name)).expect("绝对路径");
    PlatformPathsSnapshot::new(
        private(".codex"),
        private("state"),
        private("state/settings.json"),
        private("state/latest-status.json"),
        private("state/inputcodex.log"),
        None,
    )
}

#[test]
fn 未安装返回_ready_快照而不是_empty() {
    let use_case = ResolvePlatformPaths::new(StubPort(Ok(empty_snapshot())));

    assert!(matches!(
        use_case.execute(&PlatformPathsRequest::default()),
        inputcodex_application::LoadCompletion::Ready(snapshot)
            if snapshot.codex_installation().is_none()
    ));
}

#[test]
fn 失败保持稳定分类且请求_debug_不泄露路径() {
    let path = std::env::temp_dir().join("private/Codex.app");
    let request = PlatformPathsRequest::new(Some(path.clone()));
    assert!(!format!("{request:?}").contains(&path.to_string_lossy().to_string()));

    let use_case = ResolvePlatformPaths::new(StubPort(Err(ApplicationError::unavailable(
        "EXPLICIT_CODEX_PATH_INVALID",
    ))));
    let completion = use_case.execute(&request);
    assert!(matches!(
        completion,
        inputcodex_application::LoadCompletion::Failed(error)
            if error.kind() == ErrorKind::Unavailable
                && error.code().as_str() == "EXPLICIT_CODEX_PATH_INVALID"
    ));
}

#[test]
fn 取消后的同步结果按现有协调器规则变为过期() {
    let request_id = RequestId::new(7);
    let use_case = ResolvePlatformPaths::new(StubPort(Ok(empty_snapshot())));
    let completion = use_case.execute(&PlatformPathsRequest::default());
    let mut coordinator = LoadCoordinator::default();
    coordinator.begin(request_id);

    assert_eq!(coordinator.cancel(request_id), TransitionOutcome::Applied);
    assert_eq!(
        coordinator.complete(request_id, completion),
        TransitionOutcome::Stale
    );
    assert!(matches!(
        coordinator.state(),
        LoadState::Cancelling {
            request_id: current
        } if *current == request_id
    ));
}
