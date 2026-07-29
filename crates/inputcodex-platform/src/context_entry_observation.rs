#[cfg(any(target_os = "windows", target_os = "macos", test))]
use std::{
    collections::HashMap,
    io,
    path::{Path, PathBuf},
};
#[cfg(any(target_os = "windows", target_os = "macos"))]
use std::{
    fs::{self, File},
    io::Read,
};

use inputcodex_application::{
    ApplicationError, ContextEntryObservationPort, ContextEntryObservationRequest,
};
#[cfg(any(target_os = "windows", target_os = "macos"))]
use inputcodex_application::{PlatformPathsPort, PlatformPathsRequest};
use inputcodex_domain::ContextEntryCatalogObservation;
#[cfg(any(target_os = "windows", target_os = "macos", test))]
use inputcodex_domain::{ContextEntryKind, ContextEntryObservation};
#[cfg(any(target_os = "windows", target_os = "macos", test))]
use toml_edit::{Document, DocumentMut, Table};

#[cfg(any(target_os = "windows", target_os = "macos"))]
use crate::SystemPlatformPaths;

#[cfg(any(target_os = "windows", target_os = "macos", test))]
const CONTEXT_ENTRY_FILE_LIMIT: usize = 256 * 1024;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg(any(target_os = "windows", target_os = "macos", test))]
enum ContextEntryFileKind {
    File,
    Symlink,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg(any(target_os = "windows", target_os = "macos", test))]
struct ContextEntryFileMetadata {
    kind: ContextEntryFileKind,
    length: u64,
}

#[cfg(any(target_os = "windows", target_os = "macos", test))]
enum LimitedContextEntryRead {
    Bytes(Vec<u8>),
    TooLarge,
}

#[cfg(any(target_os = "windows", target_os = "macos", test))]
trait ContextEntryFileProbe {
    fn metadata(&self, path: &Path) -> io::Result<ContextEntryFileMetadata>;
    fn read_limited(&self, path: &Path, limit: usize) -> io::Result<LimitedContextEntryRead>;
}

#[derive(Debug, Clone, Copy, Default)]
#[cfg(any(target_os = "windows", target_os = "macos"))]
struct SystemContextEntryFileProbe;

#[cfg(any(target_os = "windows", target_os = "macos"))]
impl ContextEntryFileProbe for SystemContextEntryFileProbe {
    fn metadata(&self, path: &Path) -> io::Result<ContextEntryFileMetadata> {
        let metadata = fs::symlink_metadata(path)?;
        let file_type = metadata.file_type();
        let kind = if file_type.is_symlink() {
            ContextEntryFileKind::Symlink
        } else if metadata.is_file() {
            ContextEntryFileKind::File
        } else {
            ContextEntryFileKind::Other
        };
        Ok(ContextEntryFileMetadata {
            kind,
            length: metadata.len(),
        })
    }

