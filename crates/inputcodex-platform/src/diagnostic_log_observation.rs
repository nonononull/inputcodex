#[cfg(any(target_os = "windows", target_os = "macos"))]
use std::{
    fs::{self, File},
    io::{Read, Seek, SeekFrom},
};
#[cfg(any(target_os = "windows", target_os = "macos", test))]
use std::{io, path::Path};

use inputcodex_application::{
    ApplicationError, DiagnosticLogObservationPort, DiagnosticLogObservationRequest,
};
#[cfg(any(target_os = "windows", target_os = "macos"))]
use inputcodex_application::{PlatformPathsPort, PlatformPathsRequest};
use inputcodex_domain::DiagnosticLogObservation;
#[cfg(any(target_os = "windows", target_os = "macos", test))]
use serde_json::Value;

#[cfg(any(target_os = "windows", target_os = "macos"))]
use crate::SystemPlatformPaths;

#[cfg(any(target_os = "windows", target_os = "macos", test))]
const DIAGNOSTIC_LOG_TAIL_LIMIT: usize = 256 * 1024;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg(any(target_os = "windows", target_os = "macos", test))]
enum DiagnosticLogFileKind {
    File,
    Symlink,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg(any(target_os = "windows", target_os = "macos", test))]
struct DiagnosticLogFileMetadata {
    kind: DiagnosticLogFileKind,
    length: u64,
}

#[cfg(any(target_os = "windows", target_os = "macos", test))]
trait DiagnosticLogFileProbe {
    fn metadata(&self, path: &Path) -> io::Result<DiagnosticLogFileMetadata>;
    fn read_tail(&self, path: &Path, start: u64, limit: usize) -> io::Result<Vec<u8>>;
}

#[derive(Debug, Clone, Copy, Default)]
#[cfg(any(target_os = "windows", target_os = "macos"))]
struct SystemDiagnosticLogFileProbe;

#[cfg(any(target_os = "windows", target_os = "macos"))]
impl DiagnosticLogFileProbe for SystemDiagnosticLogFileProbe {
    fn metadata(&self, path: &Path) -> io::Result<DiagnosticLogFileMetadata> {
        let metadata = fs::symlink_metadata(path)?;
        let file_type = metadata.file_type();
        let kind = if file_type.is_symlink() {
            DiagnosticLogFileKind::Symlink
        } else if metadata.is_file() {
            DiagnosticLogFileKind::File
        } else {
            DiagnosticLogFileKind::Other
        };
        Ok(DiagnosticLogFileMetadata {
            kind,
            length: metadata.len(),
        })
    }

    fn read_tail(&self, path: &Path, start: u64, limit: usize) -> io::Result<Vec<u8>> {
        let mut file = File::open(path)?;
        file.seek(SeekFrom::Start(start))?;
        let mut bytes = Vec::with_capacity(limit.min(8 * 1024));
        file.take(limit as u64).read_to_end(&mut bytes)?;
        Ok(bytes)
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct SystemDiagnosticLogObservation;

impl DiagnosticLogObservationPort for SystemDiagnosticLogObservation {
    fn observe(
        &self,
        request: &DiagnosticLogObservationRequest,
    ) -> Result<Option<DiagnosticLogObservation>, ApplicationError> {
        #[cfg(any(target_os = "windows", target_os = "macos"))]
        {
            let _ = request;
            let paths = SystemPlatformPaths.resolve(&PlatformPathsRequest::default())?;
            observe_diagnostic_log_file(
                paths.diagnostic_log_file().as_path(),
                &SystemDiagnosticLogFileProbe,
            )
        }

        #[cfg(not(any(target_os = "windows", target_os = "macos")))]
        {
            let _ = request;
            Err(ApplicationError::unsupported(
                "DIAGNOSTIC_LOG_OBSERVATION_UNSUPPORTED",
            ))
        }
    }
}

#[cfg(any(target_os = "windows", target_os = "macos", test))]
fn observe_diagnostic_log_file(
    path: &Path,
    probe: &impl DiagnosticLogFileProbe,
) -> Result<Option<DiagnosticLogObservation>, ApplicationError> {
    let metadata = match probe.metadata(path) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(None),
        Err(_) => {
            return Err(ApplicationError::unavailable(
                "DIAGNOSTIC_LOG_OBSERVATION_UNAVAILABLE",
            ));
        }
    };

    if metadata.kind != DiagnosticLogFileKind::File {
        return Err(ApplicationError::invalid_input(
            "DIAGNOSTIC_LOG_OBSERVATION_INVALID_FILE_TYPE",
        ));
    }

    let truncated = metadata.length > DIAGNOSTIC_LOG_TAIL_LIMIT as u64;
    let start = metadata
        .length
        .saturating_sub(DIAGNOSTIC_LOG_TAIL_LIMIT as u64);
    let bytes = probe
        .read_tail(path, start, DIAGNOSTIC_LOG_TAIL_LIMIT)
        .map_err(|_| ApplicationError::unavailable("DIAGNOSTIC_LOG_OBSERVATION_UNAVAILABLE"))?;
    let (records, partial_record_discarded) = drop_partial_record(&bytes, truncated);
    let (valid_object_record_count, malformed_record_count) = classify_records(records);

    Ok(Some(DiagnosticLogObservation::new(
        metadata.length,
        valid_object_record_count,
        malformed_record_count,
        truncated,
        partial_record_discarded,
    )))
}

#[cfg(any(target_os = "windows", target_os = "macos", test))]
fn drop_partial_record(bytes: &[u8], truncated: bool) -> (&[u8], bool) {
    if !truncated {
        return (bytes, false);
    }

    match bytes.iter().position(|byte| *byte == b'\n') {
        Some(index) => (&bytes[index + 1..], true),
        None => (&[], true),
    }
}

#[cfg(any(target_os = "windows", target_os = "macos", test))]
fn classify_records(bytes: &[u8]) -> (usize, usize) {
    if bytes.is_empty() {
        return (0, 0);
    }

    let mut valid_object_record_count = 0;
    let mut malformed_record_count = 0;
    let mut records = bytes.split(|byte| *byte == b'\n').peekable();

    while let Some(record) = records.next() {
        if record.is_empty() && records.peek().is_none() && bytes.last() == Some(&b'\n') {
            continue;
        }

        let record = if record.last() == Some(&b'\r') {
            &record[..record.len() - 1]
        } else {
            record
        };
        let is_valid_object = !record.is_empty()
            && matches!(
                serde_json::from_slice::<Value>(record),
                Ok(Value::Object(_))
            );

        if is_valid_object {
            valid_object_record_count += 1;
        } else {
            malformed_record_count += 1;
        }
    }

    (valid_object_record_count, malformed_record_count)
}

#[cfg(test)]
mod tests {
    use std::{
        cell::RefCell,
        io,
        path::{Path, PathBuf},
    };

