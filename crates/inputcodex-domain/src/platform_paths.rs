use std::{
    fmt,
    path::{Path, PathBuf},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrivatePathError {
    Empty,
    Relative,
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct PrivatePath(PathBuf);

impl PrivatePath {
    pub fn new(path: PathBuf) -> Result<Self, PrivatePathError> {
        if path.as_os_str().is_empty() {
            return Err(PrivatePathError::Empty);
        }
        if !path.is_absolute() {
            return Err(PrivatePathError::Relative);
        }
        Ok(Self(path))
    }

    #[must_use]
    pub fn as_path(&self) -> &Path {
        &self.0
    }
}

impl fmt::Debug for PrivatePath {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("PrivatePath(<redacted>)")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApplicationInstallSource {
    Explicit,
    WindowsPackage,
    WindowsStandalone,
    MacosSystemApplications,
    MacosUserApplications,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CodexInstallation {
    application_root: PrivatePath,
    executable: PrivatePath,
    source: ApplicationInstallSource,
}

impl CodexInstallation {
    #[must_use]
    pub const fn new(
        application_root: PrivatePath,
        executable: PrivatePath,
        source: ApplicationInstallSource,
    ) -> Self {
        Self {
            application_root,
            executable,
            source,
        }
    }

    #[must_use]
    pub const fn application_root(&self) -> &PrivatePath {
        &self.application_root
    }

    #[must_use]
    pub const fn executable(&self) -> &PrivatePath {
        &self.executable
    }

    #[must_use]
    pub const fn source(&self) -> ApplicationInstallSource {
        self.source
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlatformPathsSnapshot {
    codex_home: PrivatePath,
    inputcodex_state_root: PrivatePath,
    settings_file: PrivatePath,
    latest_status_file: PrivatePath,
    diagnostic_log_file: PrivatePath,
    codex_installation: Option<CodexInstallation>,
}

impl PlatformPathsSnapshot {
    #[must_use]
    pub const fn new(
        codex_home: PrivatePath,
        inputcodex_state_root: PrivatePath,
        settings_file: PrivatePath,
        latest_status_file: PrivatePath,
        diagnostic_log_file: PrivatePath,
        codex_installation: Option<CodexInstallation>,
    ) -> Self {
        Self {
            codex_home,
            inputcodex_state_root,
            settings_file,
            latest_status_file,
            diagnostic_log_file,
            codex_installation,
        }
    }

    #[must_use]
    pub const fn codex_home(&self) -> &PrivatePath {
        &self.codex_home
    }

    #[must_use]
    pub const fn inputcodex_state_root(&self) -> &PrivatePath {
        &self.inputcodex_state_root
    }

    #[must_use]
    pub const fn settings_file(&self) -> &PrivatePath {
        &self.settings_file
    }

    #[must_use]
    pub const fn latest_status_file(&self) -> &PrivatePath {
        &self.latest_status_file
    }

    #[must_use]
    pub const fn diagnostic_log_file(&self) -> &PrivatePath {
        &self.diagnostic_log_file
    }

    #[must_use]
    pub const fn codex_installation(&self) -> Option<&CodexInstallation> {
        self.codex_installation.as_ref()
    }
}
