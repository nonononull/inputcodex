use std::{cell::Cell, mem::size_of, rc::Rc};

use inputcodex_application::{
    ApplicationError, LoadCompletion, LoadCoordinator, LoadState, ObserveRelayStatus,
    RelayStatusObservationPort, RelayStatusObservationRequest, RequestId, TransitionOutcome,
};
use inputcodex_domain::{
    CredentialPresence, RelayConfigurationStatus, RelayDocumentStatus, RelayStatusObservation,
};

struct StubPort {
    result: Result<Option<RelayStatusObservation>, ApplicationError>,
    calls: Rc<Cell<usize>>,
}

impl StubPort {
    fn new(
        result: Result<Option<RelayStatusObservation>, ApplicationError>,
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

impl RelayStatusObservationPort for StubPort {
    fn observe(
        &self,
        _request: &RelayStatusObservationRequest,
    ) -> Result<Option<RelayStatusObservation>, ApplicationError> {
        self.calls.set(self.calls.get() + 1);
        self.result
    }
}

fn observed_status() -> RelayStatusObservation {
    RelayStatusObservation::new(
        RelayDocumentStatus::Valid,
        RelayDocumentStatus::Valid,
        CredentialPresence::Present,
        CredentialPresence::Absent,
        RelayConfigurationStatus::Complete,
    )
}

#[test]
fn 固定文档事实返回_ready() {
    let expected = observed_status();
    let (port, calls) = StubPort::new(Ok(Some(expected)));
    let use_case = ObserveRelayStatus::new(port);

    assert_eq!(
        use_case.execute(&RelayStatusObservationRequest),
        LoadCompletion::Ready(expected)
    );
    assert_eq!(calls.get(), 1);
}

#[test]
fn 两份固定文档均缺失返回_empty() {
    let (port, calls) = StubPort::new(Ok(None));
    let use_case = ObserveRelayStatus::new(port);

    assert_eq!(
        use_case.execute(&RelayStatusObservationRequest),
        LoadCompletion::Empty
    );
    assert_eq!(calls.get(), 1);
}

#[test]
fn 平台错误保持_failed_和稳定诊断码() {
    let error = ApplicationError::unsupported("RELAY_STATUS_OBSERVATION_UNSUPPORTED");
    let (port, calls) = StubPort::new(Err(error));
    let use_case = ObserveRelayStatus::new(port);

    assert_eq!(
        use_case.execute(&RelayStatusObservationRequest),
        LoadCompletion::Failed(error)
    );
    assert_eq!(calls.get(), 1);
}

#[test]
fn 请求保持零字段且不可携带路径或读取策略() {
    fn assert_default<T: Default>() {}

    assert_eq!(size_of::<RelayStatusObservationRequest>(), 0);
    assert_default::<RelayStatusObservationRequest>();
}

#[test]
fn 取消后的迟到_relay_结果保持_stale() {
    let expected = observed_status();
    let (port, calls) = StubPort::new(Ok(Some(expected)));
    let use_case = ObserveRelayStatus::new(port);
    let request_id = RequestId::new(98);
    let mut coordinator = LoadCoordinator::default();
    coordinator.begin(request_id);
    assert_eq!(coordinator.cancel(request_id), TransitionOutcome::Applied);

    let late = use_case.execute(&RelayStatusObservationRequest);

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
