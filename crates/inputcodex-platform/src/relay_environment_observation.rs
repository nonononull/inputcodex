#[cfg(any(target_os = "windows", target_os = "macos", test))]
use std::{
    collections::BTreeMap,
    ffi::{OsStr, OsString},
    io,
    path::{Path, PathBuf},
};
#[cfg(any(target_os = "windows", target_os = "macos"))]
use std::{
    fs::{self, File},
    io::Read,
};

use inputcodex_application::{
    ApplicationError, RelayEnvironmentObservationPort, RelayEnvironmentObservationRequest,
};
#[cfg(any(target_os = "windows", target_os = "macos", test))]
use inputcodex_domain::{
    ClashTunCandidateStatus, ClashTunObservation, CodexDotenvStatus, ObservationCoverageStatus,
    ProxyEnvironmentCoverage, ProxyEnvironmentSource, ProxyEnvironmentVariableName,
    ProxyEnvironmentVariableObservation, RelayEnvironmentObservation,
};

#[cfg(any(target_os = "macos", test))]
mod macos;
#[cfg(any(target_os = "windows", test))]
mod windows;

#[cfg(any(target_os = "windows", target_os = "macos", test))]
const CLASH_APP_ID: &str = "io.github.clash-verge-rev.clash-verge-rev";
#[cfg(any(target_os = "windows", target_os = "macos", test))]
const CLASH_LEGACY_DIR: &str = "clash-verge-rev";
#[cfg(any(target_os = "windows", target_os = "macos", test))]
const CLASH_CONFIG_FILE: &str = "clash-verge.yaml";
#[cfg(any(target_os = "windows", target_os = "macos", test))]
const CLASH_TUN_KEY: &str = "enable_tun_mode";
#[cfg(any(target_os = "windows", target_os = "macos", test))]
const CLASH_CONFIG_LIMIT: usize = 64 * 1024;

#[cfg(any(target_os = "windows", target_os = "macos", test))]
pub(super) enum PersistentEnvironment {
    Observed(Vec<(OsString, OsString)>),
    #[cfg(any(target_os = "macos", test))]
    NotObserved,
    #[cfg(any(target_os = "windows", test))]
    Unavailable,
}

#[cfg(any(target_os = "windows", target_os = "macos", test))]
pub(super) struct RelayObservationInputs {
    pub(super) runtime_environment: Vec<(OsString, OsString)>,
    pub(super) persistent_user: PersistentEnvironment,
    pub(super) persistent_system: PersistentEnvironment,
    pub(super) codex_home: PathBuf,
    pub(super) clash_candidates: [PathBuf; 4],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg(any(target_os = "windows", target_os = "macos", test))]
pub(super) enum FileMetadata {
    File,
    Other,
}

#[cfg(any(target_os = "windows", target_os = "macos", test))]
pub(super) enum LimitedRead {
    Bytes(Vec<u8>),
    TooLarge,
}

#[cfg(any(target_os = "windows", target_os = "macos", test))]
pub(super) trait RelayFileProbe {
    fn metadata(&self, path: &Path) -> io::Result<FileMetadata>;
    fn read_limited(&self, path: &Path, limit: usize) -> io::Result<LimitedRead>;
}

#[derive(Debug, Clone, Copy, Default)]
#[cfg(any(target_os = "windows", target_os = "macos"))]
pub(super) struct SystemRelayFileProbe;

#[cfg(any(target_os = "windows", target_os = "macos"))]
impl RelayFileProbe for SystemRelayFileProbe {
    fn metadata(&self, path: &Path) -> io::Result<FileMetadata> {
        let metadata = fs::metadata(path)?;
        Ok(if metadata.is_file() {
            FileMetadata::File
        } else {
            FileMetadata::Other
        })
    }

