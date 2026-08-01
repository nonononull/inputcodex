use std::{
    fmt,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
};

use inputcodex_domain::ZedRemoteProjectObservation;

use crate::{ApplicationError, LoadCompletion};

const CANCELLED_CODE: &str = "ZED_REMOTE_PROJECT_OBSERVATION_CANCELLED";

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ZedRemoteProjectObservationRequest;

#[derive(Clone, Default)]
pub struct ZedRemoteProjectObservationCancellation {
    cancelled: Arc<AtomicBool>,
}

impl ZedRemoteProjectObservationCancellation {
    pub fn cancel(&self) {
        self.cancelled.store(true, Ordering::Release);
    }

    #[must_use]
    pub fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::Acquire)
    }
}

impl fmt::Debug for ZedRemoteProjectObservationCancellation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ZedRemoteProjectObservationCancellation")
            .field("cancelled", &self.is_cancelled())
            .finish()
    }
}

pub trait ZedRemoteProjectObservationPort {
    fn observe(
        &self,
        request: &ZedRemoteProjectObservationRequest,
        cancellation: &ZedRemoteProjectObservationCancellation,
    ) -> Result<Option<ZedRemoteProjectObservation>, ApplicationError>;
}

#[derive(Clone)]
pub struct ObserveZedRemoteProjects<P> {
    port: P,
}

impl<P> ObserveZedRemoteProjects<P> {
    #[must_use]
    pub const fn new(port: P) -> Self {
        Self { port }
    }
}

impl<P: ZedRemoteProjectObservationPort> ObserveZedRemoteProjects<P> {
    #[must_use]
    pub fn execute(
        &self,
        request: &ZedRemoteProjectObservationRequest,
        cancellation: &ZedRemoteProjectObservationCancellation,
    ) -> LoadCompletion<ZedRemoteProjectObservation> {
        if cancellation.is_cancelled() {
            return LoadCompletion::Failed(ApplicationError::cancelled(CANCELLED_CODE));
        }

        match self.port.observe(request, cancellation) {
            Ok(Some(observation)) => LoadCompletion::Ready(observation),
            Ok(None) => LoadCompletion::Empty,
            Err(error) => LoadCompletion::Failed(error),
        }
    }
}
