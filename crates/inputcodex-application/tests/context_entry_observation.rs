use std::{cell::Cell, mem::size_of, rc::Rc};

use inputcodex_application::{
    ApplicationError, ContextEntryObservationPort, ContextEntryObservationRequest, LoadCompletion,
    LoadCoordinator, LoadState, ObserveContextEntries, RequestId, TransitionOutcome,
};
use inputcodex_domain::{
    ContextEntryCatalogObservation, ContextEntryKind, ContextEntryObservation,
};

struct StubPort {
    result: Result<Option<ContextEntryCatalogObservation>, ApplicationError>,
    calls: Rc<Cell<usize>>,
}

impl StubPort {
    fn new(
        result: Result<Option<ContextEntryCatalogObservation>, ApplicationError>,
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

impl ContextEntryObservationPort for StubPort {
    fn observe(
        &self,
        _request: &ContextEntryObservationRequest,
    ) -> Result<Option<ContextEntryCatalogObservation>, ApplicationError> {
        self.calls.set(self.calls.get() + 1);
        self.result.clone()
    }
}

fn observed_catalog() -> ContextEntryCatalogObservation {
    ContextEntryCatalogObservation::new(vec![
        ContextEntryObservation::new("context7".to_owned(), ContextEntryKind::McpServer, true)
            .unwrap(),
        ContextEntryObservation::new("writer".to_owned(), ContextEntryKind::Skill, false).unwrap(),
    ])
}

#[test]
fn 已配置上下文目录返回_ready() {
    let expected = observed_catalog();
    let (port, calls) = StubPort::new(Ok(Some(expected.clone())));
    let use_case = ObserveContextEntries::new(port);

    assert_eq!(
        use_case.execute(&ContextEntryObservationRequest),
        LoadCompletion::Ready(expected)
    );
    assert_eq!(calls.get(), 1);
}

#[test]
fn 合法零条目目录仍返回_ready_而不是_empty() {
    let expected = ContextEntryCatalogObservation::new(Vec::new());
    let (port, calls) = StubPort::new(Ok(Some(expected.clone())));
    let use_case = ObserveContextEntries::new(port);

    let completion = use_case.execute(&ContextEntryObservationRequest);

    assert_eq!(completion, LoadCompletion::Ready(expected));
    assert!(!matches!(completion, LoadCompletion::Empty));
    assert_eq!(calls.get(), 1);
}

#[test]
fn 固定配置文件未配置返回_empty() {
    let (port, calls) = StubPort::new(Ok(None));
    let use_case = ObserveContextEntries::new(port);

    assert_eq!(
        use_case.execute(&ContextEntryObservationRequest),
        LoadCompletion::Empty
    );
    assert_eq!(calls.get(), 1);
}

#[test]
fn 平台错误保持_failed_和稳定诊断码() {
    let error = ApplicationError::invalid_input("CONTEXT_ENTRY_OBSERVATION_INVALID_TOML");
    let (port, calls) = StubPort::new(Err(error));
    let use_case = ObserveContextEntries::new(port);

    assert_eq!(
        use_case.execute(&ContextEntryObservationRequest),
        LoadCompletion::Failed(error)
    );
    assert_eq!(calls.get(), 1);
}

#[test]
fn 请求保持零字段且不可携带路径或配置正文() {
    fn assert_default<T: Default>() {}

    assert_eq!(size_of::<ContextEntryObservationRequest>(), 0);
    assert_default::<ContextEntryObservationRequest>();
}

#[test]
fn 取消后的迟到上下文目录结果保持_stale() {
    let expected = observed_catalog();
    let (port, calls) = StubPort::new(Ok(Some(expected)));
    let use_case = ObserveContextEntries::new(port);
    let request_id = RequestId::new(101);
    let mut coordinator = LoadCoordinator::default();
    coordinator.begin(request_id);
    assert_eq!(coordinator.cancel(request_id), TransitionOutcome::Applied);

    let late = use_case.execute(&ContextEntryObservationRequest);

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
