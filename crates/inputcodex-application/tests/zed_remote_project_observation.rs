use std::{cell::Cell, mem::size_of, rc::Rc};

use inputcodex_application::{
    ApplicationError, ErrorKind, LoadCompletion, LoadCoordinator, LoadState,
    ObserveZedRemoteProjects, RequestId, TransitionOutcome,
    ZedRemoteProjectObservationCancellation, ZedRemoteProjectObservationPort,
    ZedRemoteProjectObservationRequest,
};
use inputcodex_domain::{
    ZedRemoteProjectEntry, ZedRemoteProjectId, ZedRemoteProjectObservation, ZedRemoteProjectOrigin,
    ZedRemoteProjectSelectionHint, ZedRemoteProjectSourceCoverage, ZedRemoteProjectSourceSummary,
};

struct StubPort {
    result: Result<Option<ZedRemoteProjectObservation>, ApplicationError>,
    calls: Rc<Cell<usize>>,
    cancelled: Rc<Cell<bool>>,
}

struct StubObservation {
    calls: Rc<Cell<usize>>,
    cancelled: Rc<Cell<bool>>,
}

impl StubPort {
    fn new(
        result: Result<Option<ZedRemoteProjectObservation>, ApplicationError>,
    ) -> (Self, StubObservation) {
        let calls = Rc::new(Cell::new(0));
        let cancelled = Rc::new(Cell::new(false));
        (
            Self {
                result,
                calls: Rc::clone(&calls),
                cancelled: Rc::clone(&cancelled),
            },
            StubObservation { calls, cancelled },
        )
    }
}

impl ZedRemoteProjectObservationPort for StubPort {
    fn observe(
        &self,
        _request: &ZedRemoteProjectObservationRequest,
        cancellation: &ZedRemoteProjectObservationCancellation,
    ) -> Result<Option<ZedRemoteProjectObservation>, ApplicationError> {
        self.calls.set(self.calls.get() + 1);
        self.cancelled.set(cancellation.is_cancelled());
        self.result.clone()
    }
}

fn observation(coverage: ZedRemoteProjectSourceCoverage) -> ZedRemoteProjectObservation {
    let sources = match coverage {
        ZedRemoteProjectSourceCoverage::Complete => {
            ZedRemoteProjectSourceSummary::new(2, 2, 0).expect("完整来源应合法")
        }
        ZedRemoteProjectSourceCoverage::Partial => {
            ZedRemoteProjectSourceSummary::new(2, 1, 1).expect("部分来源应合法")
        }
    };
    let id = ZedRemoteProjectId::new(format!("zed-remote-project:v1:sha256:{}", "a".repeat(64)))
        .expect("测试稳定假名应合法");
    let entry = ZedRemoteProjectEntry::new(
        id,
        ZedRemoteProjectOrigin::CodexRemoteProject,
        ZedRemoteProjectSelectionHint::SelectedHostHint,
    );

    ZedRemoteProjectObservation::new(vec![entry], sources).expect("测试观察应合法")
}

#[test]
fn 请求保持零字段且取消标记可克隆共享() {
    fn assert_default<T: Default>() {}

    assert_eq!(size_of::<ZedRemoteProjectObservationRequest>(), 0);
    assert_default::<ZedRemoteProjectObservationRequest>();

    let cancellation = ZedRemoteProjectObservationCancellation::default();
    let shared = cancellation.clone();
    assert!(!cancellation.is_cancelled());
    shared.cancel();
    assert!(cancellation.is_cancelled());
    assert_eq!(
        format!("{cancellation:?}"),
        "ZedRemoteProjectObservationCancellation { cancelled: true }"
    );
}

#[test]
fn 完整与部分来源观察都映射为_ready() {
    for coverage in [
        ZedRemoteProjectSourceCoverage::Complete,
        ZedRemoteProjectSourceCoverage::Partial,
    ] {
        let expected = observation(coverage);
        let (port, observed) = StubPort::new(Ok(Some(expected.clone())));
        let use_case = ObserveZedRemoteProjects::new(port);
        let cancellation = ZedRemoteProjectObservationCancellation::default();

        let completion = use_case.execute(&ZedRemoteProjectObservationRequest, &cancellation);

        let LoadCompletion::Ready(actual) = completion else {
            panic!("已有项目的完整或部分覆盖都必须返回 Ready");
        };
        assert_eq!(actual, expected);
        assert_eq!(actual.sources().coverage(), coverage);
        assert_eq!(observed.calls.get(), 1);
        assert!(!observed.cancelled.get());
    }
}

#[test]
fn 无项目映射为_empty_而平台错误保持_failed() {
    let request = ZedRemoteProjectObservationRequest;
    let cancellation = ZedRemoteProjectObservationCancellation::default();
    let (empty_port, empty_observed) = StubPort::new(Ok(None));
    let empty_use_case = ObserveZedRemoteProjects::new(empty_port);
    assert_eq!(
        empty_use_case.execute(&request, &cancellation),
        LoadCompletion::Empty
    );
    assert_eq!(empty_observed.calls.get(), 1);

    let error = ApplicationError::unavailable("ZED_REMOTE_PROJECT_OBSERVATION_UNAVAILABLE");
    let (failed_port, failed_observed) = StubPort::new(Err(error));
    let failed_use_case = ObserveZedRemoteProjects::new(failed_port);
    assert_eq!(
        failed_use_case.execute(&request, &cancellation),
        LoadCompletion::Failed(error)
    );
    assert_eq!(failed_observed.calls.get(), 1);
}

#[test]
fn 执行前已取消则不调用端口并返回稳定诊断() {
    let (port, observed) = StubPort::new(Ok(Some(observation(
        ZedRemoteProjectSourceCoverage::Complete,
    ))));
    let use_case = ObserveZedRemoteProjects::new(port);
    let cancellation = ZedRemoteProjectObservationCancellation::default();
    cancellation.cancel();

    let completion = use_case.execute(&ZedRemoteProjectObservationRequest, &cancellation);
    let LoadCompletion::Failed(error) = completion else {
        panic!("预取消请求必须失败");
    };
    assert_eq!(error.kind(), ErrorKind::Cancelled);
    assert_eq!(
        error.code().as_str(),
        "ZED_REMOTE_PROJECT_OBSERVATION_CANCELLED"
    );
    assert_eq!(observed.calls.get(), 0);
}

#[test]
fn 取消后的迟到结果由加载协调器隔离为_stale() {
    let expected = observation(ZedRemoteProjectSourceCoverage::Complete);
    let (port, observed) = StubPort::new(Ok(Some(expected)));
    let use_case = ObserveZedRemoteProjects::new(port);
    let request_id = RequestId::new(132);
    let cancellation = ZedRemoteProjectObservationCancellation::default();
    let mut coordinator = LoadCoordinator::default();
    coordinator.begin(request_id);

    let late = use_case.execute(&ZedRemoteProjectObservationRequest, &cancellation);
    assert_eq!(coordinator.cancel(request_id), TransitionOutcome::Applied);
    assert_eq!(
        coordinator.complete(request_id, late),
        TransitionOutcome::Stale
    );
    assert_eq!(observed.calls.get(), 1);
    assert!(matches!(
        coordinator.state(),
        LoadState::Cancelling { request_id: current } if *current == request_id
    ));
}
