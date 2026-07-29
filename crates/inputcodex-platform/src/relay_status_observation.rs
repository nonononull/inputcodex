#[cfg(any(target_os = "windows", target_os = "macos"))]
use std::{
    fs::{self, File},
    io::Read,
};
#[cfg(any(target_os = "windows", target_os = "macos", test))]
use std::{io, path::Path, str};

use inputcodex_application::{
    ApplicationError, RelayStatusObservationPort, RelayStatusObservationRequest,
};
#[cfg(any(target_os = "windows", target_os = "macos"))]
use inputcodex_application::{PlatformPathsPort, PlatformPathsRequest};
use inputcodex_domain::RelayStatusObservation;
#[cfg(any(target_os = "windows", target_os = "macos", test))]
use inputcodex_domain::{CredentialPresence, RelayConfigurationStatus, RelayDocumentStatus};
#[cfg(any(target_os = "windows", target_os = "macos", test))]
use serde_json::Value;
#[cfg(any(target_os = "windows", target_os = "macos", test))]
use toml_edit::{DocumentMut, Item};

#[cfg(any(target_os = "windows", target_os = "macos"))]
use crate::SystemPlatformPaths;

#[cfg(any(target_os = "windows", target_os = "macos"))]
const AUTH_FILE: &str = "auth.json";
#[cfg(any(target_os = "windows", target_os = "macos"))]
const CONFIG_FILE: &str = "config.toml";
#[cfg(any(target_os = "windows", target_os = "macos", test))]
const RELAY_STATUS_FILE_LIMIT: usize = 256 * 1024;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg(any(target_os = "windows", target_os = "macos", test))]
enum RelayStatusFileKind {
    File,
    Symlink,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg(any(target_os = "windows", target_os = "macos", test))]
struct RelayStatusFileMetadata {
    kind: RelayStatusFileKind,
    length: u64,
}

#[cfg(any(target_os = "windows", target_os = "macos", test))]
enum LimitedRelayStatusRead {
    Bytes(Vec<u8>),
    TooLarge,
}

#[cfg(any(target_os = "windows", target_os = "macos", test))]
trait RelayStatusFileProbe {
    fn metadata(&self, path: &Path) -> io::Result<RelayStatusFileMetadata>;
    fn read_limited(&self, path: &Path, limit: usize) -> io::Result<LimitedRelayStatusRead>;
}

#[derive(Debug, Clone, Copy, Default)]
#[cfg(any(target_os = "windows", target_os = "macos"))]
struct SystemRelayStatusFileProbe;

#[cfg(any(target_os = "windows", target_os = "macos"))]
impl RelayStatusFileProbe for SystemRelayStatusFileProbe {
    fn metadata(&self, path: &Path) -> io::Result<RelayStatusFileMetadata> {
        let metadata = fs::symlink_metadata(path)?;
        let file_type = metadata.file_type();
        let kind = if file_type.is_symlink() {
            RelayStatusFileKind::Symlink
        } else if metadata.is_file() {
            RelayStatusFileKind::File
        } else {
            RelayStatusFileKind::Other
        };
        Ok(RelayStatusFileMetadata {
            kind,
            length: metadata.len(),
        })
    }

