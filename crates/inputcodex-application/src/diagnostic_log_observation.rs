use inputcodex_domain::DiagnosticLogObservation;

use crate::{ApplicationError, LoadCompletion};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct DiagnosticLogObservationRequest;

pub trait DiagnosticLogObservationPort {
    fn observe(
        &self,
        request: &DiagnosticLogObservationRequest,
    ) -> Result<Option<DiagnosticLogObservation>, ApplicationError>;
}

#[derive(Clone)]
pub struct ObserveDiagnosticLog<P> {
    port: P,
}

impl<P> ObserveDiagnosticLog<P> {
    #[must_use]
    pub const fn new(port: P) -> Self {
        Self { port }
    }
}

impl<P: DiagnosticLogObservationPort> ObserveDiagnosticLog<P> {
    #[must_use]
    pub fn execute(
        &self,
        request: &DiagnosticLogObservationRequest,
    ) -> LoadCompletion<DiagnosticLogObservation> {
        match self.port.observe(request) {
            Ok(Some(observation)) => LoadCompletion::Ready(observation),
            Ok(None) => LoadCompletion::Empty,
            Err(error) => LoadCompletion::Failed(error),
        }
    }
}
