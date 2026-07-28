#[cfg(any(target_os = "windows", target_os = "macos", test))]
use std::{
    ffi::OsString,
    path::{Path, PathBuf},
};

use inputcodex_application::{ApplicationError, PlatformPathsPort, PlatformPathsRequest};
use inputcodex_domain::PlatformPathsSnapshot;
#[cfg(any(target_os = "windows", target_os = "macos", test))]
use inputcodex_domain::{CodexInstallation, PrivatePath};

#[cfg(any(target_os = "macos", test))]
pub(crate) mod macos;
#[cfg(any(target_os = "windows", test))]
pub(crate) mod windows;

#[cfg(any(target_os = "windows", target_os = "macos", test))]
const SETTINGS_FILE: &str = "settings.json";
#[cfg(any(target_os = "windows", target_os = "macos", test))]
const LATEST_STATUS_FILE: &str = "latest-status.json";
#[cfg(any(target_os = "windows", target_os = "macos", test))]
const DIAGNOSTIC_LOG_FILE: &str = "inputcodex.log";

#[cfg(any(target_os = "windows", target_os = "macos", test))]
pub(crate) trait PathProbe {
    fn is_dir(&self, path: &Path) -> bool;
    fn is_file(&self, path: &Path) -> bool;
}

#[derive(Debug, Clone, Copy, Default)]
#[cfg(any(target_os = "windows", target_os = "macos"))]
pub(crate) struct SystemPathProbe;

#[cfg(any(target_os = "windows", target_os = "macos"))]
impl PathProbe for SystemPathProbe {
    fn is_dir(&self, path: &Path) -> bool {
        path.is_dir()
    }

    fn is_file(&self, path: &Path) -> bool {
        path.is_file()
    }
}

#[cfg(any(target_os = "windows", target_os = "macos", test))]
struct CommonInputs {
    user_home: Option<PathBuf>,
    codex_home: Option<OsString>,
    inputcodex_state_root: Option<PathBuf>,
}

#[cfg(any(target_os = "windows", target_os = "macos", test))]
struct ResolvedCommonPaths {
    codex_home: PathBuf,
    state_root: PathBuf,
}

#[cfg(any(target_os = "windows", target_os = "macos", test))]
fn resolve_common_roots(
    inputs: CommonInputs,
    probe: &impl PathProbe,
) -> Result<ResolvedCommonPaths, ApplicationError> {
    let user_home = resolve_user_home(inputs.user_home, probe)?;
    let state_root = inputs
        .inputcodex_state_root
        .filter(|path| path.is_absolute())
        .ok_or_else(|| ApplicationError::unavailable("INPUTCODEX_STATE_ROOT_UNAVAILABLE"))?;
    let codex_home = resolve_codex_home_from_user_home(&user_home, inputs.codex_home, probe)?;

    Ok(ResolvedCommonPaths {
        codex_home,
        state_root,
    })
}

#[cfg(any(target_os = "windows", target_os = "macos", test))]
pub(crate) fn resolve_codex_home(
    user_home: Option<PathBuf>,
    codex_home: Option<OsString>,
    probe: &impl PathProbe,
) -> Result<PathBuf, ApplicationError> {
    let user_home = resolve_user_home(user_home, probe)?;
    resolve_codex_home_from_user_home(&user_home, codex_home, probe)
}

#[cfg(any(target_os = "windows", target_os = "macos", test))]
fn resolve_user_home(
    user_home: Option<PathBuf>,
    probe: &impl PathProbe,
) -> Result<PathBuf, ApplicationError> {
    user_home
        .filter(|path| path.is_absolute() && probe.is_dir(path))
        .ok_or_else(|| ApplicationError::unavailable("USER_HOME_UNAVAILABLE"))
}

