use std::ffi::{OsStr, OsString};

use inputcodex_application::{ApplicationError, VersionStartupPort, VersionStartupRequest};
use inputcodex_domain::{ApplicationVersion, StartupIntent, VersionStartupSnapshot};

const SHOW_UPDATE_ARGUMENT: &str = "--show-update";
#[cfg(any(target_os = "windows", target_os = "macos"))]
const SHOW_UPDATE_ENVIRONMENT: &str = "INPUTCODEX_SHOW_UPDATE";

#[derive(Debug, Clone, Copy, Default)]
pub struct SystemVersionStartup;

impl VersionStartupPort for SystemVersionStartup {
    fn load(
        &self,
        _request: &VersionStartupRequest,
    ) -> Result<VersionStartupSnapshot, ApplicationError> {
        #[cfg(any(target_os = "windows", target_os = "macos"))]
        {
            resolve_version_startup(
                std::env::args_os(),
                std::env::var_os(SHOW_UPDATE_ENVIRONMENT),
            )
        }

        #[cfg(not(any(target_os = "windows", target_os = "macos")))]
        {
            Err(ApplicationError::unsupported(
                "VERSION_AND_STARTUP_UNSUPPORTED",
            ))
        }
    }
}

pub fn resolve_version_startup(
    process_arguments: impl IntoIterator<Item = OsString>,
    show_update_environment: Option<OsString>,
) -> Result<VersionStartupSnapshot, ApplicationError> {
    let startup_intent = match show_update_environment.as_deref() {
        None => startup_intent_from_arguments(process_arguments),
        Some(value) if value.is_empty() || value == OsStr::new("0") => {
            startup_intent_from_arguments(process_arguments)
        }
        Some(value) if value == OsStr::new("1") => StartupIntent::ShowUpdate,
        Some(_) => {
            return Err(ApplicationError::invalid_input("INVALID_STARTUP_OPTION"));
        }
    };
    let inputcodex_version = ApplicationVersion::new(env!("CARGO_PKG_VERSION").to_owned())
        .map_err(|_| ApplicationError::internal("VERSION_AND_STARTUP_BUILD_VERSION_INVALID"))?;

    Ok(VersionStartupSnapshot::new(
        inputcodex_version,
        startup_intent,
    ))
}

fn startup_intent_from_arguments(
    process_arguments: impl IntoIterator<Item = OsString>,
) -> StartupIntent {
    if process_arguments
        .into_iter()
        .skip(1)
        .any(|argument| argument == OsStr::new(SHOW_UPDATE_ARGUMENT))
    {
        StartupIntent::ShowUpdate
    } else {
        StartupIntent::Default
    }
}
