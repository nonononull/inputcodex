use inputcodex_domain::WatcherPreferenceObservation;

use crate::{ApplicationError, LoadCompletion};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct WatcherPreferenceObservationRequest;

pub trait WatcherPreferenceObservationPort {
    fn observe(
        &self,
        request: &WatcherPreferenceObservationRequest,
    ) -> Result<WatcherPreferenceObservation, ApplicationError>;
}

#[derive(Clone)]
pub struct ObserveWatcherPreference<P> {
    port: P,
}

impl<P> ObserveWatcherPreference<P> {
    #[must_use]
    pub const fn new(port: P) -> Self {
        Self { port }
    }
}

impl<P: WatcherPreferenceObservationPort> ObserveWatcherPreference<P> {
    #[must_use]
    pub fn execute(
        &self,
        request: &WatcherPreferenceObservationRequest,
    ) -> LoadCompletion<WatcherPreferenceObservation> {
        match self.port.observe(request) {
            Ok(observation) => LoadCompletion::Ready(observation),
            Err(error) => LoadCompletion::Failed(error),
        }
    }
}