    use inputcodex_application::ErrorKind;

    use super::*;

    enum MetadataResult {
        Value(DiagnosticLogFileMetadata),
        Error(io::ErrorKind),
    }

    enum ReadResult {
        Bytes(Vec<u8>),
        Error(io::ErrorKind),
    }

    struct MemoryProbe {
        metadata: MetadataResult,
        read: ReadResult,
        read_calls: RefCell<Vec<(u64, usize)>>,
    }

    impl MemoryProbe {
        fn new(metadata: MetadataResult, read: ReadResult) -> Self {
            Self {
                metadata,
                read,
                read_calls: RefCell::new(Vec::new()),
            }
        }
    }

    impl DiagnosticLogFileProbe for MemoryProbe {
        fn metadata(&self, _path: &Path) -> io::Result<DiagnosticLogFileMetadata> {
            match self.metadata {
                MetadataResult::Value(metadata) => Ok(metadata),
                MetadataResult::Error(kind) => Err(io::Error::from(kind)),
            }
        }

        fn read_tail(&self, _path: &Path, start: u64, limit: usize) -> io::Result<Vec<u8>> {
            self.read_calls.borrow_mut().push((start, limit));
            match &self.read {
                ReadResult::Bytes(bytes) => Ok(bytes.clone()),
                ReadResult::Error(kind) => Err(io::Error::from(*kind)),
            }
        }
    }

    fn file_metadata(length: u64) -> DiagnosticLogFileMetadata {
        DiagnosticLogFileMetadata {
            kind: DiagnosticLogFileKind::File,
            length,
        }
    }

    fn diagnostic_path() -> PathBuf {
        PathBuf::from("C:/private/inputcodex.log")
    }

    #[test]
    fn 文件不存在返回_empty_且不读取() {
        let probe = MemoryProbe::new(
            MetadataResult::Error(io::ErrorKind::NotFound),
            ReadResult::Bytes(Vec::new()),
        );

        let observation =
            observe_diagnostic_log_file(&diagnostic_path(), &probe).expect("缺失日志不应失败");

        assert!(observation.is_none());
        assert!(probe.read_calls.borrow().is_empty());
    }

    #[test]
    fn 合法空普通文件仍形成_ready_零计数事实() {
        let probe = MemoryProbe::new(
            MetadataResult::Value(file_metadata(0)),
            ReadResult::Bytes(Vec::new()),
        );

        let observation = observe_diagnostic_log_file(&diagnostic_path(), &probe)
            .expect("空文件应成功")
            .expect("空文件应形成观察");

        assert_eq!(observation.file_size_bytes(), 0);
        assert_eq!(observation.sampled_record_count(), 0);
        assert_eq!(observation.valid_object_record_count(), 0);
        assert_eq!(observation.malformed_record_count(), 0);
        assert!(!observation.truncated());
        assert!(!observation.partial_record_discarded());
        assert_eq!(
            probe.read_calls.borrow().as_slice(),
            &[(0, DIAGNOSTIC_LOG_TAIL_LIMIT)]
        );
    }

