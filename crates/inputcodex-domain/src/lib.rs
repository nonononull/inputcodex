#![forbid(unsafe_code)]

mod application_overview;
mod context_entry_observation;
mod diagnostic_log_observation;
mod local_session_directory_observation;
mod markdown_generation;
mod platform_paths;
mod relay_environment_observation;
mod relay_status_observation;
mod runtime_environment_observation;
mod settings_observation;
mod version_startup;
mod watcher_preference_observation;
mod zed_remote_project_observation;

pub use application_overview::{
    ApplicationOverview, ApplicationVersion, ApplicationVersionError, CollectedAtUnixMs,
    InstallationState, InstalledVersion, InstalledVersionUnknownReason, LiveProcessState,
    MAX_APPLICATION_VERSION_BYTES,
};
pub use context_entry_observation::{
    ContextEntryCatalogObservation, ContextEntryCategorySummary, ContextEntryKind,
    ContextEntryObservation, ContextEntryObservationError,
};
pub use diagnostic_log_observation::DiagnosticLogObservation;

pub use local_session_directory_observation::{
    LocalSessionDirectoryEntry, LocalSessionDirectoryEntryError, LocalSessionDirectoryPage,
    LocalSessionDirectoryPageError, LocalSessionSourceCoverage, LocalSessionSourceSummary,
    LocalSessionSourceSummaryError, LocalSessionTitle, MAX_LOCAL_SESSION_DIRECTORY_PAGE_SIZE,
    MAX_LOCAL_SESSION_ID_BYTES, MAX_LOCAL_SESSION_TITLE_CHARS,
};
pub use markdown_generation::{
    MAX_MARKDOWN_FILENAME_BYTES, MAX_MARKDOWN_MESSAGE_COUNT, MAX_MARKDOWN_OUTPUT_BYTES,
    MarkdownGenerationError, MarkdownMessage, MarkdownMessageRole, MarkdownUtcTimestamp,
    SessionMarkdownDocument,
};

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
pub use watcher_preference_observation::{WatcherPreference, WatcherPreferenceObservation};
pub use zed_remote_project_observation::{
    MAX_ZED_REMOTE_PROJECT_SOURCES, MAX_ZED_REMOTE_PROJECTS, ZedRemoteProjectEntry,
    ZedRemoteProjectId, ZedRemoteProjectIdError, ZedRemoteProjectObservation,
    ZedRemoteProjectObservationError, ZedRemoteProjectOrigin, ZedRemoteProjectSelectionHint,
    ZedRemoteProjectSourceCoverage, ZedRemoteProjectSourceSummary,
    ZedRemoteProjectSourceSummaryError,
};

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
