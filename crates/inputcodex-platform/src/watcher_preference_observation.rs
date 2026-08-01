#[cfg(any(target_os = "windows", target_os = "macos"))]
use std::fs;
#[cfg(any(target_os = "windows", target_os = "macos", test))]
use std::{io, path::Path};

use inputcodex_application::{
    ApplicationError, WatcherPreferenceObservationPort, WatcherPreferenceObservationRequest,
};
#[cfg(any(target_os = "windows", target_os = "macos"))]
use inputcodex_application::{PlatformPathsPort, PlatformPathsRequest};
use inputcodex_domain::{WatcherPreference, WatcherPreferenceObservation};

#[cfg(any(target_os = "windows", target_os = "macos"))]
use crate::SystemPlatformPaths;

#[cfg(any(target_os = "windows", target_os = "macos", test))]
const WATCHER_DISABLED_MARKER: &str = "watcher.disabled";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg(any(target_os = "windows", target_os = "macos", test))]
enum WatcherPreferenceMarkerKind {
    File,
    Directory,
    Symlink,
    Other,
}

#[cfg(any(target_os = "windows", target_os = "macos", test))]
trait WatcherPreferenceMetadataProbe {
    fn symlink_metadata(&self, path: &Path) -> io::Result<WatcherPreferenceMarkerKind>;
}

#[derive(Debug, Clone, Copy, Default)]
#[cfg(any(target_os = "windows", target_os = "macos"))]
struct SystemWatcherPreferenceMetadataProbe;

