use inputcodex_application::{
    ApplicationError, ErrorKind, LoadCompletion, LoadCoordinator, LoadState, RequestId,
    TransitionOutcome,
};
use inputcodex_domain::DiagnosticCode;

#[test]
fn 过期结果不能覆盖新请求() {
    let mut coordinator = LoadCoordinator::<String>::default();
    let first = RequestId::new(1);
    let second = RequestId::new(2);

    coordinator.begin(first);
    coordinator.begin(second);

    assert_eq!(
        coordinator.complete(first, LoadCompletion::Ready("旧结果".to_owned())),
        TransitionOutcome::Stale
    );
    assert_eq!(
        coordinator.state(),
        &LoadState::Loading { request_id: second }
    );

    assert_eq!(
        coordinator.complete(second, LoadCompletion::Ready("新结果".to_owned())),
        TransitionOutcome::Applied
    );
    assert_eq!(
        coordinator.state(),
        &LoadState::Ready {
            request_id: second,
            value: "新结果".to_owned(),
        }
    );
}

#[test]
fn 取消后的结果必须失效() {
    let mut coordinator = LoadCoordinator::<String>::default();
    let request_id = RequestId::new(7);

    coordinator.begin(request_id);
    assert_eq!(coordinator.cancel(request_id), TransitionOutcome::Applied);
    assert_eq!(coordinator.state(), &LoadState::Cancelling { request_id });

    assert_eq!(
        coordinator.complete(request_id, LoadCompletion::Ready("迟到".to_owned())),
        TransitionOutcome::Stale
    );
    assert_eq!(coordinator.state(), &LoadState::Cancelling { request_id });

    assert_eq!(
        coordinator.finish_cancellation(request_id),
        TransitionOutcome::Applied
    );
    assert_eq!(coordinator.state(), &LoadState::Idle);
}

#[test]
fn 空结果与失败结果具有稳定状态() {
    let mut coordinator = LoadCoordinator::<()>::default();
    let empty_request = RequestId::new(11);
    let failed_request = RequestId::new(12);
    let error = ApplicationError::new(
        ErrorKind::Unsupported,
        DiagnosticCode::new("PLATFORM_UNSUPPORTED"),
    );

    coordinator.begin(empty_request);
    assert_eq!(
        coordinator.complete(empty_request, LoadCompletion::Empty),
        TransitionOutcome::Applied
    );
    assert_eq!(
        coordinator.state(),
        &LoadState::Empty {
            request_id: empty_request,
        }
    );

    coordinator.begin(failed_request);
    assert_eq!(
        coordinator.complete(failed_request, LoadCompletion::Failed(error)),
        TransitionOutcome::Applied
    );
    assert_eq!(
        coordinator.state(),
        &LoadState::Failed {
            request_id: failed_request,
            error,
        }
    );
    assert_eq!(error.kind(), ErrorKind::Unsupported);
    assert_eq!(error.code().as_str(), "PLATFORM_UNSUPPORTED");
}
