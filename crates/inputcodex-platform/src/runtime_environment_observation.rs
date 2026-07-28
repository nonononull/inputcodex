use std::ffi::{OsStr, OsString};

use inputcodex_application::{
    ApplicationError, RuntimeEnvironmentObservationPort, RuntimeEnvironmentObservationRequest,
};
use inputcodex_domain::{
    EnvironmentValuePresence, EnvironmentVariableName, RuntimeEnvironmentConflict,
    RuntimeEnvironmentConflictObservation,
};

const OPENAI_PREFIX: &[u8] = b"OPENAI_";

#[derive(Debug, Clone, Copy, Default)]
pub struct SystemRuntimeEnvironmentObservation;

impl RuntimeEnvironmentObservationPort for SystemRuntimeEnvironmentObservation {
    fn observe(
        &self,
        _request: &RuntimeEnvironmentObservationRequest,
    ) -> Result<RuntimeEnvironmentConflictObservation, ApplicationError> {
        #[cfg(any(target_os = "windows", target_os = "macos"))]
        {
            observe_current_runtime_environment(std::env::vars_os())
        }

        #[cfg(not(any(target_os = "windows", target_os = "macos")))]
        {
            Err(ApplicationError::unsupported(
                "RUNTIME_ENVIRONMENT_OBSERVATION_UNSUPPORTED",
            ))
        }
    }
}

#[cfg(target_os = "windows")]
fn observe_current_runtime_environment(
    pairs: impl IntoIterator<Item = (OsString, OsString)>,
) -> Result<RuntimeEnvironmentConflictObservation, ApplicationError> {
    observe_windows_runtime_environment(pairs)
}

#[cfg(target_os = "macos")]
fn observe_current_runtime_environment(
    pairs: impl IntoIterator<Item = (OsString, OsString)>,
) -> Result<RuntimeEnvironmentConflictObservation, ApplicationError> {
    observe_macos_runtime_environment(pairs)
}

pub fn observe_windows_runtime_environment(
    pairs: impl IntoIterator<Item = (OsString, OsString)>,
) -> Result<RuntimeEnvironmentConflictObservation, ApplicationError> {
    observe_runtime_environment(pairs, NamePolicy::Windows)
}

pub fn observe_macos_runtime_environment(
    pairs: impl IntoIterator<Item = (OsString, OsString)>,
) -> Result<RuntimeEnvironmentConflictObservation, ApplicationError> {
    observe_runtime_environment(pairs, NamePolicy::Macos)
}

#[derive(Debug, Clone, Copy)]
enum NamePolicy {
    Windows,
    Macos,
}

fn observe_runtime_environment(
    pairs: impl IntoIterator<Item = (OsString, OsString)>,
    policy: NamePolicy,
) -> Result<RuntimeEnvironmentConflictObservation, ApplicationError> {
    let mut scanned_entry_count = 0;
    let mut conflicts = Vec::new();

    for (name, value) in pairs {
        scanned_entry_count += 1;
        if !is_candidate_name(&name, policy) {
            continue;
        }

        let normalized_name = normalize_candidate_name(name, policy)?;
        let name = EnvironmentVariableName::new(normalized_name)
            .map_err(|_| name_unrepresentable_error())?;
        let value_presence = if value.is_empty() {
            EnvironmentValuePresence::Empty
        } else {
            EnvironmentValuePresence::NonEmpty
        };
        conflicts.push(RuntimeEnvironmentConflict::runtime_process(
            name,
            value_presence,
        ));
    }

    Ok(RuntimeEnvironmentConflictObservation::new(
        scanned_entry_count,
        conflicts,
    ))
}

fn is_candidate_name(name: &OsStr, policy: NamePolicy) -> bool {
    let encoded = name.as_encoded_bytes();
    let Some(prefix) = encoded.get(..OPENAI_PREFIX.len()) else {
        return false;
    };

    match policy {
        NamePolicy::Windows => prefix.eq_ignore_ascii_case(OPENAI_PREFIX),
        NamePolicy::Macos => prefix == OPENAI_PREFIX,
    }
}

fn normalize_candidate_name(
    name: OsString,
    policy: NamePolicy,
) -> Result<String, ApplicationError> {
    let name = name
        .into_string()
        .map_err(|_| name_unrepresentable_error())?;
    match policy {
        NamePolicy::Windows => Ok(name.to_uppercase()),
        NamePolicy::Macos => Ok(name),
    }
}

const fn name_unrepresentable_error() -> ApplicationError {
    ApplicationError::internal("RUNTIME_ENVIRONMENT_NAME_UNREPRESENTABLE")
}
