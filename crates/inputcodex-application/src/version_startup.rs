use inputcodex_domain::VersionStartupSnapshot;

use crate::{ApplicationError, LoadCompletion};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct VersionStartupRequest;

pub trait VersionStartupPort {
    fn load(
        &self,
        request: &VersionStartupRequest,
    ) -> Result<VersionStartupSnapshot, ApplicationError>;
}

#[derive(Clone)]
pub struct LoadVersionStartup<P> {
    port: P,
}

impl<P> LoadVersionStartup<P> {
    #[must_use]
    pub const fn new(port: P) -> Self {
        Self { port }
    }

    #[must_use]
    pub const fn port(&self) -> &P {
        &self.port
    }
}

impl<P: VersionStartupPort> LoadVersionStartup<P> {
    pub fn execute(
        &self,
        request: &VersionStartupRequest,
    ) -> LoadCompletion<VersionStartupSnapshot> {
        match self.port.load(request) {
            Ok(snapshot) => LoadCompletion::Ready(snapshot),
            Err(error) => LoadCompletion::Failed(error),
        }
    }
}
