#![forbid(unsafe_code)]

use inputcodex_domain::DiagnosticCode;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RequestId(u64);

impl RequestId {
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    #[must_use]
    pub const fn value(self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorKind {
    Unsupported,
    Unavailable,
    Timeout,
    Cancelled,
    Internal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ApplicationError {
    kind: ErrorKind,
    code: DiagnosticCode,
}

impl ApplicationError {
    #[must_use]
    pub const fn new(kind: ErrorKind, code: DiagnosticCode) -> Self {
        Self { kind, code }
    }

    #[must_use]
    pub const fn unsupported(code: &'static str) -> Self {
        Self::new(ErrorKind::Unsupported, DiagnosticCode::new(code))
    }

    #[must_use]
    pub const fn unavailable(code: &'static str) -> Self {
        Self::new(ErrorKind::Unavailable, DiagnosticCode::new(code))
    }

    #[must_use]
    pub const fn kind(self) -> ErrorKind {
        self.kind
    }

    #[must_use]
    pub const fn code(self) -> DiagnosticCode {
        self.code
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoadState<T> {
    Idle,
    Loading {
        request_id: RequestId,
    },
    Ready {
        request_id: RequestId,
        value: T,
    },
    Empty {
        request_id: RequestId,
    },
    Failed {
        request_id: RequestId,
        error: ApplicationError,
    },
    Cancelling {
        request_id: RequestId,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoadCompletion<T> {
    Ready(T),
    Empty,
    Failed(ApplicationError),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransitionOutcome {
    Applied,
    Stale,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoadCoordinator<T> {
    state: LoadState<T>,
}

impl<T> Default for LoadCoordinator<T> {
    fn default() -> Self {
        Self {
            state: LoadState::Idle,
        }
    }
}

impl<T> LoadCoordinator<T> {
    #[must_use]
    pub const fn state(&self) -> &LoadState<T> {
        &self.state
    }

    pub fn begin(&mut self, request_id: RequestId) {
        self.state = LoadState::Loading { request_id };
    }

    pub fn cancel(&mut self, request_id: RequestId) -> TransitionOutcome {
        let is_current = matches!(
            &self.state,
            LoadState::Loading {
                request_id: current,
            } if *current == request_id
        );

        if !is_current {
            return TransitionOutcome::Stale;
        }

        self.state = LoadState::Cancelling { request_id };
        TransitionOutcome::Applied
    }

    pub fn complete(
        &mut self,
        request_id: RequestId,
        completion: LoadCompletion<T>,
    ) -> TransitionOutcome {
        let is_current = matches!(
            &self.state,
            LoadState::Loading {
                request_id: current,
            } if *current == request_id
        );

        if !is_current {
            return TransitionOutcome::Stale;
        }

        self.state = match completion {
            LoadCompletion::Ready(value) => LoadState::Ready { request_id, value },
            LoadCompletion::Empty => LoadState::Empty { request_id },
            LoadCompletion::Failed(error) => LoadState::Failed { request_id, error },
        };
        TransitionOutcome::Applied
    }

    pub fn finish_cancellation(&mut self, request_id: RequestId) -> TransitionOutcome {
        let is_current = matches!(
            &self.state,
            LoadState::Cancelling {
                request_id: current,
            } if *current == request_id
        );

        if !is_current {
            return TransitionOutcome::Stale;
        }

        self.state = LoadState::Idle;
        TransitionOutcome::Applied
    }
}

pub trait LoadPort<T> {
    fn load(&self, request_id: RequestId) -> Result<Option<T>, ApplicationError>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlatformKind {
    Windows,
    Macos,
}

pub trait PlatformPort {
    fn current_platform(&self) -> Result<PlatformKind, ApplicationError>;
}
