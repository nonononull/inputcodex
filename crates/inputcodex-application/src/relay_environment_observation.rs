use inputcodex_domain::RelayEnvironmentObservation;

use crate::{ApplicationError, LoadCompletion};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct RelayEnvironmentObservationRequest;

pub trait RelayEnvironmentObservationPort {
    fn observe(
        &self,
        request: &RelayEnvironmentObservationRequest,
    ) -> Result<RelayEnvironmentObservation, ApplicationError>;
}

#[derive(Clone)]
pub struct ObserveRelayEnvironment<P> {
    port: P,
}

impl<P> ObserveRelayEnvironment<P> {
    #[must_use]
    pub const fn new(port: P) -> Self {
        Self { port }
    }
}

impl<P: RelayEnvironmentObservationPort> ObserveRelayEnvironment<P> {
    #[must_use]
    pub fn execute(
        &self,
        request: &RelayEnvironmentObservationRequest,
    ) -> LoadCompletion<RelayEnvironmentObservation> {
        match self.port.observe(request) {
            Ok(observation) => LoadCompletion::Ready(observation),
            Err(error) => LoadCompletion::Failed(error),
        }
    }
}
