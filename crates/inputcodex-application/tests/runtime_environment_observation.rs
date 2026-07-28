use std::{cell::Cell, rc::Rc};

use inputcodex_application::{
    ApplicationError, ErrorKind, LoadCompletion, LoadCoordinator, LoadState,
    ObserveRuntimeEnvironmentConflicts, RequestId, RuntimeEnvironmentObservationPort,
    RuntimeEnvironmentObservationRequest, TransitionOutcome,
};
use inputcodex_domain::RuntimeEnvironmentConflictObservation;

struct StubPort {
    result: Result<RuntimeEnvironmentConflictObservation, ApplicationError>,
    calls: Rc<Cell<usize>>,
}

impl StubPort {
    fn new(
        result: Result<RuntimeEnvironmentConflictObservation, ApplicationError>,
    ) -> (Self, Rc<Cell<usize>>) {
        let calls = Rc::new(Cell::new(0));
        (
            Self {
                result,
                calls: Rc::clone(&calls),
            },
            calls,
        )
    }
}

impl RuntimeEnvironmentObservationPort for StubPort {
    fn observe(
        &self,
        _request: &RuntimeEnvironmentObservationRequest,
    ) -> Result<RuntimeEnvironmentConflictObservation, ApplicationError> {
        self.calls.set(self.calls.get() + 1);
        self.result.clone()
    }
}

#[test]
fn 零冲突返回_ready_而不是_empty() {
    let expected = RuntimeEnvironmentConflictObservation::new(3, Vec::new());
    let (port, calls) = StubPort::new(Ok(expected.clone()));
    let use_case = ObserveRuntimeEnvironmentConflicts::new(port);

    let completion = use_case.execute(&RuntimeEnvironmentObservationRequest);

    assert_eq!(completion, LoadCompletion::Ready(expected));
    assert!(!matches!(completion, LoadCompletion::Empty));
    assert_eq!(calls.get(), 1);
}

#[test]
fn 平台失败保持稳定错误() {
    let error = ApplicationError::unsupported("RUNTIME_ENVIRONMENT_OBSERVATION_UNSUPPORTED");
    let (port, calls) = StubPort::new(Err(error));
    let use_case = ObserveRuntimeEnvironmentConflicts::new(port);

    assert_eq!(
        use_case.execute(&RuntimeEnvironmentObservationRequest),
        LoadCompletion::Failed(error)
    );
    assert_eq!(calls.get(), 1);
}

#[test]
fn 超时构造器保持_timeout_错误种类() {
    let error = ApplicationError::timeout("RUNTIME_ENVIRONMENT_OBSERVATION_TIMEOUT");

    assert_eq!(error.kind(), ErrorKind::Timeout);
    assert_eq!(
        error.code().as_str(),
        "RUNTIME_ENVIRONMENT_OBSERVATION_TIMEOUT"
    );
}

#[test]
fn 取消后迟到结果失效且不会触发第二次平台调用() {
    let expected = RuntimeEnvironmentConflictObservation::new(1, Vec::new());
    let (port, calls) = StubPort::new(Ok(expected));
    let use_case = ObserveRuntimeEnvironmentConflicts::new(port);
    let request_id = RequestId::new(41);
    let mut coordinator = LoadCoordinator::default();
    coordinator.begin(request_id);
    assert_eq!(coordinator.cancel(request_id), TransitionOutcome::Applied);

    let late = use_case.execute(&RuntimeEnvironmentObservationRequest);

    assert_eq!(
        coordinator.complete(request_id, late),
        TransitionOutcome::Stale
    );
    assert_eq!(calls.get(), 1);
    assert!(matches!(
        coordinator.state(),
        LoadState::Cancelling { request_id: current } if *current == request_id
    ));
}

#[test]
fn 超时后迟到结果失效且错误不被空结果覆盖() {
    let expected = RuntimeEnvironmentConflictObservation::new(1, Vec::new());
    let (port, calls) = StubPort::new(Ok(expected));
    let use_case = ObserveRuntimeEnvironmentConflicts::new(port);
    let request_id = RequestId::new(51);
    let timeout = ApplicationError::timeout("RUNTIME_ENVIRONMENT_OBSERVATION_TIMEOUT");
    let mut coordinator = LoadCoordinator::default();
    coordinator.begin(request_id);
    assert_eq!(
        coordinator.complete(request_id, LoadCompletion::Failed(timeout)),
        TransitionOutcome::Applied
    );

    let late = use_case.execute(&RuntimeEnvironmentObservationRequest);

    assert_eq!(
        coordinator.complete(request_id, late),
        TransitionOutcome::Stale
    );
    assert_eq!(calls.get(), 1);
    assert_eq!(
        coordinator.state(),
        &LoadState::Failed {
            request_id,
            error: timeout,
        }
    );
}

#[test]
fn 旧请求结果不能覆盖新请求() {
    let old_value = RuntimeEnvironmentConflictObservation::new(1, Vec::new());
    let (old_port, old_calls) = StubPort::new(Ok(old_value));
    let old_use_case = ObserveRuntimeEnvironmentConflicts::new(old_port);
    let new_value = RuntimeEnvironmentConflictObservation::new(2, Vec::new());
    let (new_port, new_calls) = StubPort::new(Ok(new_value.clone()));
    let new_use_case = ObserveRuntimeEnvironmentConflicts::new(new_port);
    let old_request = RequestId::new(61);
    let new_request = RequestId::new(62);
    let mut coordinator = LoadCoordinator::default();
    coordinator.begin(old_request);
    let old_completion = old_use_case.execute(&RuntimeEnvironmentObservationRequest);
    coordinator.begin(new_request);

    assert_eq!(
        coordinator.complete(old_request, old_completion),
        TransitionOutcome::Stale
    );
    assert_eq!(
        coordinator.complete(
            new_request,
            new_use_case.execute(&RuntimeEnvironmentObservationRequest)
        ),
        TransitionOutcome::Applied
    );
    assert_eq!(old_calls.get(), 1);
    assert_eq!(new_calls.get(), 1);
    assert_eq!(
        coordinator.state(),
        &LoadState::Ready {
            request_id: new_request,
            value: new_value,
        }
    );
}
