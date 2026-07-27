use crate::CodexInstallation;

pub const MAX_APPLICATION_VERSION_BYTES: usize = 128;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApplicationVersionError {
    Empty,
    TooLong,
    ControlCharacter,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ApplicationVersion(String);

impl ApplicationVersion {
    pub fn new(value: String) -> Result<Self, ApplicationVersionError> {
        let value = value.trim();
        if value.is_empty() {
            return Err(ApplicationVersionError::Empty);
        }
        if value.len() > MAX_APPLICATION_VERSION_BYTES {
            return Err(ApplicationVersionError::TooLong);
        }
        if value.chars().any(char::is_control) {
            return Err(ApplicationVersionError::ControlCharacter);
        }
        Ok(Self(value.to_owned()))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstalledVersionUnknownReason {
    MetadataMissing,
    MetadataUnreadable,
    MetadataInvalid,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InstalledVersion {
    Known(ApplicationVersion),
    Unknown(InstalledVersionUnknownReason),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InstallationState {
    Installed {
        installation: CodexInstallation,
        version: InstalledVersion,
    },
    NotInstalled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LiveProcessState {
    NotObserved,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CollectedAtUnixMs(u64);

impl CollectedAtUnixMs {
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    #[must_use]
    pub const fn value(self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApplicationOverview {
    installation: InstallationState,
    inputcodex_version: ApplicationVersion,
    live_process_state: LiveProcessState,
    collected_at: CollectedAtUnixMs,
}

impl ApplicationOverview {
    #[must_use]
    pub const fn new(
        installation: InstallationState,
        inputcodex_version: ApplicationVersion,
        live_process_state: LiveProcessState,
        collected_at: CollectedAtUnixMs,
    ) -> Self {
        Self {
            installation,
            inputcodex_version,
            live_process_state,
            collected_at,
        }
    }

    #[must_use]
    pub const fn installation(&self) -> &InstallationState {
        &self.installation
    }

    #[must_use]
    pub const fn inputcodex_version(&self) -> &ApplicationVersion {
        &self.inputcodex_version
    }

    #[must_use]
    pub const fn live_process_state(&self) -> LiveProcessState {
        self.live_process_state
    }

    #[must_use]
    pub const fn collected_at(&self) -> CollectedAtUnixMs {
        self.collected_at
    }
}
