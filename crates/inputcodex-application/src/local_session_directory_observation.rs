use std::{
    fmt,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
};

use inputcodex_domain::{LocalSessionDirectoryPage, MAX_LOCAL_SESSION_DIRECTORY_PAGE_SIZE};

use crate::{ApplicationError, LoadCompletion};

const DEFAULT_LOCAL_SESSION_DIRECTORY_PAGE_SIZE: usize = 50;
const INVALID_PAGINATION_CODE: &str = "LOCAL_SESSION_DIRECTORY_INVALID_PAGINATION";
const CANCELLED_CODE: &str = "LOCAL_SESSION_DIRECTORY_CANCELLED";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LocalSessionDirectoryRequest {
    offset: usize,
    limit: usize,
    source_row_limit: usize,
}

impl Default for LocalSessionDirectoryRequest {
    fn default() -> Self {
        Self {
            offset: 0,
            limit: DEFAULT_LOCAL_SESSION_DIRECTORY_PAGE_SIZE,
            source_row_limit: DEFAULT_LOCAL_SESSION_DIRECTORY_PAGE_SIZE + 1,
        }
    }
}

impl LocalSessionDirectoryRequest {
    pub fn new(offset: usize, limit: usize) -> Result<Self, ApplicationError> {
        if limit == 0 || limit > MAX_LOCAL_SESSION_DIRECTORY_PAGE_SIZE {
            return Err(ApplicationError::invalid_input(INVALID_PAGINATION_CODE));
        }

        let source_row_limit = offset
            .checked_add(limit)
            .and_then(|value| value.checked_add(1))
            .ok_or_else(|| ApplicationError::invalid_input(INVALID_PAGINATION_CODE))?;

        Ok(Self {
            offset,
            limit,
            source_row_limit,
        })
    }

    #[must_use]
    pub const fn offset(self) -> usize {
        self.offset
    }

    #[must_use]
    pub const fn limit(self) -> usize {
        self.limit
    }

    #[must_use]
    pub const fn source_row_limit(self) -> usize {
        self.source_row_limit
    }
}

#[derive(Clone, Default)]
pub struct LocalSessionDirectoryCancellation {
    cancelled: Arc<AtomicBool>,
}

impl LocalSessionDirectoryCancellation {
    pub fn cancel(&self) {
        self.cancelled.store(true, Ordering::Release);
    }

    #[must_use]
    pub fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::Acquire)
    }
}

impl fmt::Debug for LocalSessionDirectoryCancellation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("LocalSessionDirectoryCancellation")
            .field("cancelled", &self.is_cancelled())
            .finish()
    }
}

pub trait LocalSessionDirectoryObservationPort {
    fn observe(
        &self,
        request: &LocalSessionDirectoryRequest,
        cancellation: &LocalSessionDirectoryCancellation,
    ) -> Result<Option<LocalSessionDirectoryPage>, ApplicationError>;
}

#[derive(Clone)]
pub struct ObserveLocalSessionDirectory<P> {
    port: P,
}

impl<P> ObserveLocalSessionDirectory<P> {
    #[must_use]
    pub const fn new(port: P) -> Self {
        Self { port }
    }
}

impl<P: LocalSessionDirectoryObservationPort> ObserveLocalSessionDirectory<P> {
    #[must_use]
    pub fn execute(
        &self,
        request: &LocalSessionDirectoryRequest,
        cancellation: &LocalSessionDirectoryCancellation,
    ) -> LoadCompletion<LocalSessionDirectoryPage> {
        if cancellation.is_cancelled() {
            return LoadCompletion::Failed(ApplicationError::cancelled(CANCELLED_CODE));
        }

        match self.port.observe(request, cancellation) {
            Ok(Some(page)) => LoadCompletion::Ready(page),
            Ok(None) => LoadCompletion::Empty,
            Err(error) => LoadCompletion::Failed(error),
        }
    }
}
