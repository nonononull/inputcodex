#[cfg(any(target_os = "windows", target_os = "macos"))]
use std::{
    fs::{self, File},
    io::Read,
};
#[cfg(any(target_os = "windows", target_os = "macos", test))]
use std::{io, path::Path};

use inputcodex_application::{
    ApplicationError, SettingsObservationPort, SettingsObservationRequest,
};
#[cfg(any(target_os = "windows", target_os = "macos"))]
use inputcodex_application::{PlatformPathsPort, PlatformPathsRequest};
use inputcodex_domain::SettingsDocumentObservation;
#[cfg(any(target_os = "windows", target_os = "macos", test))]
use serde_json::Value;

#[cfg(any(target_os = "windows", target_os = "macos"))]
use crate::SystemPlatformPaths;

#[cfg(any(target_os = "windows", target_os = "macos", test))]
const SETTINGS_FILE_LIMIT: usize = 256 * 1024;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg(any(target_os = "windows", target_os = "macos", test))]
enum SettingsFileKind {
    File,
    Symlink,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg(any(target_os = "windows", target_os = "macos", test))]
struct SettingsFileMetadata {
    kind: SettingsFileKind,
    length: u64,
}

#[cfg(any(target_os = "windows", target_os = "macos", test))]
enum LimitedSettingsRead {
    Bytes(Vec<u8>),
    TooLarge,
}

#[cfg(any(target_os = "windows", target_os = "macos", test))]
trait SettingsFileProbe {
    fn metadata(&self, path: &Path) -> io::Result<SettingsFileMetadata>;
    fn read_limited(&self, path: &Path, limit: usize) -> io::Result<LimitedSettingsRead>;
}

#[derive(Debug, Clone, Copy, Default)]
#[cfg(any(target_os = "windows", target_os = "macos"))]
struct SystemSettingsFileProbe;

#[cfg(any(target_os = "windows", target_os = "macos"))]
impl SettingsFileProbe for SystemSettingsFileProbe {
    fn metadata(&self, path: &Path) -> io::Result<SettingsFileMetadata> {
        let metadata = fs::symlink_metadata(path)?;
        let file_type = metadata.file_type();
        let kind = if file_type.is_symlink() {
            SettingsFileKind::Symlink
        } else if metadata.is_file() {
            SettingsFileKind::File
        } else {
            SettingsFileKind::Other
        };
        Ok(SettingsFileMetadata {
            kind,
            length: metadata.len(),
        })
    }

