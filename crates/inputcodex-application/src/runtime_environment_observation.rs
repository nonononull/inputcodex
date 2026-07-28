use inputcodex_domain::RuntimeEnvironmentConflictObservation;

use crate::{ApplicationError, LoadCompletion};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct RuntimeEnvironmentObservationRequest;

pub trait RuntimeEnvironmentObservationPort {
    fn observe(
        &self,
        request: &RuntimeEnvironmentObservationRequest,
    ) -> Result<RuntimeEnvironmentConflictObservation, ApplicationError>;
}

#[derive(Clone)]
pub struct ObserveRuntimeEnvironmentConflicts<P> {
    port: P,
}

impl<P> ObserveRuntimeEnvironmentConflicts<P> {
    #[must_use]
    pub const fn new(port: P) -> Self {
        Self { port }
    }
}

impl<P: RuntimeEnvironmentObservationPort> ObserveRuntimeEnvironmentConflicts<P> {
    #[must_use]
    pub fn execute(
        &self,
        request: &RuntimeEnvironmentObservationRequest,
    ) -> LoadCompletion<RuntimeEnvironmentConflictObservation> {
        match self.port.observe(request) {
            Ok(observation) => LoadCompletion::Ready(observation),
            Err(error) => LoadCompletion::Failed(error),
        }
    }
}
