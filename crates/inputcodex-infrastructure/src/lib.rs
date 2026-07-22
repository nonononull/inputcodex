#![forbid(unsafe_code)]

use inputcodex_application::{ApplicationError, LoadPort, RequestId};

#[derive(Debug, Clone, Copy, Default)]
pub struct UnconfiguredLoadPort;

impl<T> LoadPort<T> for UnconfiguredLoadPort {
    fn load(&self, _request_id: RequestId) -> Result<Option<T>, ApplicationError> {
        Err(ApplicationError::unavailable("LOAD_SOURCE_UNCONFIGURED"))
    }
}
