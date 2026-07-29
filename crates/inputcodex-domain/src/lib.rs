#![forbid(unsafe_code)]

mod application_overview;
mod diagnostic_log_observation;
mod platform_paths;
mod relay_environment_observation;
mod relay_status_observation;
mod runtime_environment_observation;
mod settings_observation;
mod version_startup;

pub use application_overview::{
    ApplicationOverview, ApplicationVersion, ApplicationVersionError, CollectedAtUnixMs,
    InstallationState, InstalledVersion, InstalledVersionUnknownReason, LiveProcessState,
    MAX_APPLICATION_VERSION_BYTES,
};
pub use diagnostic_log_observation::DiagnosticLogObservation;

pub use platform_paths::{
    ApplicationInstallSource, CodexInstallation, PlatformPathsSnapshot, PrivatePath,
    PrivatePathError,
};
pub use relay_environment_observation::{
    ClashConfigSource, ClashTunCandidateStatus, ClashTunObservation, CodexDotenvStatus,
    ObservationCoverageStatus, ProxyEnvironmentCoverage, ProxyEnvironmentSource,
    ProxyEnvironmentVariableName, ProxyEnvironmentVariableObservation, RelayEnvironmentObservation,
};
pub use relay_status_observation::{
    CredentialPresence, RelayConfigurationStatus, RelayDocumentStatus, RelayStatusObservation,
};
pub use runtime_environment_observation::{
    EnvironmentConflictSource, EnvironmentObservationStatus, EnvironmentSourceCoverage,
    EnvironmentValuePresence, EnvironmentVariableName, EnvironmentVariableNameError,
    RuntimeEnvironmentConflict, RuntimeEnvironmentConflictObservation,
};
pub use settings_observation::SettingsDocumentObservation;
pub use version_startup::{StartupIntent, VersionStartupSnapshot};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DiagnosticCode(&'static str);

impl DiagnosticCode {
    #[must_use]
    pub const fn new(value: &'static str) -> Self {
        Self(value)
    }

    #[must_use]
    pub const fn as_str(self) -> &'static str {
        self.0
    }
}