    fn read_limited(&self, path: &Path, limit: usize) -> io::Result<LimitedSettingsRead> {
        let file = File::open(path)?;
        let mut bytes = Vec::with_capacity(limit.min(8 * 1024));
        file.take(limit as u64 + 1).read_to_end(&mut bytes)?;
        if bytes.len() > limit {
            Ok(LimitedSettingsRead::TooLarge)
        } else {
            Ok(LimitedSettingsRead::Bytes(bytes))
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct SystemSettingsObservation;

impl SettingsObservationPort for SystemSettingsObservation {
    fn observe(
        &self,
        request: &SettingsObservationRequest,
    ) -> Result<Option<SettingsDocumentObservation>, ApplicationError> {
        #[cfg(any(target_os = "windows", target_os = "macos"))]
        {
            let _ = request;
            let paths = SystemPlatformPaths.resolve(&PlatformPathsRequest::default())?;
            observe_settings_file(paths.settings_file().as_path(), &SystemSettingsFileProbe)
        }

        #[cfg(not(any(target_os = "windows", target_os = "macos")))]
        {
            let _ = request;
            Err(ApplicationError::unsupported(
                "SETTINGS_OBSERVATION_UNSUPPORTED",
            ))
        }
    }
}

#[cfg(any(target_os = "windows", target_os = "macos", test))]
fn observe_settings_file(
    path: &Path,
    probe: &impl SettingsFileProbe,
) -> Result<Option<SettingsDocumentObservation>, ApplicationError> {
    let metadata = match probe.metadata(path) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(None),
        Err(_) => {
            return Err(ApplicationError::unavailable(
                "SETTINGS_OBSERVATION_UNAVAILABLE",
            ));
        }
    };

    if metadata.kind != SettingsFileKind::File {
        return Err(ApplicationError::invalid_input(
            "SETTINGS_OBSERVATION_INVALID_FILE_TYPE",
        ));
    }
    if metadata.length > SETTINGS_FILE_LIMIT as u64 {
        return Err(ApplicationError::invalid_input(
            "SETTINGS_OBSERVATION_TOO_LARGE",
        ));
    }

    let bytes = match probe.read_limited(path, SETTINGS_FILE_LIMIT) {
        Ok(LimitedSettingsRead::Bytes(bytes)) => bytes,
        Ok(LimitedSettingsRead::TooLarge) => {
            return Err(ApplicationError::invalid_input(
                "SETTINGS_OBSERVATION_TOO_LARGE",
            ));
        }
        Err(_) => {
            return Err(ApplicationError::unavailable(
                "SETTINGS_OBSERVATION_UNAVAILABLE",
            ));
        }
    };

    let value = serde_json::from_slice::<Value>(&bytes)
        .map_err(|_| ApplicationError::invalid_input("SETTINGS_OBSERVATION_INVALID_JSON"))?;
    let Value::Object(object) = value else {
        return Err(ApplicationError::invalid_input(
            "SETTINGS_OBSERVATION_INVALID_ROOT",
        ));
    };

    Ok(Some(SettingsDocumentObservation::new(object.len())))
}

#[cfg(test)]
mod tests {
    use std::{
        cell::RefCell,
        io,
        path::{Path, PathBuf},
    };

    use inputcodex_application::ErrorKind;

    use super::{
        LimitedSettingsRead, SETTINGS_FILE_LIMIT, SettingsFileKind, SettingsFileMetadata,
        SettingsFileProbe, observe_settings_file,
    };

    #[derive(Clone, Copy)]
    enum MetadataResult {
        Value(SettingsFileMetadata),
        Error(io::ErrorKind),
    }

    #[derive(Clone)]
    enum ReadResult {
        Bytes(Vec<u8>),
        TooLarge,
        Error(io::ErrorKind),
    }

    struct MemoryProbe {
        metadata: MetadataResult,
        read: ReadResult,
        read_limits: RefCell<Vec<usize>>,
    }

    impl MemoryProbe {
        fn new(metadata: MetadataResult, read: ReadResult) -> Self {
            Self {
                metadata,
                read,
                read_limits: RefCell::new(Vec::new()),
            }
        }
    }

    impl SettingsFileProbe for MemoryProbe {
        fn metadata(&self, _path: &Path) -> io::Result<SettingsFileMetadata> {
            match self.metadata {
                MetadataResult::Value(metadata) => Ok(metadata),
                MetadataResult::Error(kind) => Err(io::Error::from(kind)),
            }
        }

        fn read_limited(&self, _path: &Path, limit: usize) -> io::Result<LimitedSettingsRead> {
            self.read_limits.borrow_mut().push(limit);
            match &self.read {
                ReadResult::Bytes(bytes) => Ok(LimitedSettingsRead::Bytes(bytes.clone())),
                ReadResult::TooLarge => Ok(LimitedSettingsRead::TooLarge),
                ReadResult::Error(kind) => Err(io::Error::from(*kind)),
            }
        }
    }

    fn settings_path() -> PathBuf {
        std::env::temp_dir().join("inputcodex-settings-observation.json")
    }

    fn file_metadata(length: u64) -> MetadataResult {
        MetadataResult::Value(SettingsFileMetadata {
            kind: SettingsFileKind::File,
            length,
        })
    }

    #[test]
    fn 文件不存在返回_not_configured_且不读取() {
        let probe = MemoryProbe::new(
            MetadataResult::Error(io::ErrorKind::NotFound),
            ReadResult::Bytes(b"should-not-read".to_vec()),
        );

        assert_eq!(
            observe_settings_file(&settings_path(), &probe).expect("缺失文件应成功表达"),
            None
        );
        assert!(probe.read_limits.borrow().is_empty());
    }

    #[test]
    fn 合法对象只返回顶层条目数量且空对象仍是_some() {
        for (bytes, expected) in [
            (b"{}".as_slice(), 0),
            (br#"{"first":1,"second":{"secret":"hidden"}}"#.as_slice(), 2),
        ] {
            let probe = MemoryProbe::new(
                file_metadata(bytes.len() as u64),
                ReadResult::Bytes(bytes.to_vec()),
            );

            let observation = observe_settings_file(&settings_path(), &probe)
                .expect("合法对象应成功")
                .expect("合法对象应形成观察");

            assert_eq!(observation.top_level_entry_count(), expected);
            assert_eq!(
                probe.read_limits.borrow().as_slice(),
                &[SETTINGS_FILE_LIMIT]
            );
            assert!(!format!("{observation:?}").contains("secret"));
        }
    }

    #[test]
    fn 符号链接和非普通文件明确失败且不读取() {
        for kind in [SettingsFileKind::Symlink, SettingsFileKind::Other] {
            let probe = MemoryProbe::new(
                MetadataResult::Value(SettingsFileMetadata { kind, length: 0 }),
                ReadResult::Bytes(Vec::new()),
            );

            let error =
                observe_settings_file(&settings_path(), &probe).expect_err("非普通文件必须失败");

            assert_eq!(error.kind(), ErrorKind::InvalidInput);
            assert_eq!(
                error.code().as_str(),
                "SETTINGS_OBSERVATION_INVALID_FILE_TYPE"
            );
            assert!(probe.read_limits.borrow().is_empty());
        }
    }

    #[test]
    fn 元数据和实际读取均执行_256_kib_硬上限() {
        let metadata_too_large = MemoryProbe::new(
            file_metadata(SETTINGS_FILE_LIMIT as u64 + 1),
            ReadResult::Bytes(Vec::new()),
        );
        let error = observe_settings_file(&settings_path(), &metadata_too_large)
            .expect_err("元数据超限必须失败");
        assert_eq!(error.code().as_str(), "SETTINGS_OBSERVATION_TOO_LARGE");
        assert!(metadata_too_large.read_limits.borrow().is_empty());

        let read_too_large = MemoryProbe::new(file_metadata(1), ReadResult::TooLarge);
        let error = observe_settings_file(&settings_path(), &read_too_large)
            .expect_err("读取竞态超限必须失败");
        assert_eq!(error.code().as_str(), "SETTINGS_OBSERVATION_TOO_LARGE");
        assert_eq!(
            read_too_large.read_limits.borrow().as_slice(),
            &[SETTINGS_FILE_LIMIT]
        );
    }

    #[test]
    fn 元数据和读取_io_失败统一为_unavailable() {
        for probe in [
            MemoryProbe::new(
                MetadataResult::Error(io::ErrorKind::PermissionDenied),
                ReadResult::Bytes(Vec::new()),
            ),
            MemoryProbe::new(
                file_metadata(1),
                ReadResult::Error(io::ErrorKind::PermissionDenied),
            ),
        ] {
            let error =
                observe_settings_file(&settings_path(), &probe).expect_err("I/O 失败必须明确失败");
            assert_eq!(error.kind(), ErrorKind::Unavailable);
            assert_eq!(error.code().as_str(), "SETTINGS_OBSERVATION_UNAVAILABLE");
        }
    }

    #[test]
    fn 损坏_json_和非_utf8_使用稳定错误() {
        for bytes in [b"{".as_slice(), &[0xff, 0xfe]] {
            let probe = MemoryProbe::new(
                file_metadata(bytes.len() as u64),
                ReadResult::Bytes(bytes.to_vec()),
            );

            let error =
                observe_settings_file(&settings_path(), &probe).expect_err("损坏内容必须失败");
            assert_eq!(error.kind(), ErrorKind::InvalidInput);
            assert_eq!(error.code().as_str(), "SETTINGS_OBSERVATION_INVALID_JSON");
        }
    }

    #[test]
    fn 非_object_json_根使用稳定错误() {
        for bytes in [
            b"[]".as_slice(),
            b"null".as_slice(),
            br#""text""#.as_slice(),
        ] {
            let probe = MemoryProbe::new(
                file_metadata(bytes.len() as u64),
                ReadResult::Bytes(bytes.to_vec()),
            );

            let error =
                observe_settings_file(&settings_path(), &probe).expect_err("非 object 根必须失败");
            assert_eq!(error.kind(), ErrorKind::InvalidInput);
            assert_eq!(error.code().as_str(), "SETTINGS_OBSERVATION_INVALID_ROOT");
        }
    }
}
