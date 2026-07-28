use std::{ffi::OsString, path::PathBuf};

use inputcodex_application::ApplicationError;
use inputcodex_domain::RelayEnvironmentObservation;
#[cfg(target_os = "windows")]
use windows_registry::{CURRENT_USER, LOCAL_MACHINE};

#[cfg(target_os = "windows")]
use crate::platform_paths::SystemPathProbe;
use crate::platform_paths::{PathProbe, resolve_codex_home};

#[cfg(target_os = "windows")]
use super::SystemRelayFileProbe;
use super::{
    PersistentEnvironment, RelayFileProbe, RelayObservationInputs, clash_candidate_paths,
    observe_with_inputs,
};

#[cfg(target_os = "windows")]
const CURRENT_USER_ENVIRONMENT: &str = "Environment";
#[cfg(target_os = "windows")]
const LOCAL_MACHINE_ENVIRONMENT: &str =
    r"SYSTEM\CurrentControlSet\Control\Session Manager\Environment";

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum RegistryHive {
    CurrentUser,
    LocalMachine,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RegistryReadError {
    Unavailable,
}

trait WindowsRegistryProbe {
    fn read_environment(
        &self,
        hive: RegistryHive,
    ) -> Result<Vec<(OsString, OsString)>, RegistryReadError>;
}

#[derive(Debug, Clone, Copy, Default)]
#[cfg(target_os = "windows")]
struct SystemWindowsRegistryProbe;

#[cfg(target_os = "windows")]
impl WindowsRegistryProbe for SystemWindowsRegistryProbe {
    fn read_environment(
        &self,
        hive: RegistryHive,
    ) -> Result<Vec<(OsString, OsString)>, RegistryReadError> {
        let (root, path) = match hive {
            RegistryHive::CurrentUser => (CURRENT_USER, CURRENT_USER_ENVIRONMENT),
            RegistryHive::LocalMachine => (LOCAL_MACHINE, LOCAL_MACHINE_ENVIRONMENT),
        };
        let key = root
            .open(path)
            .map_err(|_| RegistryReadError::Unavailable)?;
        let values = key.values().map_err(|_| RegistryReadError::Unavailable)?;
        let mut environment = Vec::new();
        for (name, value) in values {
            if inputcodex_domain::ProxyEnvironmentVariableName::from_name(&name).is_none() {
                continue;
            }
            let value = String::try_from(value).map_err(|_| RegistryReadError::Unavailable)?;
            environment.push((OsString::from(name), OsString::from(value)));
        }
        Ok(environment)
    }
}

struct WindowsObservationInputs {
    runtime_environment: Vec<(OsString, OsString)>,
    user_home: Option<PathBuf>,
    codex_home: Option<OsString>,
    app_data: Option<PathBuf>,
}

#[cfg(target_os = "windows")]
pub(super) fn observe_system() -> Result<RelayEnvironmentObservation, ApplicationError> {
    observe_windows_with_inputs(
        WindowsObservationInputs {
            runtime_environment: std::env::vars_os().collect(),
            user_home: std::env::var_os("USERPROFILE").map(PathBuf::from),
            codex_home: std::env::var_os("CODEX_HOME"),
            app_data: std::env::var_os("APPDATA").map(PathBuf::from),
        },
        &SystemWindowsRegistryProbe,
        &SystemPathProbe,
        &SystemRelayFileProbe,
    )
}

fn observe_windows_with_inputs(
    inputs: WindowsObservationInputs,
    registry: &impl WindowsRegistryProbe,
    path_probe: &impl PathProbe,
    file_probe: &impl RelayFileProbe,
) -> Result<RelayEnvironmentObservation, ApplicationError> {
    let user_home = inputs.user_home.clone();
    let codex_home = resolve_codex_home(inputs.user_home, inputs.codex_home, path_probe)?;
    let user_home = user_home
        .filter(|path| path.is_absolute())
        .ok_or_else(|| ApplicationError::unavailable("USER_HOME_UNAVAILABLE"))?;
    let app_data = inputs
        .app_data
        .filter(|path| path.is_absolute())
        .ok_or_else(|| ApplicationError::unavailable("RELAY_ENVIRONMENT_OBSERVATION_FAILED"))?;

    observe_with_inputs(
        RelayObservationInputs {
            runtime_environment: inputs.runtime_environment,
            persistent_user: persistent_environment(
                registry.read_environment(RegistryHive::CurrentUser),
            ),
            persistent_system: persistent_environment(
                registry.read_environment(RegistryHive::LocalMachine),
            ),
            codex_home,
            clash_candidates: clash_candidate_paths(&app_data, &app_data, &user_home),
        },
        file_probe,
    )
}

fn persistent_environment(
    result: Result<Vec<(OsString, OsString)>, RegistryReadError>,
) -> PersistentEnvironment {
    match result {
        Ok(values) => PersistentEnvironment::Observed(values),
        Err(RegistryReadError::Unavailable) => PersistentEnvironment::Unavailable,
    }
}

#[cfg(test)]
mod tests {
    use std::{
        collections::{BTreeMap, HashSet},
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

    impl MemoryPathProbe {
        fn with_directory(mut self, path: PathBuf) -> Self {
            self.directories.insert(path);
            self
        }
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

    #[derive(Default)]
    struct MemoryRegistryProbe {
        values: BTreeMap<RegistryHive, Result<Vec<(OsString, OsString)>, RegistryReadError>>,
    }

    impl MemoryRegistryProbe {
        fn with_result(
            mut self,
            hive: RegistryHive,
            result: Result<Vec<(OsString, OsString)>, RegistryReadError>,
        ) -> Self {
            self.values.insert(hive, result);
            self
        }
    }

    impl WindowsRegistryProbe for MemoryRegistryProbe {
        fn read_environment(
            &self,
            hive: RegistryHive,
        ) -> Result<Vec<(OsString, OsString)>, RegistryReadError> {
            self.values.get(&hive).cloned().unwrap_or(Ok(Vec::new()))
        }
    }

    fn roots(name: &str) -> (PathBuf, PathBuf, MemoryPathProbe) {
        let home = std::env::temp_dir().join(format!("{name}-home"));
        let app_data = std::env::temp_dir().join(format!("{name}-appdata"));
        let probe = MemoryPathProbe::default().with_directory(home.clone());
        (home, app_data, probe)
    }

    #[test]
    fn windows_合并进程用户与系统代理来源() {
        let sentinel = "http://windows-private.invalid";
        let (home, app_data, path_probe) = roots("inputcodex-windows-relay");
        let registry = MemoryRegistryProbe::default()
            .with_result(
                RegistryHive::CurrentUser,
                Ok(vec![(
                    OsString::from("HTTPS_PROXY"),
                    OsString::from("user-secret"),
                )]),
            )
            .with_result(
                RegistryHive::LocalMachine,
                Ok(vec![(
                    OsString::from("HTTP_PROXY"),
                    OsString::from("system-secret"),
                )]),
            );

        let observation = observe_windows_with_inputs(
            WindowsObservationInputs {
                runtime_environment: vec![(OsString::from("http_proxy"), OsString::from(sentinel))],
                user_home: Some(home),
                codex_home: None,
                app_data: Some(app_data),
            },
            &registry,
            &path_probe,
            &MissingFileProbe,
        )
        .expect("Windows 观察应成功");

        assert_eq!(observation.proxy_variables().len(), 2);
        assert_eq!(
            observation.proxy_variables()[0].name(),
            ProxyEnvironmentVariableName::HttpProxy
        );
        assert_eq!(
            observation.proxy_variables()[0].sources(),
            &[
                ProxyEnvironmentSource::RuntimeProcess,
                ProxyEnvironmentSource::PersistentSystem,
            ]
        );
        assert_eq!(
            observation.proxy_variables()[1].sources(),
            &[ProxyEnvironmentSource::PersistentUser]
        );
        assert!(!format!("{observation:?}").contains(sentinel));
        assert!(!format!("{observation:?}").contains("system-secret"));
    }

    #[test]
    fn windows_空注册表键仍标记为_observed() {
        let (home, app_data, path_probe) = roots("inputcodex-windows-empty-registry");
        let observation = observe_windows_with_inputs(
            WindowsObservationInputs {
                runtime_environment: Vec::new(),
                user_home: Some(home),
                codex_home: None,
                app_data: Some(app_data),
            },
            &MemoryRegistryProbe::default(),
            &path_probe,
            &MissingFileProbe,
        )
        .expect("空注册表键也应形成可信观察");

        assert!(observation.proxy_variables().is_empty());
        assert_eq!(
            observation.coverage().persistent_user(),
            ObservationCoverageStatus::Observed
        );
        assert_eq!(
            observation.coverage().persistent_system(),
            ObservationCoverageStatus::Observed
        );
    }

    #[test]
    fn windows_单个注册表来源失败保留其他事实() {
        let (home, app_data, path_probe) = roots("inputcodex-windows-registry-gap");
        let registry = MemoryRegistryProbe::default()
            .with_result(
                RegistryHive::CurrentUser,
                Err(RegistryReadError::Unavailable),
            )
            .with_result(
                RegistryHive::LocalMachine,
                Ok(vec![(
                    OsString::from("FTP_PROXY"),
                    OsString::from("system-value"),
                )]),
            );

        let observation = observe_windows_with_inputs(
            WindowsObservationInputs {
                runtime_environment: Vec::new(),
                user_home: Some(home),
                codex_home: None,
                app_data: Some(app_data),
            },
            &registry,
            &path_probe,
            &MissingFileProbe,
        )
        .expect("局部注册表失败不得丢弃系统事实");

        assert_eq!(
            observation.coverage().persistent_user(),
            ObservationCoverageStatus::Unavailable
        );
        assert_eq!(
            observation.coverage().persistent_system(),
            ObservationCoverageStatus::Observed
        );
        assert_eq!(
            observation.proxy_variables()[0].name(),
            ProxyEnvironmentVariableName::FtpProxy
        );
    }
}
