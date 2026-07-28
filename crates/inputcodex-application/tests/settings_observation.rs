use std::{cell::Cell, rc::Rc};

use inputcodex_application::{
    ApplicationError, LoadCompletion, LoadCoordinator, LoadState, ObserveSettings, RequestId,
    SettingsObservationPort, SettingsObservationRequest, TransitionOutcome,
};
use inputcodex_domain::SettingsDocumentObservation;

struct StubPort {
    result: Result<Option<SettingsDocumentObservation>, ApplicationError>,
    calls: Rc<Cell<usize>>,
}

impl StubPort {
    fn new(
        result: Result<Option<SettingsDocumentObservation>, ApplicationError>,
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

impl SettingsObservationPort for StubPort {
    fn observe(
        &self,
        _request: &SettingsObservationRequest,
    ) -> Result<Option<SettingsDocumentObservation>, ApplicationError> {
        self.calls.set(self.calls.get() + 1);
        self.result
    }
}

#[test]
fn 已配置设置返回_ready() {
    let expected = SettingsDocumentObservation::new(3);
    let (port, calls) = StubPort::new(Ok(Some(expected)));
    let use_case = ObserveSettings::new(port);

    assert_eq!(
        use_case.execute(&SettingsObservationRequest),
        LoadCompletion::Ready(expected)
    );
    assert_eq!(calls.get(), 1);
}

#[test]
fn 合法空对象仍返回_ready_而不是_empty() {
    let expected = SettingsDocumentObservation::new(0);
    let (port, calls) = StubPort::new(Ok(Some(expected)));
    let use_case = ObserveSettings::new(port);

    let completion = use_case.execute(&SettingsObservationRequest);

    assert_eq!(completion, LoadCompletion::Ready(expected));
    assert!(!matches!(completion, LoadCompletion::Empty));
    assert_eq!(calls.get(), 1);
}

#[test]
fn 文件未配置返回_empty() {
    let (port, calls) = StubPort::new(Ok(None));
    let use_case = ObserveSettings::new(port);

    assert_eq!(
        use_case.execute(&SettingsObservationRequest),
        LoadCompletion::Empty
    );
    assert_eq!(calls.get(), 1);
}

#[test]
fn 平台错误保持_failed_和稳定诊断码() {
    let error = ApplicationError::invalid_input("SETTINGS_OBSERVATION_INVALID_JSON");
    let (port, calls) = StubPort::new(Err(error));
    let use_case = ObserveSettings::new(port);

    assert_eq!(
        use_case.execute(&SettingsObservationRequest),
        LoadCompletion::Failed(error)
    );
    assert_eq!(calls.get(), 1);
}

#[test]
fn 取消后的迟到设置结果保持_stale() {
    let expected = SettingsDocumentObservation::new(1);
    let (port, calls) = StubPort::new(Ok(Some(expected)));
    let use_case = ObserveSettings::new(port);
    let request_id = RequestId::new(92);
    let mut coordinator = LoadCoordinator::default();
    coordinator.begin(request_id);
    assert_eq!(coordinator.cancel(request_id), TransitionOutcome::Applied);

    let late = use_case.execute(&SettingsObservationRequest);

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