#[cfg(any(target_os = "windows", target_os = "macos"))]
impl WatcherPreferenceMetadataProbe for SystemWatcherPreferenceMetadataProbe {
    fn symlink_metadata(&self, path: &Path) -> io::Result<WatcherPreferenceMarkerKind> {
        let metadata = fs::symlink_metadata(path)?;
        let file_type = metadata.file_type();
        Ok(if file_type.is_symlink() {
            WatcherPreferenceMarkerKind::Symlink
        } else if metadata.is_file() {
            WatcherPreferenceMarkerKind::File
        } else if metadata.is_dir() {
            WatcherPreferenceMarkerKind::Directory
        } else {
            WatcherPreferenceMarkerKind::Other
        })
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct SystemWatcherPreferenceObservation;

impl WatcherPreferenceObservationPort for SystemWatcherPreferenceObservation {
    fn observe(
        &self,
        request: &WatcherPreferenceObservationRequest,
    ) -> Result<WatcherPreferenceObservation, ApplicationError> {
        #[cfg(any(target_os = "windows", target_os = "macos"))]
        {
            let _ = request;
            let paths = SystemPlatformPaths.resolve(&PlatformPathsRequest::default())?;
            observe_watcher_preference_at_state_root(
                paths.inputcodex_state_root().as_path(),
                &SystemWatcherPreferenceMetadataProbe,
            )
        }

        #[cfg(not(any(target_os = "windows", target_os = "macos")))]
        {
            let _ = request;
            Err(ApplicationError::unsupported(
                "WATCHER_PREFERENCE_UNSUPPORTED",
            ))
        }
    }
}

#[cfg(any(target_os = "windows", target_os = "macos", test))]
fn observe_watcher_preference_at_state_root(
    state_root: &Path,
    probe: &impl WatcherPreferenceMetadataProbe,
) -> Result<WatcherPreferenceObservation, ApplicationError> {
    let marker = state_root.join(WATCHER_DISABLED_MARKER);
    let preference = match probe.symlink_metadata(&marker) {
        Ok(WatcherPreferenceMarkerKind::File) => WatcherPreference::ExplicitlyDisabled,
        Err(error) if error.kind() == io::ErrorKind::NotFound => {
            WatcherPreference::EnabledByDefault
        }
        Ok(
            WatcherPreferenceMarkerKind::Directory
            | WatcherPreferenceMarkerKind::Symlink
            | WatcherPreferenceMarkerKind::Other,
        ) => {
            return Err(ApplicationError::invalid_input(
                "WATCHER_PREFERENCE_INVALID",
            ));
        }
        Err(_) => {
            return Err(ApplicationError::unavailable(
                "WATCHER_PREFERENCE_UNREADABLE",
            ));
        }
    };

    Ok(WatcherPreferenceObservation::new(preference))
}

#[cfg(test)]
mod tests {
    use std::{
        cell::RefCell,
        io,
        path::{Path, PathBuf},
    };

    use inputcodex_application::ErrorKind;
    use inputcodex_domain::WatcherPreference;

    use super::{
        WatcherPreferenceMarkerKind, WatcherPreferenceMetadataProbe,
        observe_watcher_preference_at_state_root,
    };

    #[derive(Debug, Clone, Copy)]
    enum MetadataResult {
        Kind(WatcherPreferenceMarkerKind),
        Error(io::ErrorKind),
    }

    struct MemoryProbe {
        result: MetadataResult,
        calls: RefCell<Vec<PathBuf>>,
    }

    impl MemoryProbe {
        fn new(result: MetadataResult) -> Self {
            Self {
                result,
                calls: RefCell::new(Vec::new()),
            }
        }
    }

    impl WatcherPreferenceMetadataProbe for MemoryProbe {
        fn symlink_metadata(&self, path: &Path) -> io::Result<WatcherPreferenceMarkerKind> {
            self.calls.borrow_mut().push(path.to_path_buf());
            match self.result {
                MetadataResult::Kind(kind) => Ok(kind),
                MetadataResult::Error(kind) => Err(io::Error::from(kind)),
            }
        }
    }

    fn state_root() -> PathBuf {
        std::env::temp_dir().join("inputcodex-private-user-fragment")
    }

    fn assert_single_fixed_probe(probe: &MemoryProbe, root: &Path) {
        assert_eq!(
            probe.calls.borrow().as_slice(),
            &[root.join("watcher.disabled")]
        );
    }

    #[test]
    fn 标记缺失映射为默认启用且只探测固定文件一次() {
        let root = state_root();
        let probe = MemoryProbe::new(MetadataResult::Error(io::ErrorKind::NotFound));

        let observation = observe_watcher_preference_at_state_root(&root, &probe)
            .expect("标记缺失应形成默认偏好");

        assert_eq!(
            observation.preference(),
            WatcherPreference::EnabledByDefault
        );
        assert_single_fixed_probe(&probe, &root);
    }

    #[test]
    fn 普通文件映射为显式禁用且只探测固定文件一次() {
        let root = state_root();
        let probe = MemoryProbe::new(MetadataResult::Kind(WatcherPreferenceMarkerKind::File));

        let observation = observe_watcher_preference_at_state_root(&root, &probe)
            .expect("普通标记文件应形成显式禁用偏好");

        assert_eq!(
            observation.preference(),
            WatcherPreference::ExplicitlyDisabled
        );
        assert_single_fixed_probe(&probe, &root);
    }

    #[test]
    fn 链接目录和其他类型均_fail_closed_为_invalid() {
        let root = state_root();
        for kind in [
            WatcherPreferenceMarkerKind::Symlink,
            WatcherPreferenceMarkerKind::Directory,
            WatcherPreferenceMarkerKind::Other,
        ] {
            let probe = MemoryProbe::new(MetadataResult::Kind(kind));

            let error = observe_watcher_preference_at_state_root(&root, &probe)
                .expect_err("非普通文件必须失败");

            assert_eq!(error.kind(), ErrorKind::InvalidInput);
            assert_eq!(error.code().as_str(), "WATCHER_PREFERENCE_INVALID");
            assert!(!format!("{error:?}").contains("private-user-fragment"));
            assert_single_fixed_probe(&probe, &root);
        }
    }

    #[test]
    fn 元数据读取失败保持_unreadable_且错误不泄漏路径() {
        let root = state_root();
        let probe = MemoryProbe::new(MetadataResult::Error(io::ErrorKind::PermissionDenied));

        let error = observe_watcher_preference_at_state_root(&root, &probe)
            .expect_err("元数据读取失败必须显式失败");

        assert_eq!(error.kind(), ErrorKind::Unavailable);
        assert_eq!(error.code().as_str(), "WATCHER_PREFERENCE_UNREADABLE");
        let debug = format!("{error:?}");
        assert!(!debug.contains("private-user-fragment"));
        assert!(!debug.contains("watcher.disabled"));
        assert_single_fixed_probe(&probe, &root);
    }
}
