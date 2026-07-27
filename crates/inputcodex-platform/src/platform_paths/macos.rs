use std::path::{Path, PathBuf};

use inputcodex_application::ApplicationError;
#[cfg(target_os = "macos")]
use inputcodex_application::PlatformPathsRequest;
use inputcodex_domain::{ApplicationInstallSource, CodexInstallation, PrivatePath};

#[cfg(target_os = "macos")]
use super::{CommonInputs, resolve_common_roots, snapshot_from_common};
use super::{PathProbe, is_manager_path};

pub(super) const APP_NAMES: [&str; 4] = [
    "Codex.app",
    "OpenAI Codex.app",
    "OpenAI.Codex.app",
    "ChatGPT.app",
];
const EXECUTABLE_NAMES: [&str; 2] = ["Codex", "ChatGPT"];

struct MacosCandidate {
    application_root: PathBuf,
    executable: PathBuf,
    source: ApplicationInstallSource,
}

#[cfg(target_os = "macos")]
pub(super) fn resolve_system(
    request: &PlatformPathsRequest,
    probe: &impl PathProbe,
) -> Result<inputcodex_domain::PlatformPathsSnapshot, ApplicationError> {
    let user_home = std::env::var_os("HOME").map(PathBuf::from);
    let discovery_home = user_home.clone();
    let state_root = user_home
        .as_ref()
        .map(|home| home.join("Library/Application Support/inputcodex"));
    let common = resolve_common_roots(
        CommonInputs {
            user_home,
            codex_home: std::env::var_os("CODEX_HOME"),
            inputcodex_state_root: state_root,
        },
        probe,
    )?;
    let discovery_home =
        discovery_home.ok_or_else(|| ApplicationError::unavailable("USER_HOME_UNAVAILABLE"))?;
    let installation =
        resolve_installation(request.explicit_application_path(), &discovery_home, probe)?;

    snapshot_from_common(common, installation)
}

#[cfg(target_os = "macos")]
pub(crate) fn resolve_installation_system(
    explicit_application_path: Option<&Path>,
    probe: &impl PathProbe,
) -> Result<Option<CodexInstallation>, ApplicationError> {
    if let Some(path) = explicit_application_path {
        return resolve_explicit(path, probe).map(Some);
    }

    let home = std::env::var_os("HOME")
        .map(PathBuf::from)
        .ok_or_else(|| ApplicationError::unavailable("USER_HOME_UNAVAILABLE"))?;
    resolve_installation(None, &home, probe)
}

#[cfg(target_os = "macos")]
pub(crate) fn resolve_installation(
    explicit_application_path: Option<&Path>,
    home: &Path,
    probe: &impl PathProbe,
) -> Result<Option<CodexInstallation>, ApplicationError> {
    match explicit_application_path {
        Some(path) => resolve_explicit(path, probe).map(Some),
        None => Ok(discover(home, probe)),
    }
}

pub(super) fn resolve_explicit(
    path: &Path,
    probe: &impl PathProbe,
) -> Result<CodexInstallation, ApplicationError> {
    if !path.is_absolute() || is_manager_path(path) {
        return Err(explicit_path_error());
    }

    if is_supported_bundle(path) {
        return installation_from_bundle(
            path.to_path_buf(),
            ApplicationInstallSource::Explicit,
            probe,
        )
        .ok_or_else(explicit_path_error);
    }

    if !probe.is_file(path) || !is_supported_executable(path) {
        return Err(explicit_path_error());
    }
    let macos_directory = path.parent().ok_or_else(explicit_path_error)?;
    if macos_directory.file_name().and_then(|name| name.to_str()) != Some("MacOS") {
        return Err(explicit_path_error());
    }
    let contents_directory = macos_directory.parent().ok_or_else(explicit_path_error)?;
    if contents_directory
        .file_name()
        .and_then(|name| name.to_str())
        != Some("Contents")
    {
        return Err(explicit_path_error());
    }
    let bundle = contents_directory
        .parent()
        .ok_or_else(explicit_path_error)?;
    if !is_supported_bundle(bundle) || !probe.is_dir(bundle) {
        return Err(explicit_path_error());
    }

    installation(
        bundle.to_path_buf(),
        path.to_path_buf(),
        ApplicationInstallSource::Explicit,
    )
    .ok_or_else(explicit_path_error)
}