    fn read_limited(&self, path: &Path, limit: usize) -> io::Result<LimitedContextEntryRead> {
        let file = File::open(path)?;
        let mut bytes = Vec::with_capacity(limit.min(8 * 1024));
        file.take(limit as u64 + 1).read_to_end(&mut bytes)?;
        if bytes.len() > limit {
            Ok(LimitedContextEntryRead::TooLarge)
        } else {
            Ok(LimitedContextEntryRead::Bytes(bytes))
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct SystemContextEntryObservation;

impl ContextEntryObservationPort for SystemContextEntryObservation {
    fn observe(
        &self,
        request: &ContextEntryObservationRequest,
    ) -> Result<Option<ContextEntryCatalogObservation>, ApplicationError> {
        #[cfg(any(target_os = "windows", target_os = "macos"))]
        {
            let _ = request;
            let paths = SystemPlatformPaths.resolve(&PlatformPathsRequest::default())?;
            let config_path = context_entry_config_path(paths.codex_home().as_path());
            observe_context_entry_file(&config_path, &SystemContextEntryFileProbe)
        }

        #[cfg(not(any(target_os = "windows", target_os = "macos")))]
        {
            let _ = request;
            Err(ApplicationError::unsupported(
                "CONTEXT_ENTRY_OBSERVATION_UNSUPPORTED",
            ))
        }
    }
}

#[cfg(any(target_os = "windows", target_os = "macos", test))]
fn context_entry_config_path(codex_home: &Path) -> PathBuf {
    codex_home.join("config.toml")
}

#[cfg(any(target_os = "windows", target_os = "macos", test))]
fn observe_context_entry_file(
    path: &Path,
    probe: &impl ContextEntryFileProbe,
) -> Result<Option<ContextEntryCatalogObservation>, ApplicationError> {
    let metadata = match probe.metadata(path) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(None),
        Err(_) => {
            return Err(ApplicationError::unavailable(
                "CONTEXT_ENTRY_OBSERVATION_UNAVAILABLE",
            ));
        }
    };

    if metadata.kind != ContextEntryFileKind::File {
        return Err(ApplicationError::invalid_input(
            "CONTEXT_ENTRY_OBSERVATION_INVALID_FILE_TYPE",
        ));
    }
    if metadata.length > CONTEXT_ENTRY_FILE_LIMIT as u64 {
        return Err(ApplicationError::invalid_input(
            "CONTEXT_ENTRY_OBSERVATION_TOO_LARGE",
        ));
    }

    let bytes = match probe.read_limited(path, CONTEXT_ENTRY_FILE_LIMIT) {
        Ok(LimitedContextEntryRead::Bytes(bytes)) => bytes,
        Ok(LimitedContextEntryRead::TooLarge) => {
            return Err(ApplicationError::invalid_input(
                "CONTEXT_ENTRY_OBSERVATION_TOO_LARGE",
            ));
        }
        Err(_) => {
            return Err(ApplicationError::unavailable(
                "CONTEXT_ENTRY_OBSERVATION_UNAVAILABLE",
            ));
        }
    };

    parse_context_entries(&bytes).map(Some)
}

#[cfg(any(target_os = "windows", target_os = "macos", test))]
fn parse_context_entries(bytes: &[u8]) -> Result<ContextEntryCatalogObservation, ApplicationError> {
    let contents = std::str::from_utf8(bytes)
        .map_err(|_| ApplicationError::invalid_input("CONTEXT_ENTRY_OBSERVATION_INVALID_UTF8"))?;
    let source_document = Document::parse(contents.to_owned())
        .map_err(|_| ApplicationError::invalid_input("CONTEXT_ENTRY_OBSERVATION_INVALID_TOML"))?;
    let source_offsets = context_entry_source_offsets(&source_document);
    let document: DocumentMut = source_document.into_mut();

    let mut entries = Vec::new();
    let mut fallback_order = 0_usize;
    for (root_id, root_item) in document.as_table().iter() {
        let Some(kind) = context_entry_kind(root_id) else {
            continue;
        };
        let table = root_item.as_table().ok_or_else(|| {
            ApplicationError::invalid_input("CONTEXT_ENTRY_OBSERVATION_INVALID_ROOT_TABLE")
        })?;

        for (id, item) in table.iter() {
            let entry_table = item.as_table().ok_or_else(|| {
                ApplicationError::invalid_input("CONTEXT_ENTRY_OBSERVATION_INVALID_ENTRY_TABLE")
            })?;
            let enabled = context_entry_enabled(entry_table)?;
            let id = id.to_owned();
            let source_offset = source_offsets
                .get(&(kind, id.clone()))
                .copied()
                .unwrap_or(usize::MAX);
            let entry = ContextEntryObservation::new(id, kind, enabled).map_err(|_| {
                ApplicationError::invalid_input("CONTEXT_ENTRY_OBSERVATION_EMPTY_ID")
            })?;
            entries.push((source_offset, fallback_order, entry));
            fallback_order += 1;
        }
    }

    entries.sort_by_key(|(source_offset, order, _)| (*source_offset, *order));
    Ok(ContextEntryCatalogObservation::new(
        entries.into_iter().map(|(_, _, entry)| entry).collect(),
    ))
}

#[cfg(any(target_os = "windows", target_os = "macos", test))]
fn context_entry_source_offsets(
    document: &Document<String>,
) -> HashMap<(ContextEntryKind, String), usize> {
    let mut offsets = HashMap::new();
    for (root_id, root_item) in document.as_table().iter() {
        let Some(kind) = context_entry_kind(root_id) else {
            continue;
        };
        let Some(table) = root_item.as_table() else {
            continue;
        };
        for (id, item) in table.iter() {
            if let Some(span) = item.span() {
                offsets.insert((kind, id.to_owned()), span.start);
            }
        }
    }
    offsets
}

#[cfg(any(target_os = "windows", target_os = "macos", test))]
fn context_entry_kind(root_id: &str) -> Option<ContextEntryKind> {
    match root_id {
        "mcp_servers" => Some(ContextEntryKind::McpServer),
        "skills" => Some(ContextEntryKind::Skill),
        "plugins" => Some(ContextEntryKind::Plugin),
        _ => None,
    }
}

#[cfg(any(target_os = "windows", target_os = "macos", test))]
fn context_entry_enabled(table: &Table) -> Result<bool, ApplicationError> {
    let enabled = context_entry_boolean(table, "enabled")?;
    let disabled = context_entry_boolean(table, "disabled")?;
    Ok(!matches!(enabled, Some(false)) && !matches!(disabled, Some(true)))
}

#[cfg(any(target_os = "windows", target_os = "macos", test))]
fn context_entry_boolean(table: &Table, key: &str) -> Result<Option<bool>, ApplicationError> {
    let Some(item) = table.get(key) else {
        return Ok(None);
    };
    item.as_bool()
        .map(Some)
        .ok_or_else(|| ApplicationError::invalid_input("CONTEXT_ENTRY_OBSERVATION_INVALID_BOOLEAN"))
}

#[cfg(test)]
mod tests {
    use std::{
        cell::RefCell,
        io,
        path::{Path, PathBuf},
    };

    use inputcodex_application::ErrorKind;
    use inputcodex_domain::{ContextEntryKind, ContextEntryObservation};

    use super::{
        CONTEXT_ENTRY_FILE_LIMIT, ContextEntryFileKind, ContextEntryFileMetadata,
        ContextEntryFileProbe, LimitedContextEntryRead, context_entry_config_path,
        observe_context_entry_file,
    };

    #[derive(Clone, Copy)]
    enum MetadataResult {
        Value(ContextEntryFileMetadata),
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
        metadata_paths: RefCell<Vec<PathBuf>>,
        read_paths: RefCell<Vec<PathBuf>>,
        read_limits: RefCell<Vec<usize>>,
    }

    impl MemoryProbe {
        fn new(metadata: MetadataResult, read: ReadResult) -> Self {
            Self {
                metadata,
                read,
                metadata_paths: RefCell::new(Vec::new()),
                read_paths: RefCell::new(Vec::new()),
                read_limits: RefCell::new(Vec::new()),
            }
        }
    }

    impl ContextEntryFileProbe for MemoryProbe {
        fn metadata(&self, path: &Path) -> io::Result<ContextEntryFileMetadata> {
            self.metadata_paths.borrow_mut().push(path.to_path_buf());
            match self.metadata {
                MetadataResult::Value(metadata) => Ok(metadata),
                MetadataResult::Error(kind) => Err(io::Error::from(kind)),
            }
        }

        fn read_limited(&self, path: &Path, limit: usize) -> io::Result<LimitedContextEntryRead> {
            self.read_paths.borrow_mut().push(path.to_path_buf());
            self.read_limits.borrow_mut().push(limit);
            match &self.read {
                ReadResult::Bytes(bytes) => Ok(LimitedContextEntryRead::Bytes(bytes.clone())),
                ReadResult::TooLarge => Ok(LimitedContextEntryRead::TooLarge),
                ReadResult::Error(kind) => Err(io::Error::from(*kind)),
            }
        }
    }

    fn config_path() -> PathBuf {
        std::env::temp_dir().join("inputcodex-context-entry-observation-config.toml")
    }

    fn file_metadata(length: u64) -> MetadataResult {
        MetadataResult::Value(ContextEntryFileMetadata {
            kind: ContextEntryFileKind::File,
            length,
        })
    }

    fn observe_bytes(
        bytes: &[u8],
    ) -> Result<
        inputcodex_domain::ContextEntryCatalogObservation,
        inputcodex_application::ApplicationError,
    > {
        let probe = MemoryProbe::new(
            file_metadata(bytes.len() as u64),
            ReadResult::Bytes(bytes.to_vec()),
        );
        observe_context_entry_file(&config_path(), &probe)
            .map(|value| value.expect("文件存在应形成观察"))
    }

    #[test]
    fn 固定路径只从_codex_home_派生_config_toml() {
        let home = std::env::temp_dir().join("inputcodex-context-entry-home");

        assert_eq!(context_entry_config_path(&home), home.join("config.toml"));
    }

    #[test]
    fn 文件不存在返回_empty_且不读取() {
        let path = config_path();
        let probe = MemoryProbe::new(
            MetadataResult::Error(io::ErrorKind::NotFound),
            ReadResult::Bytes(b"should-not-read".to_vec()),
        );

        assert_eq!(observe_context_entry_file(&path, &probe), Ok(None));
        assert_eq!(probe.metadata_paths.borrow().as_slice(), &[path]);
        assert!(probe.read_paths.borrow().is_empty());
        assert!(probe.read_limits.borrow().is_empty());
    }

    #[test]
    fn 合法空_toml_和无目标表均返回_ready_零条目() {
        for bytes in [b"".as_slice(), b"model = \"gpt-5\"\n".as_slice()] {
            let observation = observe_bytes(bytes).expect("合法 TOML 应成功");

            assert!(observation.entries().is_empty());
            for kind in [
                ContextEntryKind::McpServer,
                ContextEntryKind::Skill,
                ContextEntryKind::Plugin,
            ] {
                assert_eq!(observation.summary(kind).total(), 0);
            }
        }
    }

    #[test]
    fn 三类条目保留源顺序并只公开最小状态() {
        let contents = br#"
[skills.writer]
enabled = false
summary = "private-summary"

[mcp_servers.context7]
command = "secret-command"
args = ["secret-argument"]
env = { TOKEN = "secret-token" }
url = "https://secret.example"
disabled = false

[plugins.local]
disabled = true
path = "C:/Users/secret/plugin"

[skills.reviewer]
enabled = true
"#;

        let observation = observe_bytes(contents).expect("合法三类目录应成功");
        let entries = observation.entries();

        assert_eq!(
            entries
                .iter()
                .map(ContextEntryObservation::id)
                .collect::<Vec<_>>(),
            vec!["writer", "context7", "local", "reviewer"]
        );
        assert_eq!(
            entries
                .iter()
                .map(ContextEntryObservation::kind)
                .collect::<Vec<_>>(),
            vec![
                ContextEntryKind::Skill,
                ContextEntryKind::McpServer,
                ContextEntryKind::Plugin,
                ContextEntryKind::Skill,
            ]
        );
        assert_eq!(
            entries
                .iter()
                .map(ContextEntryObservation::is_enabled)
                .collect::<Vec<_>>(),
            vec![false, true, false, true]
        );
        assert_eq!(observation.summary(ContextEntryKind::McpServer).total(), 1);
        assert_eq!(observation.summary(ContextEntryKind::Skill).enabled(), 1);
        assert_eq!(observation.summary(ContextEntryKind::Skill).disabled(), 1);
        assert_eq!(observation.summary(ContextEntryKind::Plugin).disabled(), 1);

        let debug = format!("{observation:?}");
        for forbidden in [
            "private-summary",
            "secret-command",
            "secret-argument",
            "secret-token",
            "https://secret.example",
            "C:/Users/secret/plugin",
            "toml_body",
        ] {
            assert!(!debug.contains(forbidden), "Debug 泄露了 {forbidden}");
        }
    }

    #[test]
    fn 启用和禁用布尔组合遵循批准优先级() {
        let contents = br#"
[mcp_servers.default]
[mcp_servers.enabled_true]
enabled = true
[mcp_servers.enabled_false]
enabled = false
[mcp_servers.disabled_false]
disabled = false
[mcp_servers.disabled_true]
disabled = true
[mcp_servers.both_allow]
enabled = true
disabled = false
[mcp_servers.enabled_denies]
enabled = false
disabled = false
[mcp_servers.disabled_denies]
enabled = true
disabled = true
"#;

        let observation = observe_bytes(contents).expect("合法布尔组合应成功");
        let states = observation
            .entries()
            .iter()
            .map(|entry| (entry.id(), entry.is_enabled()))
            .collect::<Vec<_>>();

        assert_eq!(
            states,
            vec![
                ("default", true),
                ("enabled_true", true),
                ("enabled_false", false),
                ("disabled_false", true),
                ("disabled_true", false),
                ("both_allow", true),
                ("enabled_denies", false),
                ("disabled_denies", false),
            ]
        );
    }

    #[test]
    fn 错误布尔类型使整个配置失败() {
        for contents in [
            b"[mcp_servers.context7]\nenabled = \"false\"\n".as_slice(),
            b"[skills.writer]\ndisabled = 1\n".as_slice(),
        ] {
            let error = observe_bytes(contents).expect_err("错误布尔类型必须失败");

            assert_eq!(error.kind(), ErrorKind::InvalidInput);
            assert_eq!(
                error.code().as_str(),
                "CONTEXT_ENTRY_OBSERVATION_INVALID_BOOLEAN"
            );
        }
    }

    #[test]
    fn 根项和子项必须是标准_toml_table() {
        for (contents, code) in [
            (
                b"mcp_servers = []\n".as_slice(),
                "CONTEXT_ENTRY_OBSERVATION_INVALID_ROOT_TABLE",
            ),
            (
                b"mcp_servers = { context7 = { enabled = true } }\n".as_slice(),
                "CONTEXT_ENTRY_OBSERVATION_INVALID_ROOT_TABLE",
            ),
            (
                b"[skills]\nwriter = \"enabled\"\n".as_slice(),
                "CONTEXT_ENTRY_OBSERVATION_INVALID_ENTRY_TABLE",
            ),
            (
                b"[plugins]\nlocal = { enabled = true }\n".as_slice(),
                "CONTEXT_ENTRY_OBSERVATION_INVALID_ENTRY_TABLE",
            ),
        ] {
            let error = observe_bytes(contents).expect_err("错误 table 类型必须失败");
            assert_eq!(error.kind(), ErrorKind::InvalidInput);
            assert_eq!(error.code().as_str(), code);
        }
    }

    #[test]
    fn 空白条目标识使用稳定错误() {
        for contents in [
            b"[skills.\"\"]\nenabled = true\n".as_slice(),
            b"[plugins.\"   \"]\nenabled = true\n".as_slice(),
        ] {
            let error = observe_bytes(contents).expect_err("空白 ID 必须失败");
            assert_eq!(error.kind(), ErrorKind::InvalidInput);
            assert_eq!(error.code().as_str(), "CONTEXT_ENTRY_OBSERVATION_EMPTY_ID");
        }
    }

    #[test]
    fn 非法_toml_重复定义和非法_utf8_明确失败() {
        for contents in [
            b"[mcp_servers.context7\n".as_slice(),
            b"[mcp_servers.context7]\nenabled = true\n[mcp_servers.context7]\nenabled = false\n"
                .as_slice(),
        ] {
            let error = observe_bytes(contents).expect_err("非法 TOML 必须失败");
            assert_eq!(error.kind(), ErrorKind::InvalidInput);
            assert_eq!(
                error.code().as_str(),
                "CONTEXT_ENTRY_OBSERVATION_INVALID_TOML"
            );
        }

        let error = observe_bytes(&[0xff, 0xfe]).expect_err("非法 UTF-8 必须失败");
        assert_eq!(error.kind(), ErrorKind::InvalidInput);
        assert_eq!(
            error.code().as_str(),
            "CONTEXT_ENTRY_OBSERVATION_INVALID_UTF8"
        );
    }

    #[test]
    fn 符号链接和非普通文件明确失败且不读取() {
        for kind in [ContextEntryFileKind::Symlink, ContextEntryFileKind::Other] {
            let probe = MemoryProbe::new(
                MetadataResult::Value(ContextEntryFileMetadata { kind, length: 0 }),
                ReadResult::Bytes(Vec::new()),
            );

            let error =
                observe_context_entry_file(&config_path(), &probe).expect_err("非普通文件必须失败");

            assert_eq!(error.kind(), ErrorKind::InvalidInput);
            assert_eq!(
                error.code().as_str(),
                "CONTEXT_ENTRY_OBSERVATION_INVALID_FILE_TYPE"
            );
            assert!(probe.read_paths.borrow().is_empty());
        }
    }

    #[test]
    fn 元数据和实际读取均执行_256_kib_硬上限() {
        let metadata_too_large = MemoryProbe::new(
            file_metadata(CONTEXT_ENTRY_FILE_LIMIT as u64 + 1),
            ReadResult::Bytes(Vec::new()),
        );
        let error = observe_context_entry_file(&config_path(), &metadata_too_large)
            .expect_err("元数据超限必须失败");
        assert_eq!(error.code().as_str(), "CONTEXT_ENTRY_OBSERVATION_TOO_LARGE");
        assert!(metadata_too_large.read_paths.borrow().is_empty());

        let read_too_large = MemoryProbe::new(file_metadata(1), ReadResult::TooLarge);
        let error = observe_context_entry_file(&config_path(), &read_too_large)
            .expect_err("读取增长越界必须失败");
        assert_eq!(error.code().as_str(), "CONTEXT_ENTRY_OBSERVATION_TOO_LARGE");
        assert_eq!(
            read_too_large.read_limits.borrow().as_slice(),
            &[CONTEXT_ENTRY_FILE_LIMIT]
        );
    }

    #[test]
    fn 元数据和读取_io_失败统一为_unavailable_且不泄露路径() {
        let secret_path = std::env::temp_dir().join("secret-account-token-config.toml");
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
                observe_context_entry_file(&secret_path, &probe).expect_err("I/O 失败必须明确失败");

            assert_eq!(error.kind(), ErrorKind::Unavailable);
            assert_eq!(
                error.code().as_str(),
                "CONTEXT_ENTRY_OBSERVATION_UNAVAILABLE"
            );
            let debug = format!("{error:?}");
            assert!(!debug.contains("secret-account-token-config.toml"));
        }
    }
}
