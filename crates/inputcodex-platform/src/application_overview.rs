use std::{
    fs::File,
    io::{self, Read},
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};

use inputcodex_application::{
    ApplicationError, ApplicationOverviewPort, ApplicationOverviewRequest,
};
use inputcodex_domain::{
    ApplicationOverview, ApplicationVersion, CodexInstallation, CollectedAtUnixMs,
    InstallationState, InstalledVersion, LiveProcessState,
};

#[cfg(any(target_os = "windows", target_os = "macos"))]
use crate::platform_paths::SystemPathProbe;

#[cfg(any(target_os = "macos", test))]
mod macos;
#[cfg(any(target_os = "windows", test))]
mod windows;

pub(crate) enum LimitedRead {
    Bytes(Vec<u8>),
    TooLarge,
}

pub(crate) trait MetadataReader {
    fn read_limited(&self, path: &Path, limit: usize) -> io::Result<LimitedRead>;
}

#[derive(Debug, Clone, Copy, Default)]
struct SystemMetadataReader;

impl MetadataReader for SystemMetadataReader {
    fn read_limited(&self, path: &Path, limit: usize) -> io::Result<LimitedRead> {
        let file = File::open(path)?;
        let mut bytes = Vec::with_capacity(limit.min(8192) + 1);
        file.take((limit + 1) as u64).read_to_end(&mut bytes)?;
        if bytes.len() > limit {
            Ok(LimitedRead::TooLarge)
        } else {
            Ok(LimitedRead::Bytes(bytes))
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct SystemApplicationOverview;

impl ApplicationOverviewPort for SystemApplicationOverview {
    fn load(
        &self,
        request: &ApplicationOverviewRequest,
    ) -> Result<ApplicationOverview, ApplicationError> {
        #[cfg(target_os = "windows")]
        {
            let installation = crate::platform_paths::windows::resolve_installation_system(
                request.explicit_application_path(),
                &SystemPathProbe,
            )
            .map_err(map_discovery_error)?;
            build_overview(installation, |installation| {
                windows::resolve_installed_version(installation, &SystemMetadataReader)
            })
        }

        #[cfg(target_os = "macos")]
        {
            let installation = crate::platform_paths::macos::resolve_installation_system(
                request.explicit_application_path(),
                &SystemPathProbe,
            )
            .map_err(map_discovery_error)?;
            build_overview(installation, |installation| {
                macos::resolve_installed_version(installation, &SystemMetadataReader)
            })
        }

        #[cfg(not(any(target_os = "windows", target_os = "macos")))]
        {
            let _ = request;
            Err(ApplicationError::unsupported(
                "APPLICATION_OVERVIEW_UNSUPPORTED",
            ))
        }
    }
}

fn build_overview(
    installation: Option<CodexInstallation>,
    resolve_version: impl FnOnce(&CodexInstallation) -> InstalledVersion,
) -> Result<ApplicationOverview, ApplicationError> {
    let installation = match installation {
        Some(installation) => {
            let version = resolve_version(&installation);
            InstallationState::Installed {
                installation,
                version,
            }
        }
        None => InstallationState::NotInstalled,
    };
    let inputcodex_version = ApplicationVersion::new(env!("CARGO_PKG_VERSION").to_owned())
        .map_err(|_| ApplicationError::internal("APPLICATION_OVERVIEW_BUILD_VERSION_INVALID"))?;
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| ApplicationError::internal("APPLICATION_OVERVIEW_TIME_UNAVAILABLE"))?;
    let collected_at = u64::try_from(duration.as_millis())
        .map(CollectedAtUnixMs::new)
        .map_err(|_| ApplicationError::internal("APPLICATION_OVERVIEW_TIME_UNAVAILABLE"))?;

    Ok(ApplicationOverview::new(
        installation,
        inputcodex_version,
        LiveProcessState::NotObserved,
        collected_at,
    ))
}

fn map_discovery_error(error: ApplicationError) -> ApplicationError {
    if error.code().as_str() == "EXPLICIT_CODEX_PATH_INVALID" {
        error
    } else {
        ApplicationError::internal("APPLICATION_OVERVIEW_DISCOVERY_FAILED")
    }
}
