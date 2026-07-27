use std::path::{Path, PathBuf};

use inputcodex_application::ApplicationError;
#[cfg(target_os = "windows")]
use inputcodex_application::PlatformPathsRequest;
use inputcodex_domain::{ApplicationInstallSource, CodexInstallation, PrivatePath};

#[cfg(target_os = "windows")]
use super::{CommonInputs, resolve_common_roots, snapshot_from_common};
use super::{PathProbe, is_manager_path};

pub(super) const PACKAGE_FAMILY_NAMES: [&str; 3] = [
    "OpenAI.Codex_2p2nqsd0c76g0",
    "OpenAI.CodexBeta_2p2nqsd0c76g0",
    "OpenAI.ChatGPT-Desktop_2p2nqsd0c76g0",
];
const EXECUTABLE_NAMES: [&str; 2] = ["Codex.exe", "ChatGPT.exe"];

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct WindowsPackageCandidate {
    identity_order: usize,
    version: [u16; 4],
    root: PathBuf,
}

#[cfg(target_os = "windows")]
pub(super) fn resolve_system(
    request: &PlatformPathsRequest,
    probe: &impl PathProbe,
) -> Result<inputcodex_domain::PlatformPathsSnapshot, ApplicationError> {
    let user_home = std::env::var_os("USERPROFILE").map(PathBuf::from);
    let local_appdata = std::env::var_os("LOCALAPPDATA").map(PathBuf::from);
    let state_root = local_appdata.as_ref().map(|root| root.join("inputcodex"));
    let common = resolve_common_roots(
        CommonInputs {
            user_home,
            codex_home: std::env::var_os("CODEX_HOME"),
            inputcodex_state_root: state_root,
        },
        probe,
    )?;
    let local_appdata = local_appdata
        .ok_or_else(|| ApplicationError::unavailable("INPUTCODEX_STATE_ROOT_UNAVAILABLE"))?;
    let installation = resolve_installation(request, &local_appdata, probe)?;

    snapshot_from_common(common, installation)
}

#[cfg(target_os = "windows")]
fn resolve_installation(
    request: &PlatformPathsRequest,
    local_appdata: &Path,
    probe: &impl PathProbe,
) -> Result<Option<CodexInstallation>, ApplicationError> {
    if let Some(path) = request.explicit_application_path() {
        return resolve_explicit(path, probe).map(Some);
    }

    let packages = registered_package_candidates()?;
    Ok(select_package_candidate(packages, probe)
        .or_else(|| discover_standalone(local_appdata, probe)))
}

pub(super) fn resolve_explicit(
    path: &Path,
    probe: &impl PathProbe,
) -> Result<CodexInstallation, ApplicationError> {
    if !path.is_absolute() || is_manager_path(path) {
        return Err(explicit_path_error());
    }

    if probe.is_file(path) {
        return installation_from_executable(
            path.to_path_buf(),
            ApplicationInstallSource::Explicit,
            probe,
        )
        .ok_or_else(explicit_path_error);
    }
    if probe.is_dir(path) {
        return installation_from_dir(
            path.to_path_buf(),
            ApplicationInstallSource::Explicit,
            probe,
        )
        .ok_or_else(explicit_path_error);
    }

    Err(explicit_path_error())
}

pub(super) fn select_package_candidate(
    mut candidates: Vec<WindowsPackageCandidate>,
    probe: &impl PathProbe,
) -> Option<CodexInstallation> {
    candidates.sort_by(|left, right| {
        right
            .version
            .cmp(&left.version)
            .then_with(|| left.identity_order.cmp(&right.identity_order))
            .then_with(|| left.root.cmp(&right.root))
    });

    candidates.into_iter().find_map(|candidate| {
        installation_from_dir(
            candidate.root,
            ApplicationInstallSource::WindowsPackage,
            probe,
        )
    })
}

pub(super) fn discover_standalone(
    local_appdata: &Path,
    probe: &impl PathProbe,
) -> Option<CodexInstallation> {
    let roots = [
        local_appdata.join("OpenAI/Codex/bin"),
        local_appdata.join("OpenAI/Codex"),
        local_appdata.join("Programs/OpenAI/Codex"),
    ];

    roots.into_iter().find_map(|root| {
        installation_from_dir(root, ApplicationInstallSource::WindowsStandalone, probe)
    })
}

