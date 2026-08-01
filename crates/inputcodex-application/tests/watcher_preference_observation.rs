use std::{cell::Cell, mem::size_of, rc::Rc};

use inputcodex_application::{
    ApplicationError, LoadCompletion, LoadCoordinator, LoadState, ObserveWatcherPreference,
    RequestId, TransitionOutcome, WatcherPreferenceObservationPort,
    WatcherPreferenceObservationRequest,
};
use inputcodex_domain::{WatcherPreference, WatcherPreferenceObservation};

struct StubPort {
    result: Result<WatcherPreferenceObservation, ApplicationError>,
    calls: Rc<Cell<usize>>,
}

impl StubPort {
    fn new(
        result: Result<WatcherPreferenceObservation, ApplicationError>,
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

impl WatcherPreferenceObservationPort for StubPort {
    fn observe(
        &self,
        _request: &WatcherPreferenceObservationRequest,
    ) -> Result<WatcherPreferenceObservation, ApplicationError> {
        self.calls.set(self.calls.get() + 1);
        self.result
    }
}

#[test]
fn 零字段请求把两个领域状态映射为_ready() {
    assert_eq!(size_of::<WatcherPreferenceObservationRequest>(), 0);

    for preference in [
        WatcherPreference::EnabledByDefault,
        WatcherPreference::ExplicitlyDisabled,
    ] {
        let expected = WatcherPreferenceObservation::new(preference);
        let (port, calls) = StubPort::new(Ok(expected));
        let use_case = ObserveWatcherPreference::new(port);

        assert_eq!(
            use_case.execute(&WatcherPreferenceObservationRequest),
            LoadCompletion::Ready(expected)
        );
        assert_eq!(calls.get(), 1);
    }
}

#[test]
fn 平台错误保持_failed_和稳定诊断码() {
    let error = ApplicationError::unavailable("WATCHER_PREFERENCE_UNREADABLE");
    let (port, calls) = StubPort::new(Err(error));
    let use_case = ObserveWatcherPreference::new(port);

    assert_eq!(
        use_case.execute(&WatcherPreferenceObservationRequest),
        LoadCompletion::Failed(error)
    );
    assert_eq!(calls.get(), 1);
}

#[test]
fn 取消后的迟到偏好结果保持_stale() {
    let expected = WatcherPreferenceObservation::new(WatcherPreference::EnabledByDefault);
    let (port, calls) = StubPort::new(Ok(expected));
    let use_case = ObserveWatcherPreference::new(port);
    let request_id = RequestId::new(128);
    let mut coordinator = LoadCoordinator::default();
    coordinator.begin(request_id);
    assert_eq!(coordinator.cancel(request_id), TransitionOutcome::Applied);

    let late = use_case.execute(&WatcherPreferenceObservationRequest);

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
