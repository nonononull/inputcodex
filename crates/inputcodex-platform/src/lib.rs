#![forbid(unsafe_code)]

use inputcodex_application::{ApplicationError, PlatformKind, PlatformPort};

mod application_overview;
mod context_entry_observation;
mod diagnostic_log_observation;
mod platform_paths;
mod relay_environment_observation;
mod relay_status_observation;
mod runtime_environment_observation;
mod settings_observation;
mod version_startup;

pub use application_overview::SystemApplicationOverview;
pub use context_entry_observation::SystemContextEntryObservation;
pub use diagnostic_log_observation::SystemDiagnosticLogObservation;
pub use platform_paths::SystemPlatformPaths;
pub use relay_environment_observation::SystemRelayEnvironmentObservation;
pub use relay_status_observation::SystemRelayStatusObservation;
pub use runtime_environment_observation::{
    SystemRuntimeEnvironmentObservation, observe_macos_runtime_environment,
    observe_windows_runtime_environment,
};
pub use settings_observation::SystemSettingsObservation;
pub use version_startup::{SystemVersionStartup, resolve_version_startup};

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