pub(super) fn installation_from_dir(
    directory: PathBuf,
    source: ApplicationInstallSource,
    probe: &impl PathProbe,
) -> Option<CodexInstallation> {
    if !directory.is_absolute() || is_manager_path(&directory) {
        return None;
    }

    let nested = directory.join("app");
    [nested, directory]
        .into_iter()
        .find_map(|application_root| {
            if !probe.is_dir(&application_root) || is_manager_path(&application_root) {
                return None;
            }
            EXECUTABLE_NAMES
                .into_iter()
                .map(|name| application_root.join(name))
                .find(|candidate| probe.is_file(candidate))
                .and_then(|executable| installation(application_root, executable, source))
        })
}

fn installation_from_executable(
    executable: PathBuf,
    source: ApplicationInstallSource,
    probe: &impl PathProbe,
) -> Option<CodexInstallation> {
    if !is_supported_executable(&executable) || is_manager_path(&executable) {
        return None;
    }
    let application_root = executable.parent()?.to_path_buf();
    if !probe.is_dir(&application_root) {
        return None;
    }
    installation(application_root, executable, source)
}

fn installation(
    application_root: PathBuf,
    executable: PathBuf,
    source: ApplicationInstallSource,
) -> Option<CodexInstallation> {
    Some(CodexInstallation::new(
        PrivatePath::new(application_root).ok()?,
        PrivatePath::new(executable).ok()?,
        source,
    ))
}

fn is_supported_executable(path: &Path) -> bool {
    let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
        return false;
    };
    EXECUTABLE_NAMES
        .into_iter()
        .any(|candidate| name.eq_ignore_ascii_case(candidate))
}

#[cfg(target_os = "windows")]
fn registered_package_candidates() -> Result<Vec<WindowsPackageCandidate>, ApplicationError> {
    use windows::{Management::Deployment::PackageManager, core::HSTRING};

    let manager = PackageManager::new().map_err(|_| platform_paths_error())?;
    let mut candidates = Vec::new();
    for (identity_order, family_name) in PACKAGE_FAMILY_NAMES.iter().enumerate() {
        let packages = manager
            .FindPackagesByPackageFamilyName(&HSTRING::from(*family_name))
            .map_err(|_| platform_paths_error())?;
        let iterator = packages.First().map_err(|_| platform_paths_error())?;
        let mut has_current = iterator.HasCurrent().map_err(|_| platform_paths_error())?;
        while has_current {
            let package = iterator.Current().map_err(|_| platform_paths_error())?;
            let id = package.Id().map_err(|_| platform_paths_error())?;
            let version = id.Version().map_err(|_| platform_paths_error())?;
            let root = package
                .InstalledLocation()
                .and_then(|folder| folder.Path())
                .map(|path| PathBuf::from(path.to_string()))
                .map_err(|_| platform_paths_error())?;
            candidates.push(WindowsPackageCandidate {
                identity_order,
                version: [
                    version.Major,
                    version.Minor,
                    version.Build,
                    version.Revision,
                ],
                root,
            });
            has_current = iterator.MoveNext().map_err(|_| platform_paths_error())?;
        }
    }

    Ok(candidates)
}

const fn explicit_path_error() -> ApplicationError {
    ApplicationError::unavailable("EXPLICIT_CODEX_PATH_INVALID")
}

const fn platform_paths_error() -> ApplicationError {
    ApplicationError::internal("PLATFORM_PATHS_FAILED")
}

#[cfg(test)]
mod tests {
    use std::{
        collections::HashSet,
        path::{Path, PathBuf},
    };

    use inputcodex_domain::ApplicationInstallSource;

    use super::{
        PACKAGE_FAMILY_NAMES, WindowsPackageCandidate, discover_standalone, installation_from_dir,
        resolve_explicit, select_package_candidate,
    };
    use crate::platform_paths::PathProbe;

    #[derive(Default)]
    struct MemoryProbe {
        directories: HashSet<String>,
        files: HashSet<String>,
    }

    impl MemoryProbe {
        fn normalize(path: &Path) -> String {
            path.to_string_lossy()
                .replace('/', "\\")
                .to_ascii_lowercase()
        }

        fn with_directory(mut self, path: PathBuf) -> Self {
            self.directories.insert(Self::normalize(&path));
            self
        }

        fn with_file(mut self, path: PathBuf) -> Self {
            self.files.insert(Self::normalize(&path));
            self
        }
    }

    impl PathProbe for MemoryProbe {
        fn is_dir(&self, path: &Path) -> bool {
            self.directories.contains(&Self::normalize(path))
        }

        fn is_file(&self, path: &Path) -> bool {
            self.files.contains(&Self::normalize(path))
        }
    }