#[cfg(any(target_os = "windows", target_os = "macos", test))]
fn resolve_codex_home_from_user_home(
    user_home: &Path,
    codex_home: Option<OsString>,
    probe: &impl PathProbe,
) -> Result<PathBuf, ApplicationError> {
    let codex_home = match codex_home {
        None => user_home.join(".codex"),
        Some(value) if value.to_string_lossy().trim().is_empty() => user_home.join(".codex"),
        Some(value) => {
            let path = PathBuf::from(value);
            if !path.is_absolute() || probe.is_file(&path) || !probe.is_dir(&path) {
                return Err(ApplicationError::unavailable("CODEX_HOME_INVALID"));
            }
            path
        }
    };
    Ok(codex_home)
}

#[cfg(any(target_os = "windows", target_os = "macos", test))]
fn snapshot_from_common(
    common: ResolvedCommonPaths,
    installation: Option<CodexInstallation>,
) -> Result<PlatformPathsSnapshot, ApplicationError> {
    let private = |path| {
        PrivatePath::new(path).map_err(|_| ApplicationError::internal("PLATFORM_PATHS_FAILED"))
    };

    Ok(PlatformPathsSnapshot::new(
        private(common.codex_home)?,
        private(common.state_root.clone())?,
        private(common.state_root.join(SETTINGS_FILE))?,
        private(common.state_root.join(LATEST_STATUS_FILE))?,
        private(common.state_root.join(DIAGNOSTIC_LOG_FILE))?,
        installation,
    ))
}

#[cfg(test)]
fn resolve_common_paths(
    inputs: CommonInputs,
    installation: Option<CodexInstallation>,
    probe: &impl PathProbe,
) -> Result<PlatformPathsSnapshot, ApplicationError> {
    let common = resolve_common_roots(inputs, probe)?;
    snapshot_from_common(common, installation)
}

#[cfg(any(target_os = "windows", target_os = "macos", test))]
fn is_manager_path(path: &Path) -> bool {
    path.components().any(|component| {
        let std::path::Component::Normal(name) = component else {
            return false;
        };
        let normalized = name.to_string_lossy().to_ascii_lowercase();
        normalized == "inputcodex"
            || normalized == "codex++"
            || normalized == "codexplusplus"
            || normalized == "codex-plus-plus"
            || normalized.contains("codex-plus-manager")
    })
}

#[derive(Debug, Clone, Copy, Default)]
pub struct SystemPlatformPaths;

