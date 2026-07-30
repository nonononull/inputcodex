use std::{cell::Cell, rc::Rc};

use inputcodex_application::{
    ApplicationError, ErrorKind, GenerateSessionMarkdown, LoadCompletion, LoadCoordinator,
    LoadState, MarkdownGenerationCancellation, MarkdownGenerationPort, MarkdownGenerationRequest,
    RequestId, TransitionOutcome,
};
use inputcodex_domain::{
    LocalSessionTitle, MAX_LOCAL_SESSION_ID_BYTES, MarkdownMessage, MarkdownMessageRole,
    SessionMarkdownDocument,
};

struct StubPort {
    result: Result<Option<SessionMarkdownDocument>, ApplicationError>,
    calls: Rc<Cell<usize>>,
    session_id_bytes: Rc<Cell<usize>>,
    cancelled: Rc<Cell<bool>>,
}

struct StubObservation {
    calls: Rc<Cell<usize>>,
    session_id_bytes: Rc<Cell<usize>>,
    cancelled: Rc<Cell<bool>>,
}

impl StubPort {
    fn new(
        result: Result<Option<SessionMarkdownDocument>, ApplicationError>,
    ) -> (Self, StubObservation) {
        let calls = Rc::new(Cell::new(0));
        let session_id_bytes = Rc::new(Cell::new(0));
        let cancelled = Rc::new(Cell::new(false));
        (
            Self {
                result,
                calls: Rc::clone(&calls),
                session_id_bytes: Rc::clone(&session_id_bytes),
                cancelled: Rc::clone(&cancelled),
            },
            StubObservation {
                calls,
                session_id_bytes,
                cancelled,
            },
        )
    }
}

impl MarkdownGenerationPort for StubPort {
    fn generate(
        &self,
        request: &MarkdownGenerationRequest,
        cancellation: &MarkdownGenerationCancellation,
    ) -> Result<Option<SessionMarkdownDocument>, ApplicationError> {
        self.calls.set(self.calls.get() + 1);
        self.session_id_bytes.set(request.session_id().len());
        self.cancelled.set(cancellation.is_cancelled());
        self.result.clone()
    }
}

fn document() -> SessionMarkdownDocument {
    let title = LocalSessionTitle::from_raw("应用测试").expect("标题应合法");
    let message = MarkdownMessage::new(MarkdownMessageRole::Assistant, None, "完成".to_owned())
        .expect("消息应合法");
    SessionMarkdownDocument::generate(Some(&title), vec![message]).expect("文档应生成")
}

#[test]
fn 请求只接受合法会话标识且调试输出脱敏() {
    let request = MarkdownGenerationRequest::new("private-session-109".to_owned())
        .expect("合法会话标识应通过");

    assert_eq!(request.session_id(), "private-session-109");
    assert_eq!(
        format!("{request:?}"),
        "MarkdownGenerationRequest { session_id_bytes: 19 }"
    );
    assert!(!format!("{request:?}").contains("private-session-109"));
}

#[test]
fn 非法会话标识返回稳定_invalid_input_诊断() {
    for session_id in [
        String::new(),
        "   ".to_owned(),
        " leading".to_owned(),
        "embedded space".to_owned(),
        "line\nbreak".to_owned(),
        "x".repeat(MAX_LOCAL_SESSION_ID_BYTES + 1),
    ] {
        let error = MarkdownGenerationRequest::new(session_id).expect_err("非法标识必须失败");
        assert_eq!(error.kind(), ErrorKind::InvalidInput);
        assert_eq!(
            error.code().as_str(),
            "MARKDOWN_GENERATION_INVALID_SESSION_ID"
        );
    }
}

#[test]
fn 端口成功空值和错误精确映射为加载完成态() {
    let request = MarkdownGenerationRequest::new("session-109".to_owned()).expect("请求应合法");
    let cancellation = MarkdownGenerationCancellation::default();
    let expected = document();
    let (ready_port, ready_observed) = StubPort::new(Ok(Some(expected.clone())));

    assert_eq!(
        GenerateSessionMarkdown::new(ready_port).execute(&request, &cancellation),
        LoadCompletion::Ready(expected)
    );
    assert_eq!(ready_observed.calls.get(), 1);
    assert_eq!(ready_observed.session_id_bytes.get(), 11);
    assert!(!ready_observed.cancelled.get());

    let (empty_port, empty_observed) = StubPort::new(Ok(None));
    assert_eq!(
        GenerateSessionMarkdown::new(empty_port).execute(&request, &cancellation),
        LoadCompletion::Empty
    );
    assert_eq!(empty_observed.calls.get(), 1);

    let error = ApplicationError::unavailable("MARKDOWN_GENERATION_UNAVAILABLE");
    let (failed_port, failed_observed) = StubPort::new(Err(error));
    assert_eq!(
        GenerateSessionMarkdown::new(failed_port).execute(&request, &cancellation),
        LoadCompletion::Failed(error)
    );
    assert_eq!(failed_observed.calls.get(), 1);
}

#[test]
fn 取消标记可克隆共享且只公开布尔状态() {
    let cancellation = MarkdownGenerationCancellation::default();
    let shared = cancellation.clone();
    assert!(!cancellation.is_cancelled());

    shared.cancel();

    assert!(cancellation.is_cancelled());
    assert_eq!(
        format!("{cancellation:?}"),
        "MarkdownGenerationCancellation { cancelled: true }"
    );
}

#[test]
fn 执行前已取消则不调用端口并返回稳定诊断() {
    let (port, observed) = StubPort::new(Ok(Some(document())));
    let use_case = GenerateSessionMarkdown::new(port);
    let request = MarkdownGenerationRequest::new("session-109".to_owned()).expect("请求应合法");
    let cancellation = MarkdownGenerationCancellation::default();
    cancellation.cancel();

    let completion = use_case.execute(&request, &cancellation);

    let LoadCompletion::Failed(error) = completion else {
        panic!("预取消请求必须失败");
    };
    assert_eq!(error.kind(), ErrorKind::Cancelled);
    assert_eq!(error.code().as_str(), "MARKDOWN_GENERATION_CANCELLED");
    assert_eq!(observed.calls.get(), 0);
}

#[test]
fn 旧请求迟到结果由加载协调器隔离() {
    let (port, observed) = StubPort::new(Ok(Some(document())));
    let use_case = GenerateSessionMarkdown::new(port);
    let request = MarkdownGenerationRequest::new("session-109".to_owned()).expect("请求应合法");
    let cancellation = MarkdownGenerationCancellation::default();
    let request_id = RequestId::new(109);
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
