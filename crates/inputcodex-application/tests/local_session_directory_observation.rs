use std::{cell::Cell, rc::Rc};

use inputcodex_application::{
    ApplicationError, ErrorKind, LoadCompletion, LoadCoordinator, LoadState,
    LocalSessionDirectoryCancellation, LocalSessionDirectoryObservationPort,
    LocalSessionDirectoryRequest, ObserveLocalSessionDirectory, RequestId, TransitionOutcome,
};
use inputcodex_domain::{
    LocalSessionDirectoryEntry, LocalSessionDirectoryPage, LocalSessionSourceCoverage,
    LocalSessionSourceSummary, LocalSessionTitle,
};

struct StubPort {
    result: Result<Option<LocalSessionDirectoryPage>, ApplicationError>,
    calls: Rc<Cell<usize>>,
    request: Rc<Cell<Option<(usize, usize, usize)>>>,
    cancelled: Rc<Cell<bool>>,
}

struct StubObservation {
    calls: Rc<Cell<usize>>,
    request: Rc<Cell<Option<(usize, usize, usize)>>>,
    cancelled: Rc<Cell<bool>>,
}

impl StubPort {
    fn new(
        result: Result<Option<LocalSessionDirectoryPage>, ApplicationError>,
    ) -> (Self, StubObservation) {
        let calls = Rc::new(Cell::new(0));
        let request = Rc::new(Cell::new(None));
        let cancelled = Rc::new(Cell::new(false));
        (
            Self {
                result,
                calls: Rc::clone(&calls),
                request: Rc::clone(&request),
                cancelled: Rc::clone(&cancelled),
            },
            StubObservation {
                calls,
                request,
                cancelled,
            },
        )
    }
}

impl LocalSessionDirectoryObservationPort for StubPort {
    fn observe(
        &self,
        request: &LocalSessionDirectoryRequest,
        cancellation: &LocalSessionDirectoryCancellation,
    ) -> Result<Option<LocalSessionDirectoryPage>, ApplicationError> {
        self.calls.set(self.calls.get() + 1);
        self.request.set(Some((
            request.offset(),
            request.limit(),
            request.source_row_limit(),
        )));
        self.cancelled.set(cancellation.is_cancelled());
        self.result.clone()
    }
}

fn page(coverage: LocalSessionSourceCoverage) -> LocalSessionDirectoryPage {
    let sources = match coverage {
        LocalSessionSourceCoverage::Complete => {
            LocalSessionSourceSummary::new(1, 1, 0).expect("完整来源应合法")
        }
        LocalSessionSourceCoverage::Partial => {
            LocalSessionSourceSummary::new(2, 1, 1).expect("部分来源应合法")
        }
    };
    let entry = LocalSessionDirectoryEntry::new(
        "session-104".to_owned(),
        LocalSessionTitle::from_raw("本地会话"),
        false,
        Some(104),
    )
    .expect("条目应合法");

    LocalSessionDirectoryPage::new(vec![entry], 0, 50, false, sources).expect("页面应合法")
}

#[test]
fn 请求提供默认分页并预先验证每来源读取上限() {
    let default = LocalSessionDirectoryRequest::default();
    assert_eq!(default.offset(), 0);
    assert_eq!(default.limit(), 50);
    assert_eq!(default.source_row_limit(), 51);

    let explicit = LocalSessionDirectoryRequest::new(25, 100).expect("合法分页应通过");
    assert_eq!(explicit.offset(), 25);
    assert_eq!(explicit.limit(), 100);
    assert_eq!(explicit.source_row_limit(), 126);
}

#[test]
fn 非法分页返回稳定的_invalid_input_诊断() {
    for result in [
        LocalSessionDirectoryRequest::new(0, 0),
        LocalSessionDirectoryRequest::new(0, 101),
        LocalSessionDirectoryRequest::new(usize::MAX - 1, 1),
    ] {
        let error = result.expect_err("非法分页必须失败");
        assert_eq!(error.kind(), ErrorKind::InvalidInput);
        assert_eq!(
            error.code().as_str(),
            "LOCAL_SESSION_DIRECTORY_INVALID_PAGINATION"
        );
    }
}

