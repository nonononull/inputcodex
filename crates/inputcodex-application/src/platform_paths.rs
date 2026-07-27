use std::{
    fmt,
    path::{Path, PathBuf},
};

use inputcodex_domain::PlatformPathsSnapshot;

use crate::{ApplicationError, LoadCompletion};

#[derive(Clone, Default, PartialEq, Eq)]
pub struct PlatformPathsRequest {
    explicit_application_path: Option<PathBuf>,
}

impl PlatformPathsRequest {
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

impl fmt::Debug for PlatformPathsRequest {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = self
            .explicit_application_path
            .as_ref()
            .map(|_| "<redacted>");
        formatter
            .debug_struct("PlatformPathsRequest")
            .field("explicit_application_path", &value)
            .finish()
    }
}

pub trait PlatformPathsPort {
    fn resolve(
        &self,
        request: &PlatformPathsRequest,
    ) -> Result<PlatformPathsSnapshot, ApplicationError>;
}

#[derive(Clone)]
pub struct ResolvePlatformPaths<P> {
    port: P,
}

impl<P> ResolvePlatformPaths<P> {
    #[must_use]
    pub const fn new(port: P) -> Self {
        Self { port }
    }

    #[must_use]
    pub const fn port(&self) -> &P {
        &self.port
    }
}

impl<P: PlatformPathsPort> ResolvePlatformPaths<P> {
    pub fn execute(&self, request: &PlatformPathsRequest) -> LoadCompletion<PlatformPathsSnapshot> {
        match self.port.resolve(request) {
            Ok(snapshot) => LoadCompletion::Ready(snapshot),
            Err(error) => LoadCompletion::Failed(error),
        }
    }
}