impl PlatformPathsPort for SystemPlatformPaths {
    fn resolve(
        &self,
        request: &PlatformPathsRequest,
    ) -> Result<PlatformPathsSnapshot, ApplicationError> {
        #[cfg(target_os = "windows")]
        {
            windows::resolve_system(request, &SystemPathProbe)
        }

        #[cfg(target_os = "macos")]
        {
            macos::resolve_system(request, &SystemPathProbe)
        }

        #[cfg(not(any(target_os = "windows", target_os = "macos")))]
        {
            let _ = request;
            Err(ApplicationError::unsupported("PLATFORM_PATHS_UNSUPPORTED"))
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{
        collections::HashSet,
        ffi::OsString,
        path::{Path, PathBuf},
    };

    use super::{CommonInputs, PathProbe, resolve_codex_home, resolve_common_paths};

    #[derive(Default)]
    struct MemoryProbe {
        directories: HashSet<PathBuf>,
        files: HashSet<PathBuf>,
    }

    impl MemoryProbe {
        fn with_directory(mut self, path: PathBuf) -> Self {
            self.directories.insert(path);
            self
        }

        fn with_file(mut self, path: PathBuf) -> Self {
            self.files.insert(path);
            self
        }
    }

    impl PathProbe for MemoryProbe {
        fn is_dir(&self, path: &Path) -> bool {
            self.directories.contains(path)
        }

        fn is_file(&self, path: &Path) -> bool {
            self.files.contains(path)
        }
    }

    fn absolute_root(name: &str) -> PathBuf {
        std::env::temp_dir().join(name)
    }

    #[test]
    fn 空白_codex_home_使用用户目录并派生固定文件名() {
        let user_home = absolute_root("inputcodex-common-home");
        let state_root = absolute_root("inputcodex-common-state");
        let probe = MemoryProbe::default().with_directory(user_home.clone());

        let snapshot = resolve_common_paths(
            CommonInputs {
                user_home: Some(user_home.clone()),
                codex_home: Some(OsString::from("   ")),
                inputcodex_state_root: Some(state_root.clone()),
            },
            None,
            &probe,
        )
        .expect("空白 CODEX_HOME 应使用用户目录下 .codex");

        assert_eq!(snapshot.codex_home().as_path(), user_home.join(".codex"));
        assert_eq!(
            snapshot.settings_file().as_path(),
            state_root.join("settings.json")
        );
        assert_eq!(
            snapshot.latest_status_file().as_path(),
            state_root.join("latest-status.json")
        );
        assert_eq!(
            snapshot.diagnostic_log_file().as_path(),
            state_root.join("inputcodex.log")
        );
        assert!(snapshot.codex_installation().is_none());
    }

    #[test]
    fn 窄_codex_home_解析复用完整路径语义() {
        let user_home = absolute_root("inputcodex-narrow-home");
        let explicit = absolute_root("inputcodex-narrow-codex-home");
        let probe = MemoryProbe::default()
            .with_directory(user_home.clone())
            .with_directory(explicit.clone());

        assert_eq!(
            resolve_codex_home(
                Some(user_home.clone()),
                Some(explicit.clone().into_os_string()),
                &probe,
            )
            .expect("合法显式 CODEX_HOME 应原样返回"),
            explicit
        );
        assert_eq!(
            resolve_codex_home(Some(user_home.clone()), Some(OsString::from("   ")), &probe)
                .expect("空白 CODEX_HOME 应回退用户目录"),
            user_home.join(".codex")
        );

        let error = resolve_codex_home(
            Some(user_home),
            Some(OsString::from("relative/.codex")),
            &probe,
        )
        .expect_err("无效显式 CODEX_HOME 不得静默回退");
        assert_eq!(error.code().as_str(), "CODEX_HOME_INVALID");
    }

    #[test]
    fn 非空无效_codex_home_明确失败且不回退() {
        let user_home = absolute_root("inputcodex-invalid-home");
        let state_root = absolute_root("inputcodex-invalid-state");
        let missing = absolute_root("inputcodex-missing-codex-home");
        let file = absolute_root("inputcodex-file-codex-home");
        let probe = MemoryProbe::default()
            .with_directory(user_home.clone())
            .with_file(file.clone());

        for value in [
            OsString::from("relative/.codex"),
            missing.into_os_string(),
            file.into_os_string(),
        ] {
            let error = resolve_common_paths(
                CommonInputs {
                    user_home: Some(user_home.clone()),
                    codex_home: Some(value),
                    inputcodex_state_root: Some(state_root.clone()),
                },
                None,
                &probe,
            )
            .expect_err("无效非空 CODEX_HOME 必须失败");

            assert_eq!(error.code().as_str(), "CODEX_HOME_INVALID");
        }
    }

    #[test]
    fn 缺失用户目录或状态根返回稳定错误() {
        let user_home = absolute_root("inputcodex-required-home");
        let state_root = absolute_root("inputcodex-required-state");
        let probe = MemoryProbe::default().with_directory(user_home.clone());

        let missing_home = resolve_common_paths(
            CommonInputs {
                user_home: None,
                codex_home: None,
                inputcodex_state_root: Some(state_root),
            },
            None,
            &probe,
        )
        .expect_err("用户目录缺失必须失败");
        assert_eq!(missing_home.code().as_str(), "USER_HOME_UNAVAILABLE");

        let missing_state = resolve_common_paths(
            CommonInputs {
                user_home: Some(user_home),
                codex_home: None,
                inputcodex_state_root: None,
            },
            None,
            &probe,
        )
        .expect_err("状态根缺失必须失败");
        assert_eq!(
            missing_state.code().as_str(),
            "INPUTCODEX_STATE_ROOT_UNAVAILABLE"
        );
    }
}
