use inputcodex_domain::RelayStatusObservation;

use crate::{ApplicationError, LoadCompletion};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct RelayStatusObservationRequest;

pub trait RelayStatusObservationPort {
    fn observe(
        &self,
        request: &RelayStatusObservationRequest,
    ) -> Result<Option<RelayStatusObservation>, ApplicationError>;
}

#[derive(Clone)]
pub struct ObserveRelayStatus<P> {
    port: P,
}

impl<P> ObserveRelayStatus<P> {
    #[must_use]
    pub const fn new(port: P) -> Self {
        Self { port }
    }
}

impl<P: RelayStatusObservationPort> ObserveRelayStatus<P> {
    #[must_use]
    pub fn execute(
        &self,
        request: &RelayStatusObservationRequest,
    ) -> LoadCompletion<RelayStatusObservation> {
        match self.port.observe(request) {
            Ok(Some(observation)) => LoadCompletion::Ready(observation),
            Ok(None) => LoadCompletion::Empty,
            Err(error) => LoadCompletion::Failed(error),
        }
    }
}