#[test]
fn 完整来源页面映射为_ready_并向端口传递有界请求() {
    let expected = page(LocalSessionSourceCoverage::Complete);
    let (port, observed) = StubPort::new(Ok(Some(expected.clone())));
    let use_case = ObserveLocalSessionDirectory::new(port);
    let request = LocalSessionDirectoryRequest::new(0, 50).expect("分页应合法");
    let cancellation = LocalSessionDirectoryCancellation::default();

    assert_eq!(
        use_case.execute(&request, &cancellation),
        LoadCompletion::Ready(expected)
    );
    assert_eq!(observed.calls.get(), 1);
    assert_eq!(observed.request.get(), Some((0, 50, 51)));
    assert!(!observed.cancelled.get());
}

#[test]
fn 部分来源页面仍映射为_ready_且保留_partial_覆盖事实() {
    let expected = page(LocalSessionSourceCoverage::Partial);
    let (port, _) = StubPort::new(Ok(Some(expected.clone())));
    let use_case = ObserveLocalSessionDirectory::new(port);

    let completion = use_case.execute(
        &LocalSessionDirectoryRequest::default(),
        &LocalSessionDirectoryCancellation::default(),
    );

    let LoadCompletion::Ready(actual) = completion else {
        panic!("部分来源但已有条目必须返回 Ready");
    };
    assert_eq!(actual, expected);
    assert_eq!(
        actual.sources().coverage(),
        LocalSessionSourceCoverage::Partial
    );
}

#[test]
fn 无条目映射为_empty_而平台错误保持_failed() {
    let (empty_port, empty_observed) = StubPort::new(Ok(None));
    let empty_use_case = ObserveLocalSessionDirectory::new(empty_port);
    let request = LocalSessionDirectoryRequest::default();
    let cancellation = LocalSessionDirectoryCancellation::default();

    assert_eq!(
        empty_use_case.execute(&request, &cancellation),
        LoadCompletion::Empty
    );
    assert_eq!(empty_observed.calls.get(), 1);

    let error = ApplicationError::unavailable("LOCAL_SESSION_DIRECTORY_UNAVAILABLE");
    let (failed_port, failed_observed) = StubPort::new(Err(error));
    let failed_use_case = ObserveLocalSessionDirectory::new(failed_port);
    assert_eq!(
        failed_use_case.execute(&request, &cancellation),
        LoadCompletion::Failed(error)
    );
    assert_eq!(failed_observed.calls.get(), 1);
}

#[test]
fn 取消标记可克隆共享且调试输出不泄露实现细节() {
    let cancellation = LocalSessionDirectoryCancellation::default();
    let shared = cancellation.clone();
    assert!(!cancellation.is_cancelled());

    shared.cancel();

    assert!(cancellation.is_cancelled());
    assert_eq!(
        format!("{cancellation:?}"),
        "LocalSessionDirectoryCancellation { cancelled: true }"
    );
}

#[test]
fn 执行前已取消则不调用端口并返回稳定_cancelled_诊断() {
    let (port, observed) = StubPort::new(Ok(Some(page(LocalSessionSourceCoverage::Complete))));
    let use_case = ObserveLocalSessionDirectory::new(port);
    let cancellation = LocalSessionDirectoryCancellation::default();
    cancellation.cancel();

    let completion = use_case.execute(&LocalSessionDirectoryRequest::default(), &cancellation);

    let LoadCompletion::Failed(error) = completion else {
        panic!("预取消请求必须失败");
    };
    assert_eq!(error.kind(), ErrorKind::Cancelled);
    assert_eq!(error.code().as_str(), "LOCAL_SESSION_DIRECTORY_CANCELLED");
    assert_eq!(observed.calls.get(), 0);
}

#[test]
fn 取消后的迟到页面由加载协调器隔离为_stale() {
    let expected = page(LocalSessionSourceCoverage::Complete);
    let (port, observed) = StubPort::new(Ok(Some(expected)));
    let use_case = ObserveLocalSessionDirectory::new(port);
    let request_id = RequestId::new(104);
    let request = LocalSessionDirectoryRequest::default();
    let cancellation = LocalSessionDirectoryCancellation::default();
    let mut coordinator = LoadCoordinator::default();
    coordinator.begin(request_id);

    let late = use_case.execute(&request, &cancellation);
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
