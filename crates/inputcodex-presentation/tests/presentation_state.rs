use inputcodex_application::{LoadState, RequestId};
use inputcodex_presentation::{Message, PresentationState, apply_message};

#[test]
fn 展示层消息只驱动应用状态机() {
    let mut state = PresentationState::default();
    let request_id = RequestId::new(21);

    apply_message(&mut state, Message::Begin(request_id));
    assert_eq!(state.load_state(), &LoadState::Loading { request_id });

    apply_message(&mut state, Message::Cancel(request_id));
    assert_eq!(state.load_state(), &LoadState::Cancelling { request_id });

    apply_message(&mut state, Message::CancellationFinished(request_id));
    assert_eq!(state.load_state(), &LoadState::Idle);
}
