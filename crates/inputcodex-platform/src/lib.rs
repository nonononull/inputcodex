#![forbid(unsafe_code)]

use inputcodex_application::{ApplicationError, PlatformKind, PlatformPort};

#[derive(Debug, Clone, Copy, Default)]
pub struct SystemPlatform;

impl PlatformPort for SystemPlatform {
    fn current_platform(&self) -> Result<PlatformKind, ApplicationError> {
        #[cfg(target_os = "windows")]
        {
            Ok(PlatformKind::Windows)
        }

        #[cfg(target_os = "macos")]
        {
            Ok(PlatformKind::Macos)
        }

        #[cfg(not(any(target_os = "windows", target_os = "macos")))]
        {
            Err(ApplicationError::unsupported("PLATFORM_UNSUPPORTED"))
        }
    }
}