#[cfg(target_os = "macos")]
pub(super) fn discover(home: &Path, probe: &impl PathProbe) -> Option<CodexInstallation> {
    let candidate = discover_candidate(home, probe)?;
    installation(
        candidate.application_root,
        candidate.executable,
        candidate.source,
    )
}

fn discover_candidate(home: &Path, probe: &impl PathProbe) -> Option<MacosCandidate> {
    let roots = [
        (
            PathBuf::from("/Applications"),
            ApplicationInstallSource::MacosSystemApplications,
        ),
        (
            home.join("Applications"),
            ApplicationInstallSource::MacosUserApplications,
        ),
    ];

    roots.into_iter().find_map(|(root, source)| {
        APP_NAMES
            .into_iter()
            .find_map(|name| candidate_from_bundle(root.join(name), source, probe))
    })
}

fn installation_from_bundle(
    bundle: PathBuf,
    source: ApplicationInstallSource,
    probe: &impl PathProbe,
) -> Option<CodexInstallation> {
    let candidate = candidate_from_bundle(bundle, source, probe)?;
    installation(
        candidate.application_root,
        candidate.executable,
        candidate.source,
    )
}

fn candidate_from_bundle(
    bundle: PathBuf,
    source: ApplicationInstallSource,
    probe: &impl PathProbe,
) -> Option<MacosCandidate> {
    if is_manager_path(&bundle) || !is_supported_bundle(&bundle) || !probe.is_dir(&bundle) {
        return None;
    }

    let executable_directory = bundle.join("Contents/MacOS");
    let executable = EXECUTABLE_NAMES
        .into_iter()
        .map(|name| executable_directory.join(name))
        .find(|candidate| probe.is_file(candidate))
        .unwrap_or_else(|| executable_directory.join("Codex"));

    Some(MacosCandidate {
        application_root: bundle,
        executable,
        source,
    })
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

fn is_supported_bundle(path: &Path) -> bool {
    let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
        return false;
    };
    APP_NAMES
        .into_iter()
        .any(|candidate| name.eq_ignore_ascii_case(candidate))
}

fn is_supported_executable(path: &Path) -> bool {
    let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
        return false;
    };
    EXECUTABLE_NAMES
        .into_iter()
        .any(|candidate| name.eq_ignore_ascii_case(candidate))
}

const fn explicit_path_error() -> ApplicationError {
    ApplicationError::unavailable("EXPLICIT_CODEX_PATH_INVALID")
}

#[cfg(test)]
mod tests {
    use std::{
        cell::Cell,
        collections::HashSet,
        path::{Path, PathBuf},
    };

    use inputcodex_domain::ApplicationInstallSource;

    #[cfg(target_os = "macos")]
    use super::resolve_installation_system;
    use super::{APP_NAMES, discover_candidate, resolve_explicit};
    use crate::platform_paths::PathProbe;

    #[derive(Default)]
    struct MemoryProbe {
        directories: HashSet<PathBuf>,
        files: HashSet<PathBuf>,
        directory_checks: Cell<usize>,
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
            self.directory_checks.set(self.directory_checks.get() + 1);
            self.directories.contains(path)
        }