    #[test]
    fn 完整末尾无换行的_json_object_仍为合法记录() {
        let bytes = br#"{"event":"private","detail":"secret"}"#.to_vec();
        let probe = MemoryProbe::new(
            MetadataResult::Value(file_metadata(bytes.len() as u64)),
            ReadResult::Bytes(bytes),
        );

        let observation = observe_diagnostic_log_file(&diagnostic_path(), &probe)
            .expect("合法记录应成功")
            .expect("合法记录应形成观察");

        assert_eq!(observation.sampled_record_count(), 1);
        assert_eq!(observation.valid_object_record_count(), 1);
        assert_eq!(observation.malformed_record_count(), 0);
        assert!(!format!("{observation:?}").contains("secret"));
    }

    #[test]
    fn 空行_损坏_json_非_object_和非法_utf8_都计为_malformed() {
        let bytes = b"\n{\"ok\":true}\r\n[]\n{\n\xff".to_vec();
        let probe = MemoryProbe::new(
            MetadataResult::Value(file_metadata(bytes.len() as u64)),
            ReadResult::Bytes(bytes),
        );

        let observation = observe_diagnostic_log_file(&diagnostic_path(), &probe)
            .expect("单条损坏不应使整体失败")
            .expect("普通文件应形成观察");

        assert_eq!(observation.sampled_record_count(), 5);
        assert_eq!(observation.valid_object_record_count(), 1);
        assert_eq!(observation.malformed_record_count(), 4);
    }

    #[test]
    fn 超限文件只读取尾部并丢弃首个可能残片() {
        let length = DIAGNOSTIC_LOG_TAIL_LIMIT as u64 + 9;
        let probe = MemoryProbe::new(
            MetadataResult::Value(file_metadata(length)),
            ReadResult::Bytes(b"partial\n{\"new\":true}\n{}\n".to_vec()),
        );

        let observation = observe_diagnostic_log_file(&diagnostic_path(), &probe)
            .expect("尾部观察应成功")
            .expect("普通文件应形成观察");

        assert_eq!(observation.file_size_bytes(), length);
        assert_eq!(observation.sampled_record_count(), 2);
        assert_eq!(observation.valid_object_record_count(), 2);
        assert_eq!(observation.malformed_record_count(), 0);
        assert!(observation.truncated());
        assert!(observation.partial_record_discarded());
        assert_eq!(
            probe.read_calls.borrow().as_slice(),
            &[(9, DIAGNOSTIC_LOG_TAIL_LIMIT)]
        );
    }

    #[test]
    fn 超限窗口没有换行时丢弃全部可能残片() {
        let length = DIAGNOSTIC_LOG_TAIL_LIMIT as u64 + 1;
        let probe = MemoryProbe::new(
            MetadataResult::Value(file_metadata(length)),
            ReadResult::Bytes(b"partial-without-newline".to_vec()),
        );

        let observation = observe_diagnostic_log_file(&diagnostic_path(), &probe)
            .expect("尾部观察应成功")
            .expect("普通文件应形成观察");

        assert_eq!(observation.sampled_record_count(), 0);
        assert!(observation.truncated());
        assert!(observation.partial_record_discarded());
    }

    #[test]
    fn 符号链接和非普通文件明确失败且不读取() {
        for kind in [DiagnosticLogFileKind::Symlink, DiagnosticLogFileKind::Other] {
            let probe = MemoryProbe::new(
                MetadataResult::Value(DiagnosticLogFileMetadata { kind, length: 0 }),
                ReadResult::Bytes(Vec::new()),
            );

            let error = observe_diagnostic_log_file(&diagnostic_path(), &probe)
                .expect_err("非普通文件必须失败");

            assert_eq!(error.kind(), ErrorKind::InvalidInput);
            assert_eq!(
                error.code().as_str(),
                "DIAGNOSTIC_LOG_OBSERVATION_INVALID_FILE_TYPE"
            );
            assert!(probe.read_calls.borrow().is_empty());
        }
    }

    #[test]
    fn 元数据和尾部读取_io_失败统一为_unavailable() {
        for probe in [
            MemoryProbe::new(
                MetadataResult::Error(io::ErrorKind::PermissionDenied),
                ReadResult::Bytes(Vec::new()),
            ),
            MemoryProbe::new(
                MetadataResult::Value(file_metadata(1)),
                ReadResult::Error(io::ErrorKind::PermissionDenied),
            ),
        ] {
            let error = observe_diagnostic_log_file(&diagnostic_path(), &probe)
                .expect_err("I/O 失败必须明确失败");

            assert_eq!(error.kind(), ErrorKind::Unavailable);
            assert_eq!(
                error.code().as_str(),
                "DIAGNOSTIC_LOG_OBSERVATION_UNAVAILABLE"
            );
        }
    }
}
