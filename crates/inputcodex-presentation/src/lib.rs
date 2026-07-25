#![forbid(unsafe_code)]

use inputcodex_application::{LoadCoordinator, LoadState, RequestId, TransitionOutcome};
#[cfg(any(feature = "iced-runtime", test))]
use std::sync::atomic::{AtomicBool, Ordering};

pub const PERFORMANCE_PROBE_ENVIRONMENT_VARIABLE: &str = "INPUTCODEX_PERFORMANCE_PROBE";
pub const PERFORMANCE_READY_MARKER: &str = "INPUTCODEX_PERFORMANCE_READY_V1";

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

#[cfg(any(feature = "iced-runtime", test))]
fn take_performance_ready_marker(
    enabled_value: Option<&str>,
    reported: &AtomicBool,
) -> Option<&'static str> {
    if enabled_value != Some("1") {
        return None;
    }

    reported
        .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
        .is_ok()
        .then_some(PERFORMANCE_READY_MARKER)
}

#[cfg(test)]
mod performance_probe_tests {
    use super::{PERFORMANCE_READY_MARKER, take_performance_ready_marker};
    use std::sync::atomic::{AtomicBool, Ordering};

    #[test]
    fn probe_requires_exact_opt_in_value() {
        for value in [None, Some(""), Some("0"), Some("true"), Some("01")] {
            let reported = AtomicBool::new(false);

            assert_eq!(take_performance_ready_marker(value, &reported), None);
            assert!(!reported.load(Ordering::Acquire));
        }
    }

    #[test]
    fn probe_returns_stable_marker_once() {
        let reported = AtomicBool::new(false);

        assert_eq!(
            take_performance_ready_marker(Some("1"), &reported),
            Some(PERFORMANCE_READY_MARKER)
        );
        assert_eq!(take_performance_ready_marker(Some("1"), &reported), None);
        assert!(reported.load(Ordering::Acquire));
    }
}

#[cfg(feature = "iced-runtime")]
mod runtime {
    use super::{
        Message, PERFORMANCE_PROBE_ENVIRONMENT_VARIABLE, PresentationState, apply_message,
        take_performance_ready_marker,
    };
    use iced::Element;
    use std::env;
    use std::error::Error;
    use std::fmt::{self, Display, Formatter};
    use std::io::{self, Write};
    use std::sync::atomic::AtomicBool;

    static PERFORMANCE_READY_REPORTED: AtomicBool = AtomicBool::new(false);

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
        let element = iced::widget::container("").into();
        report_performance_ready();
        element
    }

    fn report_performance_ready() {
        let enabled_value = env::var(PERFORMANCE_PROBE_ENVIRONMENT_VARIABLE).ok();
        if let Some(marker) =
            take_performance_ready_marker(enabled_value.as_deref(), &PERFORMANCE_READY_REPORTED)
        {
            let stdout = io::stdout();
            let mut output = stdout.lock();
            let _ = writeln!(output, "{marker}");
            let _ = output.flush();
        }
    }
}

#[cfg(feature = "iced-runtime")]
pub use runtime::{PresentationError, run};