        fn is_file(&self, path: &Path) -> bool {
            self.files.contains(path)
        }
    }

    fn home(name: &str) -> PathBuf {
        std::env::temp_dir().join(name)
    }

    #[test]
    fn 固定应用名称和顺序不漂移() {
        assert_eq!(
            APP_NAMES,
            [
                "Codex.app",
                "OpenAI Codex.app",
                "OpenAI.Codex.app",
                "ChatGPT.app",
            ]
        );
    }

    #[test]
    fn 系统应用目录优先于用户目录且每个根按名称顺序() {
        let user_home = home("inputcodex-macos-home");
        let system_chatgpt = PathBuf::from("/Applications/ChatGPT.app");
        let user_codex = user_home.join("Applications/Codex.app");
        let probe = MemoryProbe::default()
            .with_directory(system_chatgpt.clone())
            .with_file(system_chatgpt.join("Contents/MacOS/ChatGPT"))
            .with_directory(user_codex.clone())
            .with_file(user_codex.join("Contents/MacOS/Codex"));

        let candidate = discover_candidate(&user_home, &probe).expect("系统目录候选应优先");

        assert_eq!(candidate.application_root, system_chatgpt);
        assert_eq!(
            candidate.source,
            ApplicationInstallSource::MacosSystemApplications
        );

        let system_codex = PathBuf::from("/Applications/Codex.app");
        let system_openai_codex = PathBuf::from("/Applications/OpenAI Codex.app");
        let name_probe = MemoryProbe::default()
            .with_directory(system_codex.clone())
            .with_file(system_codex.join("Contents/MacOS/Codex"))
            .with_directory(system_openai_codex.clone())
            .with_file(system_openai_codex.join("Contents/MacOS/Codex"));

        let first_name = discover_candidate(&user_home, &name_probe).expect("首个固定名称应优先");
        assert_eq!(first_name.application_root, system_codex);
    }

    #[test]
    fn 显式_app_与_contents_macos_可执行文件归一为同一安装() {
        let bundle = home("inputcodex-explicit-macos").join("Codex.app");
        let executable = bundle.join("Contents/MacOS/Codex");
        let probe = MemoryProbe::default()
            .with_directory(bundle.clone())
            .with_file(executable.clone());

        let from_bundle = resolve_explicit(&bundle, &probe).expect("合法 app 应成功");
        let from_executable = resolve_explicit(&executable, &probe).expect("合法可执行文件应成功");

        assert_eq!(from_bundle.application_root().as_path(), bundle);
        assert_eq!(from_bundle.executable().as_path(), executable);
        assert_eq!(from_bundle, from_executable);
        assert_eq!(from_bundle.source(), ApplicationInstallSource::Explicit);
    }

    #[test]
    fn 显式路径拒绝管理器名称和非标准可执行结构() {
        let manager = home("inputcodex-manager-macos").join("inputcodex/Codex.app");
        let arbitrary = home("inputcodex-arbitrary-macos").join("Codex");
        let probe = MemoryProbe::default()
            .with_directory(manager.clone())
            .with_file(manager.join("Contents/MacOS/Codex"))
            .with_file(arbitrary.clone());

        for path in [&manager, &arbitrary] {
            let error = resolve_explicit(path, &probe).expect_err("无效显式路径必须失败");
            assert_eq!(error.code().as_str(), "EXPLICIT_CODEX_PATH_INVALID");
        }
    }

    #[test]
    fn 自动发现最多检查两个根四个名称() {
        let user_home = home("inputcodex-eight-candidates");
        let probe = MemoryProbe::default();

        assert!(discover_candidate(&user_home, &probe).is_none());
        assert_eq!(probe.directory_checks.get(), 8);
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn 安装专用入口可直接解析显式路径而不依赖共同状态目录() {
        let bundle = home("inputcodex-overview-explicit-macos").join("Codex.app");
        let executable = bundle.join("Contents/MacOS/Codex");
        let probe = MemoryProbe::default()
            .with_directory(bundle.clone())
            .with_file(executable);

        let installation = resolve_installation_system(Some(&bundle), &probe)
            .expect("合法显式路径应成功")
            .expect("显式路径应返回安装");

        assert_eq!(installation.application_root().as_path(), bundle);
        assert_eq!(installation.source(), ApplicationInstallSource::Explicit);
    }
}