    fn read_limited(&self, path: &Path, limit: usize) -> io::Result<LimitedRead> {
        let file = File::open(path)?;
        let mut bytes = Vec::with_capacity(limit.min(8 * 1024));
        file.take(limit as u64 + 1).read_to_end(&mut bytes)?;
        if bytes.len() > limit {
            Ok(LimitedRead::TooLarge)
        } else {
            Ok(LimitedRead::Bytes(bytes))
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct SystemRelayEnvironmentObservation;

impl RelayEnvironmentObservationPort for SystemRelayEnvironmentObservation {
    fn observe(
        &self,
        _request: &RelayEnvironmentObservationRequest,
    ) -> Result<inputcodex_domain::RelayEnvironmentObservation, ApplicationError> {
        #[cfg(target_os = "windows")]
        {
            windows::observe_system()
        }

        #[cfg(target_os = "macos")]
        {
            macos::observe_system()
        }

        #[cfg(not(any(target_os = "windows", target_os = "macos")))]
        {
            Err(ApplicationError::unsupported(
                "RELAY_ENVIRONMENT_OBSERVATION_UNSUPPORTED",
            ))
        }
    }
}

#[cfg(any(target_os = "windows", target_os = "macos", test))]
pub(super) fn observe_with_inputs(
    inputs: RelayObservationInputs,
    probe: &impl RelayFileProbe,
) -> Result<RelayEnvironmentObservation, ApplicationError> {
    let mut proxy_variables = proxy_variables_from_pairs(
        inputs.runtime_environment,
        ProxyEnvironmentSource::RuntimeProcess,
    )?;
    let persistent_user = append_persistent_environment(
        &mut proxy_variables,
        inputs.persistent_user,
        ProxyEnvironmentSource::PersistentUser,
    )?;
    let persistent_system = append_persistent_environment(
        &mut proxy_variables,
        inputs.persistent_system,
        ProxyEnvironmentSource::PersistentSystem,
    )?;

    Ok(RelayEnvironmentObservation::new(
        proxy_variables,
        ProxyEnvironmentCoverage::new(
            ObservationCoverageStatus::Observed,
            persistent_user,
            persistent_system,
        ),
        inspect_codex_dotenv(&inputs.codex_home, probe),
        inspect_clash_candidates(&inputs.clash_candidates, probe),
    ))
}

#[cfg(any(target_os = "windows", target_os = "macos", test))]
fn append_persistent_environment(
    proxy_variables: &mut Vec<ProxyEnvironmentVariableObservation>,
    environment: PersistentEnvironment,
    source: ProxyEnvironmentSource,
) -> Result<ObservationCoverageStatus, ApplicationError> {
    match environment {
        PersistentEnvironment::Observed(pairs) => {
            proxy_variables.extend(proxy_variables_from_pairs(pairs, source)?);
            Ok(ObservationCoverageStatus::Observed)
        }
        #[cfg(any(target_os = "macos", test))]
        PersistentEnvironment::NotObserved => Ok(ObservationCoverageStatus::NotObserved),
        #[cfg(any(target_os = "windows", test))]
        PersistentEnvironment::Unavailable => Ok(ObservationCoverageStatus::Unavailable),
    }
}

#[cfg(any(target_os = "windows", target_os = "macos", test))]
fn proxy_variables_from_pairs(
    pairs: impl IntoIterator<Item = (OsString, OsString)>,
    source: ProxyEnvironmentSource,
) -> Result<Vec<ProxyEnvironmentVariableObservation>, ApplicationError> {
    let mut observations = Vec::new();
    for (name, value) in pairs {
        if !contains_non_whitespace(&value) {
            continue;
        }
        let Some(name) = proxy_variable_name(&name)? else {
            continue;
        };
        observations.push(ProxyEnvironmentVariableObservation::new(name, vec![source]));
    }
    Ok(observations)
}

#[cfg(any(target_os = "windows", target_os = "macos", test))]
fn contains_non_whitespace(value: &OsStr) -> bool {
    value
        .as_encoded_bytes()
        .iter()
        .any(|byte| !byte.is_ascii_whitespace())
}

#[cfg(any(target_os = "windows", target_os = "macos", test))]
fn proxy_variable_name(
    name: &OsStr,
) -> Result<Option<ProxyEnvironmentVariableName>, ApplicationError> {
    let encoded = name.as_encoded_bytes();
    let is_candidate = [
        b"HTTP_PROXY".as_slice(),
        b"HTTPS_PROXY".as_slice(),
        b"ALL_PROXY".as_slice(),
        b"NO_PROXY".as_slice(),
        b"FTP_PROXY".as_slice(),
    ]
    .into_iter()
    .any(|candidate| encoded.eq_ignore_ascii_case(candidate));
    if !is_candidate {
        return Ok(None);
    }

    let name = name.to_str().ok_or_else(name_unrepresentable_error)?;
    ProxyEnvironmentVariableName::from_name(name)
        .map(Some)
        .ok_or_else(name_unrepresentable_error)
}

#[cfg(any(target_os = "windows", target_os = "macos", test))]
const fn name_unrepresentable_error() -> ApplicationError {
    ApplicationError::internal("RELAY_ENVIRONMENT_NAME_UNREPRESENTABLE")
}

#[cfg(any(target_os = "windows", target_os = "macos", test))]
pub(super) fn clash_candidate_paths(
    platform_data_dir: &Path,
    platform_config_dir: &Path,
    user_home: &Path,
) -> [PathBuf; 4] {
    [
        platform_data_dir.join(CLASH_APP_ID).join(CLASH_CONFIG_FILE),
        platform_config_dir
            .join(CLASH_APP_ID)
            .join(CLASH_CONFIG_FILE),
        user_home
            .join(".config")
            .join(CLASH_APP_ID)
            .join(CLASH_CONFIG_FILE),
        user_home
            .join(".config")
            .join(CLASH_LEGACY_DIR)
            .join(CLASH_CONFIG_FILE),
    ]
}

#[cfg(any(target_os = "windows", target_os = "macos", test))]
fn inspect_codex_dotenv(codex_home: &Path, probe: &impl RelayFileProbe) -> CodexDotenvStatus {
    match probe.metadata(&codex_home.join(".env")) {
        Ok(FileMetadata::File) => CodexDotenvStatus::Present,
        Ok(FileMetadata::Other) => CodexDotenvStatus::Absent,
        Err(error) if error.kind() == io::ErrorKind::NotFound => CodexDotenvStatus::Absent,
        Err(_) => CodexDotenvStatus::Unavailable,
    }
}

#[cfg(any(target_os = "windows", target_os = "macos", test))]
fn inspect_clash_candidates(
    candidates: &[PathBuf; 4],
    probe: &impl RelayFileProbe,
) -> ClashTunObservation {
    let mut observed = BTreeMap::new();
    let mut status = |path: &PathBuf| {
        *observed
            .entry(path.clone())
            .or_insert_with(|| inspect_clash_candidate(path, probe))
    };

    ClashTunObservation::new(
        status(&candidates[0]),
        status(&candidates[1]),
        status(&candidates[2]),
        status(&candidates[3]),
    )
}

#[cfg(any(target_os = "windows", target_os = "macos", test))]
fn inspect_clash_candidate(path: &Path, probe: &impl RelayFileProbe) -> ClashTunCandidateStatus {
    match probe.read_limited(path, CLASH_CONFIG_LIMIT) {
        Err(error) if error.kind() == io::ErrorKind::NotFound => ClashTunCandidateStatus::Absent,
        Err(_) => ClashTunCandidateStatus::Unreadable,
        Ok(LimitedRead::TooLarge) => ClashTunCandidateStatus::Invalid,
        Ok(LimitedRead::Bytes(bytes)) => std::str::from_utf8(&bytes)
            .ok()
            .and_then(parse_clash_tun)
            .map_or(ClashTunCandidateStatus::Invalid, |enabled| {
                if enabled {
                    ClashTunCandidateStatus::Enabled
                } else {
                    ClashTunCandidateStatus::Disabled
                }
            }),
    }
}

#[cfg(any(target_os = "windows", target_os = "macos", test))]
fn parse_clash_tun(contents: &str) -> Option<bool> {
    let mut result = None;
    for raw_line in contents.lines() {
        if raw_line.chars().next().is_some_and(char::is_whitespace) {
            continue;
        }
        let line = raw_line.split('#').next()?.trim();
        let Some((candidate, value)) = line.split_once(':') else {
            continue;
        };
        if candidate.trim() != CLASH_TUN_KEY || result.is_some() {
            if candidate.trim() == CLASH_TUN_KEY {
                return None;
            }
            continue;
        }
        result = match value
            .trim()
            .trim_matches(['\'', '"'])
            .to_ascii_lowercase()
            .as_str()
        {
            "true" => Some(true),
            "false" => Some(false),
            _ => return None,
        };
    }
    result
}

#[cfg(test)]
mod tests {
    use std::{
        cell::RefCell,
        collections::BTreeMap,
        ffi::OsString,
        io,
        path::{Path, PathBuf},
    };

    use inputcodex_domain::{
        ClashConfigSource, ClashTunCandidateStatus, CodexDotenvStatus, ObservationCoverageStatus,
        ProxyEnvironmentSource, ProxyEnvironmentVariableName,
    };

    use super::*;

    #[derive(Clone)]
    enum MetadataResult {
        File,
        Other,
        Error(io::ErrorKind),
    }

    #[derive(Clone)]
    enum ReadResult {
        Bytes(Vec<u8>),
        TooLarge,
        Error(io::ErrorKind),
    }

    #[derive(Default)]
    struct MemoryFileProbe {
        metadata: BTreeMap<PathBuf, MetadataResult>,
        reads: BTreeMap<PathBuf, ReadResult>,
        metadata_calls: RefCell<Vec<PathBuf>>,
        read_calls: RefCell<Vec<(PathBuf, usize)>>,
    }

    impl MemoryFileProbe {
        fn with_metadata(mut self, path: PathBuf, result: MetadataResult) -> Self {
            self.metadata.insert(path, result);
            self
        }

        fn with_read(mut self, path: PathBuf, result: ReadResult) -> Self {
            self.reads.insert(path, result);
            self
        }
    }

    impl RelayFileProbe for MemoryFileProbe {
        fn metadata(&self, path: &Path) -> io::Result<FileMetadata> {
            self.metadata_calls.borrow_mut().push(path.to_path_buf());
            match self.metadata.get(path) {
                Some(MetadataResult::File) => Ok(FileMetadata::File),
                Some(MetadataResult::Other) => Ok(FileMetadata::Other),
                Some(MetadataResult::Error(kind)) => Err(io::Error::from(*kind)),
                None => Err(io::Error::from(io::ErrorKind::NotFound)),
            }
        }

        fn read_limited(&self, path: &Path, limit: usize) -> io::Result<LimitedRead> {
            self.read_calls
                .borrow_mut()
                .push((path.to_path_buf(), limit));
            match self.reads.get(path) {
                Some(ReadResult::Bytes(bytes)) => Ok(LimitedRead::Bytes(bytes.clone())),
                Some(ReadResult::TooLarge) => Ok(LimitedRead::TooLarge),
                Some(ReadResult::Error(kind)) => Err(io::Error::from(*kind)),
                None => Err(io::Error::from(io::ErrorKind::NotFound)),
            }
        }
    }

    fn root(name: &str) -> PathBuf {
        std::env::temp_dir().join(name)
    }

    #[test]
    fn 代理名称双平台一致且只记录非空存在状态() {
        let sentinel = "http://private-proxy.invalid:7890";
        let inputs = RelayObservationInputs {
            runtime_environment: vec![
                (OsString::from("https_proxy"), OsString::from(sentinel)),
                (OsString::from("NO_PROXY"), OsString::from("   ")),
                (OsString::from("OPENAI_API_KEY"), OsString::from("secret")),
            ],
            persistent_user: PersistentEnvironment::Observed(vec![(
                OsString::from("HTTPS_PROXY"),
                OsString::from("user-secret"),
            )]),
            persistent_system: PersistentEnvironment::NotObserved,
            codex_home: root("inputcodex-relay-codex"),
            clash_candidates: clash_candidate_paths(
                &root("inputcodex-relay-data"),
                &root("inputcodex-relay-config"),
                &root("inputcodex-relay-home"),
            ),
        };

        let observation =
            observe_with_inputs(inputs, &MemoryFileProbe::default()).expect("共享观察应成功");

        assert_eq!(observation.proxy_variables().len(), 1);
        assert_eq!(
            observation.proxy_variables()[0].name(),
            ProxyEnvironmentVariableName::HttpsProxy
        );
        assert_eq!(
            observation.proxy_variables()[0].sources(),
            &[
                ProxyEnvironmentSource::RuntimeProcess,
                ProxyEnvironmentSource::PersistentUser,
            ]
        );
        assert_eq!(
            observation.coverage().runtime_process(),
            ObservationCoverageStatus::Observed
        );
        assert_eq!(
            observation.coverage().persistent_user(),
            ObservationCoverageStatus::Observed
        );
        assert_eq!(
            observation.coverage().persistent_system(),
            ObservationCoverageStatus::NotObserved
        );
        assert!(!format!("{observation:?}").contains(sentinel));
        assert!(!format!("{observation:?}").contains("user-secret"));
    }

    #[test]
    fn codex_dotenv_只查元数据且不读取内容() {
        let codex_home = root("inputcodex-dotenv-home");
        let dotenv = codex_home.join(".env");
        let probe = MemoryFileProbe::default()
            .with_metadata(dotenv.clone(), MetadataResult::File)
            .with_read(dotenv.clone(), ReadResult::Bytes(b"SECRET=value".to_vec()));

        assert_eq!(
            inspect_codex_dotenv(&codex_home, &probe),
            CodexDotenvStatus::Present
        );
        assert_eq!(probe.metadata_calls.borrow().as_slice(), &[dotenv]);
        assert!(probe.read_calls.borrow().is_empty());
    }

    #[test]
    fn clash_读取状态区分缺失禁用启用不可读与非法() {
        let path = root("inputcodex-clash-status.yaml");
        let cases = [
            (
                ReadResult::Error(io::ErrorKind::NotFound),
                ClashTunCandidateStatus::Absent,
            ),
            (
                ReadResult::Bytes(b"enable_tun_mode: false\n".to_vec()),
                ClashTunCandidateStatus::Disabled,
            ),
            (
                ReadResult::Bytes(b"enable_tun_mode: true\n".to_vec()),
                ClashTunCandidateStatus::Enabled,
            ),
            (
                ReadResult::Error(io::ErrorKind::PermissionDenied),
                ClashTunCandidateStatus::Unreadable,
            ),
            (ReadResult::TooLarge, ClashTunCandidateStatus::Invalid),
            (
                ReadResult::Bytes(vec![0xff, 0xfe]),
                ClashTunCandidateStatus::Invalid,
            ),
            (
                ReadResult::Bytes(b"  enable_tun_mode: true\n".to_vec()),
                ClashTunCandidateStatus::Invalid,
            ),
            (
                ReadResult::Bytes(b"enable_tun_mode: maybe\n".to_vec()),
                ClashTunCandidateStatus::Invalid,
            ),
        ];

        for (input, expected) in cases {
            let probe = MemoryFileProbe::default().with_read(path.clone(), input);
            assert_eq!(inspect_clash_candidate(&path, &probe), expected);
            assert_eq!(probe.read_calls.borrow()[0].1, CLASH_CONFIG_LIMIT);
        }
    }

    #[test]
    fn clash_重复候选只读取一次并映射回固定逻辑来源() {
        let shared = root("inputcodex-clash-shared.yaml");
        let home_app = root("inputcodex-clash-home-app.yaml");
        let home_legacy = root("inputcodex-clash-home-legacy.yaml");
        let candidates = [shared.clone(), shared.clone(), home_app, home_legacy];
        let probe = MemoryFileProbe::default().with_read(
            shared.clone(),
            ReadResult::Bytes(b"enable_tun_mode: true\n".to_vec()),
        );

        let observation = inspect_clash_candidates(&candidates, &probe);

        assert_eq!(
            observation.status(ClashConfigSource::PlatformData),
            ClashTunCandidateStatus::Enabled
        );
        assert_eq!(
            observation.status(ClashConfigSource::PlatformConfig),
            ClashTunCandidateStatus::Enabled
        );
        assert_eq!(
            probe
                .read_calls
                .borrow()
                .iter()
                .filter(|(path, _)| path == &shared)
                .count(),
            1
        );
    }

    #[test]
    fn dotenv_其他元数据与错误不会伪装成存在() {
        let codex_home = root("inputcodex-dotenv-status-home");
        let dotenv = codex_home.join(".env");

        for (result, expected) in [
            (MetadataResult::Other, CodexDotenvStatus::Absent),
            (
                MetadataResult::Error(io::ErrorKind::NotFound),
                CodexDotenvStatus::Absent,
            ),
            (
                MetadataResult::Error(io::ErrorKind::PermissionDenied),
                CodexDotenvStatus::Unavailable,
            ),
        ] {
            let probe = MemoryFileProbe::default().with_metadata(dotenv.clone(), result.clone());
            assert_eq!(inspect_codex_dotenv(&codex_home, &probe), expected);
        }
    }
}