    fn root(name: &str) -> PathBuf {
        std::env::temp_dir().join(name)
    }

    #[test]
    fn 包家族固定且查询顺序稳定() {
        assert_eq!(
            PACKAGE_FAMILY_NAMES,
            [
                "OpenAI.Codex_2p2nqsd0c76g0",
                "OpenAI.CodexBeta_2p2nqsd0c76g0",
                "OpenAI.ChatGPT-Desktop_2p2nqsd0c76g0",
            ]
        );
    }

    #[test]
    fn 注册包按数值版本降序且同版身份顺序稳定() {
        let older = root("inputcodex-package-1-9");
        let beta = root("inputcodex-package-beta-1-10");
        let stable = root("inputcodex-package-stable-1-10");
        let probe = MemoryProbe::default()
            .with_directory(older.clone())
            .with_file(older.join("Codex.exe"))
            .with_directory(beta.join("app"))
            .with_file(beta.join("app/ChatGPT.exe"))
            .with_directory(stable.join("app"))
            .with_file(stable.join("app/Codex.exe"));
        let candidates = vec![
            WindowsPackageCandidate {
                identity_order: 0,
                version: [1, 9, 99, 0],
                root: older,
            },
            WindowsPackageCandidate {
                identity_order: 1,
                version: [1, 10, 0, 0],
                root: beta,
            },
            WindowsPackageCandidate {
                identity_order: 0,
                version: [1, 10, 0, 0],
                root: stable.clone(),
            },
        ];

        let installation =
            select_package_candidate(candidates, &probe).expect("同版本时稳定身份应优先");

        assert_eq!(
            installation.application_root().as_path(),
            stable.join("app")
        );
        assert_eq!(
            installation.source(),
            ApplicationInstallSource::WindowsPackage
        );
    }

    #[test]
    fn 包根和_app_子目录只接受安全可执行文件() {
        let package_root = root("inputcodex-package-root");
        let nested_root = root("inputcodex-package-nested");
        let probe = MemoryProbe::default()
            .with_directory(package_root.clone())
            .with_file(package_root.join("ChatGPT.exe"))
            .with_directory(nested_root.join("app"))
            .with_file(nested_root.join("app/Codex.exe"));

        let direct = installation_from_dir(
            package_root.clone(),
            ApplicationInstallSource::WindowsPackage,
            &probe,
        )
        .expect("包根安全可执行文件应被接受");
        let nested = installation_from_dir(
            nested_root.clone(),
            ApplicationInstallSource::WindowsPackage,
            &probe,
        )
        .expect("app 子目录安全可执行文件应被接受");

        assert_eq!(direct.application_root().as_path(), package_root);
        assert_eq!(nested.application_root().as_path(), nested_root.join("app"));
    }

    #[test]
    fn standalone_只检查三个固定根且接受大小写等价文件名() {
        let local_appdata = root("inputcodex-local-appdata");
        let first = local_appdata.join("OpenAI/Codex/bin");
        let second = local_appdata.join("OpenAI/Codex");
        let third = local_appdata.join("Programs/OpenAI/Codex");
        let probe = MemoryProbe::default()
            .with_directory(first.clone())
            .with_file(first.join("codex.exe"))
            .with_directory(second)
            .with_directory(third);

        let installation = discover_standalone(&local_appdata, &probe)
            .expect("第一个固定根中的大小写等价可执行文件应被识别");

        assert_eq!(installation.application_root().as_path(), first);
        assert_eq!(
            installation.source(),
            ApplicationInstallSource::WindowsStandalone
        );
    }

    #[test]
    fn 显式路径拒绝管理器和无效路径且不自动回退() {
        let manager = root("inputcodex-manager").join("CodexPlusPlus/Codex.exe");
        let invalid = root("inputcodex-invalid-explicit");
        let valid = root("inputcodex-valid-explicit");
        let probe = MemoryProbe::default()
            .with_file(manager.clone())
            .with_directory(valid.clone())
            .with_file(valid.join("ChatGPT.exe"));

        for path in [&manager, &invalid] {
            let error = resolve_explicit(path, &probe).expect_err("无效显式路径必须失败");
            assert_eq!(error.code().as_str(), "EXPLICIT_CODEX_PATH_INVALID");
        }

        let installation = resolve_explicit(&valid, &probe).expect("合法显式目录应成功");
        assert_eq!(installation.application_root().as_path(), valid);
        assert_eq!(installation.source(), ApplicationInstallSource::Explicit);
    }
}
