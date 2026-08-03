#![forbid(unsafe_code)]

use inputcodex_application::{ApplicationError, PlatformKind, PlatformPort};

mod application_overview;
mod context_entry_observation;
mod diagnostic_log_observation;
#[cfg(any(target_os = "windows", target_os = "macos"))]
mod local_session_directory_observation;
#[cfg(not(any(target_os = "windows", target_os = "macos")))]
mod local_session_directory_observation {
    use inputcodex_application::{
        ApplicationError, LocalSessionDirectoryCancellation, LocalSessionDirectoryObservationPort,
        LocalSessionDirectoryRequest,
    };
    use inputcodex_domain::LocalSessionDirectoryPage;

    #[derive(Debug, Clone, Copy, Default)]
    pub struct SystemLocalSessionDirectoryObservation;

    impl LocalSessionDirectoryObservationPort for SystemLocalSessionDirectoryObservation {
        fn observe(
            &self,
            request: &LocalSessionDirectoryRequest,
            cancellation: &LocalSessionDirectoryCancellation,
        ) -> Result<Option<LocalSessionDirectoryPage>, ApplicationError> {
            let _ = (request, cancellation);
            Err(ApplicationError::unsupported(
                "LOCAL_SESSION_DIRECTORY_UNSUPPORTED",
            ))
        }
    }
}
#[cfg(any(target_os = "windows", target_os = "macos"))]
mod markdown_generation;
#[cfg(not(any(target_os = "windows", target_os = "macos")))]
mod markdown_generation {
    use inputcodex_application::{
        ApplicationError, MarkdownGenerationCancellation, MarkdownGenerationPort,
        MarkdownGenerationRequest,
    };
    use inputcodex_domain::SessionMarkdownDocument;

    #[derive(Debug, Clone, Copy, Default)]
    pub struct SystemMarkdownGeneration;

    impl MarkdownGenerationPort for SystemMarkdownGeneration {
        fn generate(
            &self,
            request: &MarkdownGenerationRequest,
            cancellation: &MarkdownGenerationCancellation,
        ) -> Result<Option<SessionMarkdownDocument>, ApplicationError> {
            let _ = (request, cancellation);
            Err(ApplicationError::unsupported(
                "MARKDOWN_GENERATION_UNSUPPORTED",
            ))
        }
    }
}
mod platform_paths;
mod relay_environment_observation;
mod relay_status_observation;
mod runtime_environment_observation;
mod settings_observation;
mod version_startup;
mod watcher_preference_mutation;
mod watcher_preference_observation;

pub use application_overview::SystemApplicationOverview;
pub use context_entry_observation::SystemContextEntryObservation;
pub use diagnostic_log_observation::SystemDiagnosticLogObservation;
pub use local_session_directory_observation::SystemLocalSessionDirectoryObservation;
pub use markdown_generation::SystemMarkdownGeneration;
pub use platform_paths::SystemPlatformPaths;
pub use relay_environment_observation::SystemRelayEnvironmentObservation;
pub use relay_status_observation::SystemRelayStatusObservation;
pub use runtime_environment_observation::{
    SystemRuntimeEnvironmentObservation, observe_macos_runtime_environment,
    observe_windows_runtime_environment,
};
pub use settings_observation::SystemSettingsObservation;
pub use version_startup::{SystemVersionStartup, resolve_version_startup};
pub use watcher_preference_mutation::SystemWatcherPreferenceMutation;
pub use watcher_preference_observation::SystemWatcherPreferenceObservation;

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