    fn read_limited(&self, path: &Path, limit: usize) -> io::Result<LimitedRelayStatusRead> {
        let file = File::open(path)?;
        let mut bytes = Vec::with_capacity(limit.min(8 * 1024));
        file.take(limit as u64 + 1).read_to_end(&mut bytes)?;
        if bytes.len() > limit {
            Ok(LimitedRelayStatusRead::TooLarge)
        } else {
            Ok(LimitedRelayStatusRead::Bytes(bytes))
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg(any(target_os = "windows", target_os = "macos", test))]
struct AuthDocumentObservation {
    status: RelayDocumentStatus,
    chatgpt_credentials: CredentialPresence,
    openai_api_key: CredentialPresence,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg(any(target_os = "windows", target_os = "macos", test))]
enum RelayConfigurationBasis {
    NotConfigured,
    Incomplete,
    CredentialRequired { has_bearer_token: bool },
    NotObserved,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg(any(target_os = "windows", target_os = "macos", test))]
struct ConfigDocumentObservation {
    status: RelayDocumentStatus,
    basis: RelayConfigurationBasis,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct SystemRelayStatusObservation;

impl RelayStatusObservationPort for SystemRelayStatusObservation {
    fn observe(
        &self,
        request: &RelayStatusObservationRequest,
    ) -> Result<Option<RelayStatusObservation>, ApplicationError> {
        #[cfg(any(target_os = "windows", target_os = "macos"))]
        {
            let _ = request;
            let paths = SystemPlatformPaths.resolve(&PlatformPathsRequest::default())?;
            let auth_path = paths.codex_home().as_path().join(AUTH_FILE);
            let config_path = paths.codex_home().as_path().join(CONFIG_FILE);
            Ok(observe_relay_status_files(
                auth_path.as_path(),
                config_path.as_path(),
                &SystemRelayStatusFileProbe,
            ))
        }

        #[cfg(not(any(target_os = "windows", target_os = "macos")))]
        {
            let _ = request;
            Err(ApplicationError::unsupported(
                "RELAY_STATUS_OBSERVATION_UNSUPPORTED",
            ))
        }
    }
}

#[cfg(any(target_os = "windows", target_os = "macos", test))]
fn observe_relay_status_files(
    auth_path: &Path,
    config_path: &Path,
    probe: &impl RelayStatusFileProbe,
) -> Option<RelayStatusObservation> {
    let auth = observe_auth_document(auth_path, probe);
    let config = observe_config_document(config_path, probe);

    if auth.status == RelayDocumentStatus::Missing && config.status == RelayDocumentStatus::Missing
    {
        return None;
    }

    let relay_configuration = match config.basis {
        RelayConfigurationBasis::NotConfigured => RelayConfigurationStatus::NotConfigured,
        RelayConfigurationBasis::Incomplete => RelayConfigurationStatus::Incomplete,
        RelayConfigurationBasis::NotObserved => RelayConfigurationStatus::NotObserved,
        RelayConfigurationBasis::CredentialRequired {
            has_bearer_token: true,
        } => RelayConfigurationStatus::Complete,
        RelayConfigurationBasis::CredentialRequired {
            has_bearer_token: false,
        } => match auth.openai_api_key {
            CredentialPresence::Present => RelayConfigurationStatus::Complete,
            CredentialPresence::Absent => RelayConfigurationStatus::Incomplete,
            CredentialPresence::NotObserved => RelayConfigurationStatus::NotObserved,
        },
    };

    Some(RelayStatusObservation::new(
        auth.status,
        config.status,
        auth.chatgpt_credentials,
        auth.openai_api_key,
        relay_configuration,
    ))
}

#[cfg(any(target_os = "windows", target_os = "macos", test))]
fn observe_auth_document(
    path: &Path,
    probe: &impl RelayStatusFileProbe,
) -> AuthDocumentObservation {
    let bytes = match read_document(path, probe) {
        Ok(bytes) => bytes,
        Err(status) => {
            let presence = if status == RelayDocumentStatus::Missing {
                CredentialPresence::Absent
            } else {
                CredentialPresence::NotObserved
            };
            return AuthDocumentObservation {
                status,
                chatgpt_credentials: presence,
                openai_api_key: presence,
            };
        }
    };

    let Ok(Value::Object(object)) = serde_json::from_slice::<Value>(&bytes) else {
        return AuthDocumentObservation {
            status: RelayDocumentStatus::Invalid,
            chatgpt_credentials: CredentialPresence::NotObserved,
            openai_api_key: CredentialPresence::NotObserved,
        };
    };

    let is_chatgpt = object
        .get("auth_mode")
        .and_then(Value::as_str)
        .is_some_and(|mode| mode.eq_ignore_ascii_case("chatgpt"));
    let has_login_secret = object
        .get("tokens")
        .and_then(Value::as_object)
        .is_some_and(|tokens| {
            ["access_token", "id_token", "refresh_token"]
                .iter()
                .any(|key| has_non_empty_json_string(tokens.get(*key)))
        });
    let has_openai_api_key = has_non_empty_json_string(object.get("OPENAI_API_KEY"));

    AuthDocumentObservation {
        status: RelayDocumentStatus::Valid,
        chatgpt_credentials: presence(is_chatgpt && has_login_secret),
        openai_api_key: presence(has_openai_api_key),
    }
}

#[cfg(any(target_os = "windows", target_os = "macos", test))]
fn observe_config_document(
    path: &Path,
    probe: &impl RelayStatusFileProbe,
) -> ConfigDocumentObservation {
    let bytes = match read_document(path, probe) {
        Ok(bytes) => bytes,
        Err(status) => {
            let basis = if status == RelayDocumentStatus::Missing {
                RelayConfigurationBasis::NotConfigured
            } else {
                RelayConfigurationBasis::NotObserved
            };
            return ConfigDocumentObservation { status, basis };
        }
    };

    let Ok(contents) = str::from_utf8(&bytes) else {
        return invalid_config_document();
    };
    let Ok(document) = contents.parse::<DocumentMut>() else {
        return invalid_config_document();
    };

    let Some(provider_id) = document
        .get("model_provider")
        .and_then(Item::as_str)
        .map(str::trim)
        .filter(|provider| !provider.is_empty())
    else {
        return ConfigDocumentObservation {
            status: RelayDocumentStatus::Valid,
            basis: RelayConfigurationBasis::NotConfigured,
        };
    };

    let Some(provider) = document
        .get("model_providers")
        .and_then(Item::as_table)
        .and_then(|providers| providers.get(provider_id))
        .and_then(Item::as_table)
    else {
        return ConfigDocumentObservation {
            status: RelayDocumentStatus::Valid,
            basis: RelayConfigurationBasis::Incomplete,
        };
    };

    let requires_openai_auth =
        provider.get("requires_openai_auth").and_then(Item::as_bool) == Some(true);
    let has_base_url = provider
        .get("base_url")
        .and_then(Item::as_str)
        .is_some_and(|value| !value.trim().is_empty());
    let has_bearer_token = provider
        .get("experimental_bearer_token")
        .and_then(Item::as_str)
        .is_some_and(|value| !value.trim().is_empty());

    let basis = if requires_openai_auth && has_base_url {
        RelayConfigurationBasis::CredentialRequired { has_bearer_token }
    } else {
        RelayConfigurationBasis::Incomplete
    };

    ConfigDocumentObservation {
        status: RelayDocumentStatus::Valid,
        basis,
    }
}

#[cfg(any(target_os = "windows", target_os = "macos", test))]
fn invalid_config_document() -> ConfigDocumentObservation {
    ConfigDocumentObservation {
        status: RelayDocumentStatus::Invalid,
        basis: RelayConfigurationBasis::NotObserved,
    }
}

#[cfg(any(target_os = "windows", target_os = "macos", test))]
fn read_document(
    path: &Path,
    probe: &impl RelayStatusFileProbe,
) -> Result<Vec<u8>, RelayDocumentStatus> {
    let metadata = match probe.metadata(path) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == io::ErrorKind::NotFound => {
            return Err(RelayDocumentStatus::Missing);
        }
        Err(_) => return Err(RelayDocumentStatus::Unreadable),
    };

    if metadata.kind != RelayStatusFileKind::File {
        return Err(RelayDocumentStatus::Unreadable);
    }
    if metadata.length > RELAY_STATUS_FILE_LIMIT as u64 {
        return Err(RelayDocumentStatus::TooLarge);
    }

    match probe.read_limited(path, RELAY_STATUS_FILE_LIMIT) {
        Ok(LimitedRelayStatusRead::Bytes(bytes)) => Ok(bytes),
        Ok(LimitedRelayStatusRead::TooLarge) => Err(RelayDocumentStatus::TooLarge),
        Err(_) => Err(RelayDocumentStatus::Unreadable),
    }
}

#[cfg(any(target_os = "windows", target_os = "macos", test))]
fn has_non_empty_json_string(value: Option<&Value>) -> bool {
    value
        .and_then(Value::as_str)
        .is_some_and(|value| !value.trim().is_empty())
}

#[cfg(any(target_os = "windows", target_os = "macos", test))]
const fn presence(present: bool) -> CredentialPresence {
    if present {
        CredentialPresence::Present
    } else {
        CredentialPresence::Absent
    }
}

#[cfg(test)]
mod tests {
    use std::{
        cell::RefCell,
        io,
        path::{Path, PathBuf},
    };

    use inputcodex_domain::{CredentialPresence, RelayConfigurationStatus, RelayDocumentStatus};

    use super::{
        LimitedRelayStatusRead, RELAY_STATUS_FILE_LIMIT, RelayStatusFileKind,
        RelayStatusFileMetadata, RelayStatusFileProbe, observe_relay_status_files,
    };

    #[derive(Clone, Copy)]
    enum MetadataResult {
        Value(RelayStatusFileMetadata),
        Error(io::ErrorKind),
    }

    #[derive(Clone)]
    enum ReadResult {
        Bytes(Vec<u8>),
        TooLarge,
        Error(io::ErrorKind),
    }

    #[derive(Clone)]
    struct DocumentSpec {
        metadata: MetadataResult,
        read: ReadResult,
    }

    impl DocumentSpec {
        fn missing() -> Self {
            Self {
                metadata: MetadataResult::Error(io::ErrorKind::NotFound),
                read: ReadResult::Bytes(Vec::new()),
            }
        }

        fn file(bytes: &[u8]) -> Self {
            Self {
                metadata: MetadataResult::Value(RelayStatusFileMetadata {
                    kind: RelayStatusFileKind::File,
                    length: bytes.len() as u64,
                }),
                read: ReadResult::Bytes(bytes.to_vec()),
            }
        }

        fn kind(kind: RelayStatusFileKind) -> Self {
            Self {
                metadata: MetadataResult::Value(RelayStatusFileMetadata { kind, length: 0 }),
                read: ReadResult::Bytes(Vec::new()),
            }
        }

        fn metadata_too_large() -> Self {
            Self {
                metadata: MetadataResult::Value(RelayStatusFileMetadata {
                    kind: RelayStatusFileKind::File,
                    length: RELAY_STATUS_FILE_LIMIT as u64 + 1,
                }),
                read: ReadResult::Bytes(Vec::new()),
            }
        }

        fn read_too_large() -> Self {
            Self {
                metadata: MetadataResult::Value(RelayStatusFileMetadata {
                    kind: RelayStatusFileKind::File,
                    length: 1,
                }),
                read: ReadResult::TooLarge,
            }
        }

        fn metadata_error(kind: io::ErrorKind) -> Self {
            Self {
                metadata: MetadataResult::Error(kind),
                read: ReadResult::Bytes(Vec::new()),
            }
        }

        fn read_error(kind: io::ErrorKind) -> Self {
            Self {
                metadata: MetadataResult::Value(RelayStatusFileMetadata {
                    kind: RelayStatusFileKind::File,
                    length: 1,
                }),
                read: ReadResult::Error(kind),
            }
        }
    }

    struct MemoryProbe {
        auth: DocumentSpec,
        config: DocumentSpec,
        metadata_paths: RefCell<Vec<PathBuf>>,
        read_requests: RefCell<Vec<(PathBuf, usize)>>,
    }

    impl MemoryProbe {
        fn new(auth: DocumentSpec, config: DocumentSpec) -> Self {
            Self {
                auth,
                config,
                metadata_paths: RefCell::new(Vec::new()),
                read_requests: RefCell::new(Vec::new()),
            }
        }

        fn spec(&self, path: &Path) -> &DocumentSpec {
            match path.file_name().and_then(|name| name.to_str()) {
                Some("auth.json") => &self.auth,
                Some("config.toml") => &self.config,
                _ => panic!("观察器访问了未批准文件名"),
            }
        }
    }

    impl RelayStatusFileProbe for MemoryProbe {
        fn metadata(&self, path: &Path) -> io::Result<RelayStatusFileMetadata> {
            self.metadata_paths.borrow_mut().push(path.to_path_buf());
            match self.spec(path).metadata {
                MetadataResult::Value(metadata) => Ok(metadata),
                MetadataResult::Error(kind) => Err(io::Error::from(kind)),
            }
        }

        fn read_limited(&self, path: &Path, limit: usize) -> io::Result<LimitedRelayStatusRead> {
            self.read_requests
                .borrow_mut()
                .push((path.to_path_buf(), limit));
            match &self.spec(path).read {
                ReadResult::Bytes(bytes) => Ok(LimitedRelayStatusRead::Bytes(bytes.clone())),
                ReadResult::TooLarge => Ok(LimitedRelayStatusRead::TooLarge),
                ReadResult::Error(kind) => Err(io::Error::from(*kind)),
            }
        }
    }

    fn auth_path() -> PathBuf {
        PathBuf::from("private-home").join("auth.json")
    }

    fn config_path() -> PathBuf {
        PathBuf::from("private-home").join("config.toml")
    }

    fn observe(probe: &MemoryProbe) -> Option<inputcodex_domain::RelayStatusObservation> {
        observe_relay_status_files(&auth_path(), &config_path(), probe)
    }

    #[test]
    fn 两份固定文档均缺失返回_none_且不读取() {
        let probe = MemoryProbe::new(DocumentSpec::missing(), DocumentSpec::missing());

        assert_eq!(observe(&probe), None);
        assert!(probe.read_requests.borrow().is_empty());
    }

    #[test]
    fn 合法认证和配置通过_openai_api_key_形成_complete() {
        let auth = br#"{
            "auth_mode":"chatgpt",
            "tokens":{"access_token":"test-only"},
            "OPENAI_API_KEY":"test-only"
        }"#;
        let config = br#"
            model_provider = "hidden-provider"

            [model_providers.hidden-provider]
            requires_openai_auth = true
            base_url = "test-only"
        "#;
        let probe = MemoryProbe::new(DocumentSpec::file(auth), DocumentSpec::file(config));

        let observation = observe(&probe).expect("任一文档存在必须形成观察");

        assert_eq!(
            observation.auth_document_status(),
            RelayDocumentStatus::Valid
        );
        assert_eq!(
            observation.config_document_status(),
            RelayDocumentStatus::Valid
        );
        assert_eq!(
            observation.chatgpt_credentials(),
            CredentialPresence::Present
        );
        assert_eq!(observation.openai_api_key(), CredentialPresence::Present);
        assert_eq!(
            observation.relay_configuration(),
            RelayConfigurationStatus::Complete
        );
    }

    #[test]
    fn bearer_token_可在认证文档不可读时独立形成_complete() {
        let config = br#"
            model_provider = "hidden-provider"

            [model_providers.hidden-provider]
            requires_openai_auth = true
            base_url = "test-only"
            experimental_bearer_token = "test-only"
        "#;
        let probe = MemoryProbe::new(
            DocumentSpec::metadata_error(io::ErrorKind::PermissionDenied),
            DocumentSpec::file(config),
        );

        let observation = observe(&probe).expect("配置文档存在必须形成观察");

        assert_eq!(
            observation.auth_document_status(),
            RelayDocumentStatus::Unreadable
        );
        assert_eq!(
            observation.openai_api_key(),
            CredentialPresence::NotObserved
        );
        assert_eq!(
            observation.relay_configuration(),
            RelayConfigurationStatus::Complete
        );
    }

    #[test]
    fn relay_配置明确区分未配置不完整和不可观察() {
        let cases = [
            (
                DocumentSpec::missing(),
                DocumentSpec::file(br#"{"OPENAI_API_KEY":"test-only"}"#),
                RelayConfigurationStatus::NotConfigured,
            ),
            (
                DocumentSpec::file(b"model_provider = \"hidden-provider\""),
                DocumentSpec::file(b"{}"),
                RelayConfigurationStatus::Incomplete,
            ),
            (
                DocumentSpec::file(
                    br#"
                        model_provider = "hidden-provider"
                        [model_providers.hidden-provider]
                        requires_openai_auth = true
                        base_url = "test-only"
                    "#,
                ),
                DocumentSpec::file(b"{"),
                RelayConfigurationStatus::NotObserved,
            ),
        ];

        for (config, auth, expected) in cases {
            let probe = MemoryProbe::new(auth, config);
            let observation = observe(&probe).expect("至少一份文档存在必须形成观察");
            assert_eq!(observation.relay_configuration(), expected);
        }
    }

    #[test]
    fn 损坏_json_toml_非对象根和非_utf8_保留_invalid() {
        let invalid_auth_documents = [b"{".as_slice(), b"[]".as_slice(), &[0xff, 0xfe]];
        for auth in invalid_auth_documents {
            let probe = MemoryProbe::new(DocumentSpec::file(auth), DocumentSpec::missing());
            let observation = observe(&probe).expect("损坏文档仍是可观察事实");
            assert_eq!(
                observation.auth_document_status(),
                RelayDocumentStatus::Invalid
            );
            assert_eq!(
                observation.chatgpt_credentials(),
                CredentialPresence::NotObserved
            );
        }

        for config in [b"[".as_slice(), &[0xff, 0xfe]] {
            let probe = MemoryProbe::new(DocumentSpec::missing(), DocumentSpec::file(config));
            let observation = observe(&probe).expect("损坏文档仍是可观察事实");
            assert_eq!(
                observation.config_document_status(),
                RelayDocumentStatus::Invalid
            );
            assert_eq!(
                observation.relay_configuration(),
                RelayConfigurationStatus::NotObserved
            );
        }
    }

    #[test]
    fn 元数据和读取竞态均执行单文件_256_kib_上限() {
        for auth in [
            DocumentSpec::metadata_too_large(),
            DocumentSpec::read_too_large(),
        ] {
            let probe = MemoryProbe::new(auth, DocumentSpec::missing());
            let observation = observe(&probe).expect("超限必须保留状态");
            assert_eq!(
                observation.auth_document_status(),
                RelayDocumentStatus::TooLarge
            );
            assert_eq!(
                observation.chatgpt_credentials(),
                CredentialPresence::NotObserved
            );
        }
    }

    #[test]
    fn 符号链接非普通文件和_io_失败保留_unreadable() {
        for auth in [
            DocumentSpec::kind(RelayStatusFileKind::Symlink),
            DocumentSpec::kind(RelayStatusFileKind::Other),
            DocumentSpec::metadata_error(io::ErrorKind::PermissionDenied),
            DocumentSpec::read_error(io::ErrorKind::PermissionDenied),
        ] {
            let probe = MemoryProbe::new(auth, DocumentSpec::missing());
            let observation = observe(&probe).expect("不可读文档仍是可观察事实");
            assert_eq!(
                observation.auth_document_status(),
                RelayDocumentStatus::Unreadable
            );
            assert_eq!(
                observation.openai_api_key(),
                CredentialPresence::NotObserved
            );
        }
    }

    #[test]
    fn 认证结构仅接受批准的非空字符串形态() {
        let cases = [
            (
                b"{}".as_slice(),
                CredentialPresence::Absent,
                CredentialPresence::Absent,
            ),
            (
                br#"{"auth_mode":"api","tokens":{"access_token":"test-only"}}"#.as_slice(),
                CredentialPresence::Absent,
                CredentialPresence::Absent,
            ),
            (
                br#"{"auth_mode":"CHATGPT","tokens":{"refresh_token":"  "},"OPENAI_API_KEY":1}"#
                    .as_slice(),
                CredentialPresence::Absent,
                CredentialPresence::Absent,
            ),
        ];

        for (auth, expected_chatgpt, expected_api_key) in cases {
            let probe = MemoryProbe::new(DocumentSpec::file(auth), DocumentSpec::missing());
            let observation = observe(&probe).expect("合法认证文档必须形成观察");
            assert_eq!(observation.chatgpt_credentials(), expected_chatgpt);
            assert_eq!(observation.openai_api_key(), expected_api_key);
        }
    }

    #[test]
    fn 只访问两个固定文件名并对实际读取传入固定上限() {
        let probe = MemoryProbe::new(DocumentSpec::file(b"{}"), DocumentSpec::file(b""));

        observe(&probe).expect("合法文档必须形成观察");

        let metadata_names = probe
            .metadata_paths
            .borrow()
            .iter()
            .map(|path| path.file_name().expect("必须有文件名").to_owned())
            .collect::<Vec<_>>();
        assert_eq!(metadata_names, ["auth.json", "config.toml"]);
        assert_eq!(
            probe
                .read_requests
                .borrow()
                .iter()
                .map(|(_, limit)| *limit)
                .collect::<Vec<_>>(),
            [RELAY_STATUS_FILE_LIMIT, RELAY_STATUS_FILE_LIMIT]
        );
    }

    #[test]
    fn 返回值_debug_不包含文档内容文件名或_provider() {
        let auth = br#"{
            "auth_mode":"chatgpt",
            "tokens":{"id_token":"private-test-value"},
            "OPENAI_API_KEY":"private-test-value"
        }"#;
        let config = br#"
            model_provider = "private-provider"
            [model_providers.private-provider]
            requires_openai_auth = true
            base_url = "private-endpoint"
        "#;
        let probe = MemoryProbe::new(DocumentSpec::file(auth), DocumentSpec::file(config));
        let observation = observe(&probe).expect("合法文档必须形成观察");
        let debug = format!("{observation:?}");

        for forbidden in [
            "private-test-value",
            "private-provider",
            "private-endpoint",
            "auth.json",
            "config.toml",
        ] {
            assert!(!debug.contains(forbidden), "观察结果泄漏: {forbidden}");
        }
    }
}
