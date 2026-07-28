use inputcodex_domain::SettingsDocumentObservation;

use crate::{ApplicationError, LoadCompletion};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct SettingsObservationRequest;

pub trait SettingsObservationPort {
    fn observe(
        &self,
        request: &SettingsObservationRequest,
    ) -> Result<Option<SettingsDocumentObservation>, ApplicationError>;
}

#[derive(Clone)]
pub struct ObserveSettings<P> {
    port: P,
}

impl<P> ObserveSettings<P> {
    #[must_use]
    pub const fn new(port: P) -> Self {
        Self { port }
    }
}

impl<P: SettingsObservationPort> ObserveSettings<P> {
    #[must_use]
    pub fn execute(
        &self,
        request: &SettingsObservationRequest,
    ) -> LoadCompletion<SettingsDocumentObservation> {
        match self.port.observe(request) {
            Ok(Some(observation)) => LoadCompletion::Ready(observation),
            Ok(None) => LoadCompletion::Empty,
            Err(error) => LoadCompletion::Failed(error),
        }
    }
}
