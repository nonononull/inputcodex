use std::{
    fmt,
    path::{Path, PathBuf},
};

use inputcodex_domain::ApplicationOverview;

use crate::{ApplicationError, LoadCompletion};

#[derive(Clone, Default, PartialEq, Eq)]
pub struct ApplicationOverviewRequest {
    explicit_application_path: Option<PathBuf>,
}

impl ApplicationOverviewRequest {
    #[must_use]
    pub const fn new(explicit_application_path: Option<PathBuf>) -> Self {
        Self {
            explicit_application_path,
        }
    }

    #[must_use]
    pub fn explicit_application_path(&self) -> Option<&Path> {
        self.explicit_application_path.as_deref()
    }
}

impl fmt::Debug for ApplicationOverviewRequest {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = self
            .explicit_application_path
            .as_ref()
            .map(|_| "<redacted>");
        formatter
            .debug_struct("ApplicationOverviewRequest")
            .field("explicit_application_path", &value)
            .finish()
    }
}

pub trait ApplicationOverviewPort {
    fn load(
        &self,
        request: &ApplicationOverviewRequest,
    ) -> Result<ApplicationOverview, ApplicationError>;
}

#[derive(Clone)]
pub struct LoadApplicationOverview<P> {
    port: P,
}

impl<P> LoadApplicationOverview<P> {
    #[must_use]
    pub const fn new(port: P) -> Self {
        Self { port }
    }

    #[must_use]
    pub const fn port(&self) -> &P {
        &self.port
    }
}

impl<P: ApplicationOverviewPort> LoadApplicationOverview<P> {
    pub fn execute(
        &self,
        request: &ApplicationOverviewRequest,
    ) -> LoadCompletion<ApplicationOverview> {
        match self.port.load(request) {
            Ok(overview) => LoadCompletion::Ready(overview),
            Err(error) => LoadCompletion::Failed(error),
        }
    }
}
