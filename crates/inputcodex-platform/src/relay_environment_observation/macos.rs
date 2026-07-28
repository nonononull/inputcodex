use std::{ffi::OsString, path::PathBuf};

use inputcodex_application::ApplicationError;
use inputcodex_domain::RelayEnvironmentObservation;

#[cfg(target_os = "macos")]
use crate::platform_paths::SystemPathProbe;
use crate::platform_paths::{PathProbe, resolve_codex_home};

#[cfg(target_os = "macos")]
use super::SystemRelayFileProbe;
use super::{
    PersistentEnvironment, RelayFileProbe, RelayObservationInputs, clash_candidate_paths,
    observe_with_inputs,
};

struct MacosObservationInputs {
    runtime_environment: Vec<(OsString, OsString)>,
    user_home: Option<PathBuf>,
    codex_home: Option<OsString>,
}

#[cfg(target_os = "macos")]
pub(super) fn observe_system() -> Result<RelayEnvironmentObservation, ApplicationError> {
    observe_macos_with_inputs(
        MacosObservationInputs {
            runtime_environment: std::env::vars_os().collect(),
            user_home: std::env::var_os("HOME").map(PathBuf::from),
            codex_home: std::env::var_os("CODEX_HOME"),
        },
        &SystemPathProbe,
        &SystemRelayFileProbe,
    )
}

fn observe_macos_with_inputs(
    inputs: MacosObservationInputs,
    path_probe: &impl PathProbe,
    file_probe: &impl RelayFileProbe,
) -> Result<RelayEnvironmentObservation, ApplicationError> {
    let user_home = inputs.user_home.clone();
    let codex_home = resolve_codex_home(inputs.user_home, inputs.codex_home, path_probe)?;
    let user_home = user_home
        .filter(|path| path.is_absolute())
        .ok_or_else(|| ApplicationError::unavailable("USER_HOME_UNAVAILABLE"))?;
    let platform_root = user_home.join("Library/Application Support");

    observe_with_inputs(
        RelayObservationInputs {
            runtime_environment: inputs.runtime_environment,
            persistent_user: PersistentEnvironment::NotObserved,
            persistent_system: PersistentEnvironment::NotObserved,
            codex_home,
            clash_candidates: clash_candidate_paths(&platform_root, &platform_root, &user_home),
        },
        file_probe,
    )
}

#[cfg(test)]
mod tests {
    use std::{
        collections::HashSet,
        ffi::OsString,
        io,
        path::{Path, PathBuf},
    };

    use inputcodex_domain::{
        ObservationCoverageStatus, ProxyEnvironmentSource, ProxyEnvironmentVariableName,
    };

    use crate::platform_paths::PathProbe;

    use super::super::{FileMetadata, LimitedRead, RelayFileProbe};
    use super::*;

    #[derive(Default)]
    struct MemoryPathProbe {
        directories: HashSet<PathBuf>,
    }

    impl PathProbe for MemoryPathProbe {
        fn is_dir(&self, path: &Path) -> bool {
            self.directories.contains(path)
        }

        fn is_file(&self, _path: &Path) -> bool {
            false
        }
    }

    struct MissingFileProbe;

    impl RelayFileProbe for MissingFileProbe {
        fn metadata(&self, _path: &Path) -> io::Result<FileMetadata> {
            Err(io::Error::from(io::ErrorKind::NotFound))
        }

        fn read_limited(&self, _path: &Path, _limit: usize) -> io::Result<LimitedRead> {
            Err(io::Error::from(io::ErrorKind::NotFound))
        }
    }

    #[test]
    fn macos_仅观察进程代理且持久化来源明确_not_observed() {
        let sentinel = "http://macos-private.invalid";
        let home = std::env::temp_dir().join("inputcodex-macos-relay-home");
        let mut path_probe = MemoryPathProbe::default();
        path_probe.directories.insert(home.clone());

        let observation = observe_macos_with_inputs(
            MacosObservationInputs {
                runtime_environment: vec![(OsString::from("all_proxy"), OsString::from(sentinel))],
                user_home: Some(home),
                codex_home: None,
            },
            &path_probe,
            &MissingFileProbe,
        )
        .expect("macOS 观察应成功");

        assert_eq!(observation.proxy_variables().len(), 1);
        assert_eq!(
            observation.proxy_variables()[0].name(),
            ProxyEnvironmentVariableName::AllProxy
        );
        assert_eq!(
            observation.proxy_variables()[0].sources(),
            &[ProxyEnvironmentSource::RuntimeProcess]
        );
        assert_eq!(
            observation.coverage().persistent_user(),
            ObservationCoverageStatus::NotObserved
        );
        assert_eq!(
            observation.coverage().persistent_system(),
            ObservationCoverageStatus::NotObserved
        );
        assert!(!format!("{observation:?}").contains(sentinel));
    }

    #[test]
    fn macos_用户目录不可用时明确失败() {
        let error = observe_macos_with_inputs(
            MacosObservationInputs {
                runtime_environment: Vec::new(),
                user_home: None,
                codex_home: None,
            },
            &MemoryPathProbe::default(),
            &MissingFileProbe,
        )
        .expect_err("缺失用户目录不得伪造空报告");

        assert_eq!(error.code().as_str(), "USER_HOME_UNAVAILABLE");
    }
}
