#![forbid(unsafe_code)]

use inputcodex_application::{LoadCoordinator, LoadState, RequestId, TransitionOutcome};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PresentationState {
    load: LoadCoordinator<()>,
}

impl PresentationState {
    #[must_use]
    pub const fn load_state(&self) -> &LoadState<()> {
        self.load.state()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Message {
    Begin(RequestId),
    Cancel(RequestId),
    CancellationFinished(RequestId),
}

pub fn apply_message(state: &mut PresentationState, message: Message) -> TransitionOutcome {
    match message {
        Message::Begin(request_id) => {
            state.load.begin(request_id);
            TransitionOutcome::Applied
        }
        Message::Cancel(request_id) => state.load.cancel(request_id),
        Message::CancellationFinished(request_id) => state.load.finish_cancellation(request_id),
    }
}

#[cfg(feature = "iced-runtime")]
mod runtime {
    use super::{Message, PresentationState, apply_message};
    use iced::Element;
    use std::error::Error;
    use std::fmt::{self, Display, Formatter};

    #[derive(Debug)]
    pub struct PresentationError(iced::Error);

    impl Display for PresentationError {
        fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
            formatter.write_str("inputcodex 展示层运行失败")
        }
    }

    impl Error for PresentationError {
        fn source(&self) -> Option<&(dyn Error + 'static)> {
            Some(&self.0)
        }
    }

    pub fn run() -> Result<(), PresentationError> {
        iced::application(PresentationState::default, update, view)
            .title("inputcodex")
            .run()
            .map_err(PresentationError)
    }

    fn update(state: &mut PresentationState, message: Message) {
        let _ = apply_message(state, message);
    }

    fn view(_state: &PresentationState) -> Element<'_, Message> {
        iced::widget::container("").into()
    }
}

#[cfg(feature = "iced-runtime")]
pub use runtime::{PresentationError, run};
