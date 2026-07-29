use std::{cell::Cell, rc::Rc};

use inputcodex_application::{
    ApplicationError, DiagnosticLogObservationPort, DiagnosticLogObservationRequest,
    LoadCompletion, LoadCoordinator, LoadState, ObserveDiagnosticLog, RequestId, TransitionOutcome,
};
use inputcodex_domain::DiagnosticLogObservation;

struct StubPort {
    result: Result<Option<DiagnosticLogObservation>, ApplicationError>,
    calls: Rc<Cell<usize>>,
}

impl StubPort {
    fn new(
        result: Result<Option<DiagnosticLogObservation>, ApplicationError>,
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

impl DiagnosticLogObservationPort for StubPort {
    fn observe(
        &self,
        _request: &DiagnosticLogObservationRequest,
    ) -> Result<Option<DiagnosticLogObservation>, ApplicationError> {
        self.calls.set(self.calls.get() + 1);
        self.result
    }
}

#[test]
fn 日志结构事实返回_ready() {
    let expected = DiagnosticLogObservation::new(1024, 4, 1, false, false);
    let (port, calls) = StubPort::new(Ok(Some(expected)));
    let use_case = ObserveDiagnosticLog::new(port);

    assert_eq!(
        use_case.execute(&DiagnosticLogObservationRequest),
        LoadCompletion::Ready(expected)
    );
    assert_eq!(calls.get(), 1);
}

#[test]
fn 合法空日志仍返回_ready_而不是_empty() {
    let expected = DiagnosticLogObservation::new(0, 0, 0, false, false);
    let (port, calls) = StubPort::new(Ok(Some(expected)));
    let use_case = ObserveDiagnosticLog::new(port);

    let completion = use_case.execute(&DiagnosticLogObservationRequest);

    assert_eq!(completion, LoadCompletion::Ready(expected));
    assert!(!matches!(completion, LoadCompletion::Empty));
    assert_eq!(calls.get(), 1);
}

#[test]
fn 诊断日志不存在返回_empty() {
    let (port, calls) = StubPort::new(Ok(None));
    let use_case = ObserveDiagnosticLog::new(port);

    assert_eq!(
        use_case.execute(&DiagnosticLogObservationRequest),
        LoadCompletion::Empty
    );
    assert_eq!(calls.get(), 1);
}

#[test]
fn 平台错误保持_failed_和稳定诊断码() {
    let error = ApplicationError::unavailable("DIAGNOSTIC_LOG_OBSERVATION_UNAVAILABLE");
    let (port, calls) = StubPort::new(Err(error));
    let use_case = ObserveDiagnosticLog::new(port);

    assert_eq!(
        use_case.execute(&DiagnosticLogObservationRequest),
        LoadCompletion::Failed(error)
    );
    assert_eq!(calls.get(), 1);
}

#[test]
fn 取消后的迟到诊断日志结果保持_stale() {
    let expected = DiagnosticLogObservation::new(64, 1, 0, false, false);
    let (port, calls) = StubPort::new(Ok(Some(expected)));
    let use_case = ObserveDiagnosticLog::new(port);
    let request_id = RequestId::new(95);
    let mut coordinator = LoadCoordinator::default();
    coordinator.begin(request_id);
    assert_eq!(coordinator.cancel(request_id), TransitionOutcome::Applied);

    let late = use_case.execute(&DiagnosticLogObservationRequest);

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
