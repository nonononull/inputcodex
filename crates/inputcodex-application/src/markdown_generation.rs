use std::{
    fmt,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
};

use inputcodex_domain::{MAX_LOCAL_SESSION_ID_BYTES, SessionMarkdownDocument};

use crate::{ApplicationError, LoadCompletion};

const INVALID_SESSION_ID_CODE: &str = "MARKDOWN_GENERATION_INVALID_SESSION_ID";
const CANCELLED_CODE: &str = "MARKDOWN_GENERATION_CANCELLED";

#[derive(Clone, PartialEq, Eq)]
pub struct MarkdownGenerationRequest {
    session_id: String,
}

impl MarkdownGenerationRequest {
    pub fn new(session_id: String) -> Result<Self, ApplicationError> {
        if session_id.trim().is_empty()
            || session_id.trim() != session_id
            || session_id.chars().any(char::is_whitespace)
            || session_id.chars().any(char::is_control)
            || session_id.len() > MAX_LOCAL_SESSION_ID_BYTES
        {
            return Err(ApplicationError::invalid_input(INVALID_SESSION_ID_CODE));
        }

        Ok(Self { session_id })
    }

    #[must_use]
    pub fn session_id(&self) -> &str {
        &self.session_id
    }
}

impl fmt::Debug for MarkdownGenerationRequest {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("MarkdownGenerationRequest")
            .field("session_id_bytes", &self.session_id.len())
            .finish()
    }
}

#[derive(Clone, Default)]
pub struct MarkdownGenerationCancellation {
    cancelled: Arc<AtomicBool>,
}

impl MarkdownGenerationCancellation {
    pub fn cancel(&self) {
        self.cancelled.store(true, Ordering::Release);
    }

    #[must_use]
    pub fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::Acquire)
    }
}

impl fmt::Debug for MarkdownGenerationCancellation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("MarkdownGenerationCancellation")
            .field("cancelled", &self.is_cancelled())
            .finish()
    }
}

pub trait MarkdownGenerationPort {
    fn generate(
        &self,
        request: &MarkdownGenerationRequest,
        cancellation: &MarkdownGenerationCancellation,
    ) -> Result<Option<SessionMarkdownDocument>, ApplicationError>;
}

#[derive(Clone)]
pub struct GenerateSessionMarkdown<P> {
    port: P,
}

impl<P> GenerateSessionMarkdown<P> {
    #[must_use]
    pub const fn new(port: P) -> Self {
        Self { port }
    }
}

impl<P: MarkdownGenerationPort> GenerateSessionMarkdown<P> {
    #[must_use]
    pub fn execute(
        &self,
        request: &MarkdownGenerationRequest,
        cancellation: &MarkdownGenerationCancellation,
    ) -> LoadCompletion<SessionMarkdownDocument> {
        if cancellation.is_cancelled() {
            return LoadCompletion::Failed(ApplicationError::cancelled(CANCELLED_CODE));
        }

        match self.port.generate(request, cancellation) {
            Ok(Some(document)) => LoadCompletion::Ready(document),
            Ok(None) => LoadCompletion::Empty,
            Err(error) => LoadCompletion::Failed(error),
        }
    }
}
