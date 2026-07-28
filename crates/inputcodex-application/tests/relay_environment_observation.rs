use std::{cell::Cell, rc::Rc};

use inputcodex_application::{
    ApplicationError, LoadCompletion, ObserveRelayEnvironment, RelayEnvironmentObservationPort,
    RelayEnvironmentObservationRequest,
};
use inputcodex_domain::{
    ClashTunCandidateStatus, ClashTunObservation, CodexDotenvStatus, ObservationCoverageStatus,
    ProxyEnvironmentCoverage, ProxyEnvironmentSource, ProxyEnvironmentVariableName,
    ProxyEnvironmentVariableObservation, RelayEnvironmentObservation,
};

#[derive(Clone)]
struct StubPort {
    result: Result<RelayEnvironmentObservation, ApplicationError>,
    calls: Rc<Cell<usize>>,
}

impl StubPort {
    fn new(
        result: Result<RelayEnvironmentObservation, ApplicationError>,
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

impl RelayEnvironmentObservationPort for StubPort {
    fn observe(
        &self,
        _request: &RelayEnvironmentObservationRequest,
    ) -> Result<RelayEnvironmentObservation, ApplicationError> {
        self.calls.set(self.calls.get() + 1);
        self.result.clone()
    }
}

fn observed_coverage() -> ProxyEnvironmentCoverage {
    ProxyEnvironmentCoverage::new(
        ObservationCoverageStatus::Observed,
        ObservationCoverageStatus::Observed,
        ObservationCoverageStatus::Observed,
    )
}

fn absent_clash_tun() -> ClashTunObservation {
    ClashTunObservation::new(
        ClashTunCandidateStatus::Absent,
        ClashTunCandidateStatus::Absent,
        ClashTunCandidateStatus::Absent,
        ClashTunCandidateStatus::Absent,
    )
}

#[test]
fn 成功观察始终返回_ready() {
    let expected = RelayEnvironmentObservation::new(
        vec![ProxyEnvironmentVariableObservation::new(
            ProxyEnvironmentVariableName::HttpProxy,
            vec![ProxyEnvironmentSource::RuntimeProcess],
        )],
        observed_coverage(),
        CodexDotenvStatus::Absent,
        absent_clash_tun(),
    );
    let (port, calls) = StubPort::new(Ok(expected.clone()));
    let use_case = ObserveRelayEnvironment::new(port);

    assert_eq!(
        use_case.execute(&RelayEnvironmentObservationRequest),
        LoadCompletion::Ready(expected)
    );
    assert_eq!(calls.get(), 1);
}

#[test]
fn 零风险报告也不得返回_empty() {
    let expected = RelayEnvironmentObservation::new(
        Vec::new(),
        observed_coverage(),
        CodexDotenvStatus::Absent,
        absent_clash_tun(),
    );
    let (port, calls) = StubPort::new(Ok(expected.clone()));
    let use_case = ObserveRelayEnvironment::new(port);

    let completion = use_case.execute(&RelayEnvironmentObservationRequest);

    assert_eq!(completion, LoadCompletion::Ready(expected));
    assert!(!matches!(completion, LoadCompletion::Empty));
    assert_eq!(calls.get(), 1);
}

#[test]
fn 局部不可用仍作为_ready_事实返回() {
    let expected = RelayEnvironmentObservation::new(
        Vec::new(),
        ProxyEnvironmentCoverage::new(
            ObservationCoverageStatus::Observed,
            ObservationCoverageStatus::Unavailable,
            ObservationCoverageStatus::NotObserved,
        ),
        CodexDotenvStatus::Unavailable,
        ClashTunObservation::new(
            ClashTunCandidateStatus::Absent,
            ClashTunCandidateStatus::Unreadable,
            ClashTunCandidateStatus::Invalid,
            ClashTunCandidateStatus::Absent,
        ),
    );
    let (port, calls) = StubPort::new(Ok(expected.clone()));
    let use_case = ObserveRelayEnvironment::new(port);

    assert_eq!(
        use_case.execute(&RelayEnvironmentObservationRequest),
        LoadCompletion::Ready(expected)
    );
    assert_eq!(calls.get(), 1);
}

#[test]
fn 平台硬错误返回_failed() {
    let error = ApplicationError::unavailable("RELAY_ENVIRONMENT_OBSERVATION_UNAVAILABLE");
    let (port, calls) = StubPort::new(Err(error));
    let use_case = ObserveRelayEnvironment::new(port);

    assert_eq!(
        use_case.execute(&RelayEnvironmentObservationRequest),
        LoadCompletion::Failed(error)
    );
    assert_eq!(calls.get(), 1);
}
