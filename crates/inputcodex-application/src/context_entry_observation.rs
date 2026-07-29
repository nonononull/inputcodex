use inputcodex_domain::ContextEntryCatalogObservation;

use crate::{ApplicationError, LoadCompletion};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ContextEntryObservationRequest;

pub trait ContextEntryObservationPort {
    fn observe(
        &self,
        request: &ContextEntryObservationRequest,
    ) -> Result<Option<ContextEntryCatalogObservation>, ApplicationError>;
}

#[derive(Clone)]
pub struct ObserveContextEntries<P> {
    port: P,
}

impl<P> ObserveContextEntries<P> {
    #[must_use]
    pub const fn new(port: P) -> Self {
        Self { port }
    }
}

impl<P: ContextEntryObservationPort> ObserveContextEntries<P> {
    #[must_use]
    pub fn execute(
        &self,
        request: &ContextEntryObservationRequest,
    ) -> LoadCompletion<ContextEntryCatalogObservation> {
        match self.port.observe(request) {
            Ok(Some(observation)) => LoadCompletion::Ready(observation),
            Ok(None) => LoadCompletion::Empty,
            Err(error) => LoadCompletion::Failed(error),
        }
    }
}
