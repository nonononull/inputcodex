use std::{
    collections::{BTreeMap, HashMap, HashSet},
    ffi::{OsStr, OsString},
    fs::{self, File, Metadata},
    io::{self, Read},
    path::{Path, PathBuf},
    sync::{
        Arc,
        atomic::{AtomicU8, Ordering},
    },
    time::{Duration, Instant},
};

use inputcodex_application::{
    ApplicationError, ZedRemoteProjectObservationCancellation, ZedRemoteProjectObservationPort,
    ZedRemoteProjectObservationRequest,
};
use inputcodex_domain::{
    MAX_ZED_REMOTE_PROJECTS, ZedRemoteProjectEntry, ZedRemoteProjectId,
    ZedRemoteProjectObservation, ZedRemoteProjectOrigin, ZedRemoteProjectSelectionHint,
    ZedRemoteProjectSourceSummary,
};
use rusqlite::{Connection, ErrorCode, OpenFlags, OptionalExtension, params, types::ValueRef};
use serde_json::{Map, Value};
use sha2::{Digest, Sha256};

#[cfg(any(target_os = "windows", target_os = "macos"))]
use crate::resolve_codex_home_for_observation;

const GLOBAL_STATE_FILE: &str = ".codex-global-state.json";
const SQLITE_HOME_ENV: &str = "CODEX_SQLITE_HOME";
const SQLITE_DIRECTORY: &str = "sqlite";
const LEGACY_DATABASE: &str = "state_5.sqlite";
const ID_DOMAIN: &[u8] = b"inputcodex.zed-remote-project.v1\0";
const ID_PREFIX: &str = "zed-remote-project:v1:sha256:";

const INVALID_STATE_CODE: &str = "ZED_REMOTE_PROJECT_OBSERVATION_INVALID_STATE";
const INVALID_SQLITE_HOME_CODE: &str = "ZED_REMOTE_PROJECT_OBSERVATION_INVALID_SQLITE_HOME";
const UNSUPPORTED_SCHEMA_CODE: &str = "ZED_REMOTE_PROJECT_OBSERVATION_UNSUPPORTED_SCHEMA";
const UNAVAILABLE_CODE: &str = "ZED_REMOTE_PROJECT_OBSERVATION_UNAVAILABLE";
const RESOURCE_LIMIT_CODE: &str = "ZED_REMOTE_PROJECT_OBSERVATION_RESOURCE_LIMIT";
const TIMEOUT_CODE: &str = "ZED_REMOTE_PROJECT_OBSERVATION_TIMEOUT";
const CANCELLED_CODE: &str = "ZED_REMOTE_PROJECT_OBSERVATION_CANCELLED";
const COLLISION_CODE: &str = "ZED_REMOTE_PROJECT_ID_COLLISION";
#[cfg(not(any(target_os = "windows", target_os = "macos")))]
const UNSUPPORTED_CODE: &str = "ZED_REMOTE_PROJECT_OBSERVATION_UNSUPPORTED";

const DEFAULT_OBSERVATION_TIMEOUT: Duration = Duration::from_secs(2);
const DEFAULT_BUSY_TIMEOUT: Duration = Duration::from_millis(50);
const DEFAULT_PROGRESS_STEPS: i32 = 1_000;
const MAX_SQLITE_DATABASE_BYTES: u64 = 1024 * 1024 * 1024;
const MAX_SQLITE_DIRECTORY_ENTRIES: usize = 512;
const MAX_INTERNAL_CANDIDATES: usize = 512;
const INTERRUPTION_NONE: u8 = 0;
const INTERRUPTION_CANCELLED: u8 = 1;
const INTERRUPTION_TIMEOUT: u8 = 2;

pub(super) const MAX_GLOBAL_STATE_BYTES: usize = 1024 * 1024;
pub(super) const MAX_SQLITE_DATABASES: usize = 32;
pub(super) const MAX_SQLITE_CWDS_PER_DATABASE: usize = 80;
pub(super) const MAX_REMOTE_COMPONENT_BYTES: usize = 4 * 1024;

#[derive(Debug, Clone, Copy)]
pub(super) struct ZedRemoteProjectObservationPolicy {
    observation_timeout: Duration,
    busy_timeout: Duration,
    progress_steps: i32,
}

impl Default for ZedRemoteProjectObservationPolicy {
    fn default() -> Self {
        Self::new(
            DEFAULT_OBSERVATION_TIMEOUT,
            DEFAULT_BUSY_TIMEOUT,
            DEFAULT_PROGRESS_STEPS,
        )
    }
}

impl ZedRemoteProjectObservationPolicy {
    #[must_use]
    pub(super) const fn new(
        observation_timeout: Duration,
        busy_timeout: Duration,
        progress_steps: i32,
    ) -> Self {
        Self {
            observation_timeout,
            busy_timeout,
            progress_steps,
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct SystemZedRemoteProjectObservation;

impl ZedRemoteProjectObservationPort for SystemZedRemoteProjectObservation {
    fn observe(
        &self,
        request: &ZedRemoteProjectObservationRequest,
        cancellation: &ZedRemoteProjectObservationCancellation,
    ) -> Result<Option<ZedRemoteProjectObservation>, ApplicationError> {
        let _ = request;

        #[cfg(any(target_os = "windows", target_os = "macos"))]
        {
            let codex_home = resolve_codex_home_for_observation()?;
            let sqlite_root = resolve_sqlite_root(&codex_home, std::env::var_os(SQLITE_HOME_ENV))?;
            observe_zed_remote_projects_at_roots(
                &codex_home,
                &sqlite_root,
                cancellation,
                ZedRemoteProjectObservationPolicy::default(),
            )
        }

        #[cfg(not(any(target_os = "windows", target_os = "macos")))]
        {
            let _ = cancellation;
            Err(ApplicationError::unsupported(UNSUPPORTED_CODE))
        }
    }
}

pub(super) fn resolve_sqlite_root(
    codex_home: &Path,
    override_value: Option<OsString>,
) -> Result<PathBuf, ApplicationError> {
    let Some(value) = override_value else {
        return Ok(codex_home.to_path_buf());
    };
    if value.to_string_lossy().trim().is_empty() {
        return Ok(codex_home.to_path_buf());
    }

    let path = PathBuf::from(value);
    if !path.is_absolute() || validate_directory_tree(&path).is_err() {
        return Err(ApplicationError::unavailable(INVALID_SQLITE_HOME_CODE));
    }
    Ok(path)
}

pub(super) trait IdentityDigest {
    fn digest(&self, payload: &[u8]) -> [u8; 32];
}

#[derive(Debug, Clone, Copy)]
struct Sha256IdentityDigest;

impl IdentityDigest for Sha256IdentityDigest {
    fn digest(&self, payload: &[u8]) -> [u8; 32] {
        Sha256::digest(payload).into()
    }
}

#[cfg(test)]
#[allow(dead_code)]
pub(super) fn stable_project_id(
    user: &str,
    host: &str,
    port: Option<u16>,
    remote_path: &str,
) -> Result<ZedRemoteProjectId, ApplicationError> {
    let identity = RemoteIdentity::new(user, host, port, remote_path)?;
    stable_project_id_with_digest(&identity, &Sha256IdentityDigest)
}

fn stable_project_id_with_digest(
    identity: &RemoteIdentity,
    digest: &impl IdentityDigest,
) -> Result<ZedRemoteProjectId, ApplicationError> {
    const HEX: &[u8; 16] = b"0123456789abcdef";

    let payload = identity.digest_payload()?;
    let mut encoded = String::with_capacity(64);
    for byte in digest.digest(&payload) {
        encoded.push(char::from(HEX[usize::from(byte >> 4)]));
        encoded.push(char::from(HEX[usize::from(byte & 0x0f)]));
    }
    ZedRemoteProjectId::new(format!("{ID_PREFIX}{encoded}"))
        .map_err(|_| ApplicationError::internal(UNAVAILABLE_CODE))
}

pub(super) fn observe_zed_remote_projects_at_roots(
    codex_home: &Path,
    sqlite_root: &Path,
    cancellation: &ZedRemoteProjectObservationCancellation,
    policy: ZedRemoteProjectObservationPolicy,
) -> Result<Option<ZedRemoteProjectObservation>, ApplicationError> {
    observe_zed_remote_projects_at_roots_with_digest(
        codex_home,
        sqlite_root,
        cancellation,
        policy,
        &Sha256IdentityDigest,
    )
}

pub(super) fn observe_zed_remote_projects_at_roots_with_digest(
    codex_home: &Path,
    sqlite_root: &Path,
    cancellation: &ZedRemoteProjectObservationCancellation,
    policy: ZedRemoteProjectObservationPolicy,
    digest: &impl IdentityDigest,
) -> Result<Option<ZedRemoteProjectObservation>, ApplicationError> {
    let deadline = Instant::now()
        .checked_add(policy.observation_timeout)
        .unwrap_or_else(Instant::now);
    let interruption = InterruptionControl::new(cancellation.clone(), deadline);
    interruption.ensure_active()?;
    validate_directory_tree(codex_home).map_err(|_| unavailable())?;
    validate_directory_tree(sqlite_root).map_err(|_| unavailable())?;

    let state_path = codex_home.join(GLOBAL_STATE_FILE);
    let state = read_global_state(&state_path, &interruption)?;
    let mut discovered = usize::from(state.is_some());
    let mut readable = usize::from(state.is_some());
    let mut failed = 0_usize;
    let mut first_error = None;
    let parsed_state = state.as_ref().map(parse_global_state).transpose()?;
    let mut candidates = parsed_state
        .as_ref()
        .map(|state| state.candidates.clone())
        .unwrap_or_default();

    let discovery = discover_sqlite_databases(sqlite_root)?;
    discovered = discovered
        .checked_add(discovery.candidates.len())
        .ok_or_else(resource_limit)?;

    for path in &discovery.candidates {
        interruption.ensure_active()?;
        match read_sqlite_cwds(path, policy, &interruption) {
            Ok(cwds) => {
                readable += 1;
                if let Some(state) = parsed_state.as_ref() {
                    for cwd in cwds {
                        if let Some(candidate) = state.candidate_from_cwd(&cwd)? {
                            push_candidate(&mut candidates, candidate)?;
                        }
                    }
                }
            }
            Err(error)
                if matches!(
                    error.kind(),
                    inputcodex_application::ErrorKind::Cancelled
                        | inputcodex_application::ErrorKind::Timeout
                ) =>
            {
                return Err(error);
            }
            Err(error) => {
                failed += 1;
                first_error.get_or_insert(error);
            }
        }
    }
    discovery.ensure_unchanged()?;
    interruption.ensure_active()?;

    let projects = build_projects(candidates, digest)?;
    if projects.is_empty() {
        return if failed == 0 {
            Ok(None)
        } else {
            Err(first_error.unwrap_or_else(unavailable))
        };
    }

    let sources = ZedRemoteProjectSourceSummary::new(discovered, readable, failed)
        .map_err(|_| unavailable())?;
    ZedRemoteProjectObservation::new(projects, sources)
        .map(Some)
        .map_err(|_| resource_limit())
}

fn read_global_state(
    path: &Path,
    interruption: &InterruptionControl,
) -> Result<Option<Value>, ApplicationError> {
    interruption.ensure_active()?;
    let Some(bytes) = read_bounded_optional_file(path, MAX_GLOBAL_STATE_BYTES, interruption)?
    else {
        return Ok(None);
    };
    let state: Value = serde_json::from_slice(&bytes).map_err(|_| invalid_state())?;
    if !state.is_object() {
        return Err(invalid_state());
    }
    Ok(Some(state))
}

fn read_bounded_optional_file(
    path: &Path,
    max_bytes: usize,
    interruption: &InterruptionControl,
) -> Result<Option<Vec<u8>>, ApplicationError> {
    let parent = path.parent().ok_or_else(unavailable)?;
    validate_directory_tree(parent).map_err(|_| unavailable())?;
    let before = match fs::symlink_metadata(path) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(None),
        Err(_) => return Err(unavailable()),
    };
    if !before.is_file() || metadata_is_link_or_reparse(&before) {
        return Err(unavailable());
    }
    if before.len() > u64::try_from(max_bytes).unwrap_or(u64::MAX) {
        return Err(resource_limit());
    }

    let mut file = open_file_no_follow(path).map_err(|_| unavailable())?;
    let opened_before = file.metadata().map_err(|_| unavailable())?;
    if !opened_before.is_file() || !same_file_snapshot(&before, &opened_before) {
        return Err(unavailable());
    }
    let take_limit = u64::try_from(max_bytes)
        .unwrap_or(u64::MAX)
        .saturating_add(1);
    let mut bytes = Vec::with_capacity(max_bytes.min(64 * 1024));
    file.by_ref()
        .take(take_limit)
        .read_to_end(&mut bytes)
        .map_err(|_| unavailable())?;
    interruption.ensure_active()?;
    if bytes.len() > max_bytes {
        return Err(resource_limit());
    }

    let opened_after = file.metadata().map_err(|_| unavailable())?;
    validate_directory_tree(parent).map_err(|_| unavailable())?;
    let after = fs::symlink_metadata(path).map_err(|_| unavailable())?;
    if !after.is_file()
        || metadata_is_link_or_reparse(&after)
        || !same_file_snapshot(&before, &opened_after)
        || !same_file_snapshot(&opened_after, &after)
    {
        return Err(unavailable());
    }
    Ok(Some(bytes))
}

#[derive(Clone, PartialEq, Eq)]
struct RemoteIdentity {
    user: String,
    host: String,
    port: Option<u16>,
    remote_path: String,
}

impl RemoteIdentity {
    fn new(
        user: &str,
        host: &str,
        port: Option<u16>,
        remote_path: &str,
    ) -> Result<Self, ApplicationError> {
        let user = bounded_component(user, true)?;
        let host = bounded_component(host, false)?;
        let remote_path = bounded_component(remote_path, false)?;
        if port == Some(0)
            || host.chars().any(|character| {
                character.is_whitespace() || matches!(character, '/' | '?' | '#' | '@')
            })
            || !remote_path.starts_with('/')
        {
            return Err(invalid_state());
        }
        validate_bracketed_host(&host)?;
        Ok(Self {
            user,
            host,
            port,
            remote_path,
        })
    }

    fn digest_payload(&self) -> Result<Vec<u8>, ApplicationError> {
        let mut payload = Vec::with_capacity(
            ID_DOMAIN.len() + self.user.len() + self.host.len() + self.remote_path.len() + 32,
        );
        payload.extend_from_slice(ID_DOMAIN);
        append_length_prefixed(&mut payload, self.user.as_bytes())?;
        append_length_prefixed(&mut payload, self.host.as_bytes())?;
        match self.port {
            None => payload.push(0),
            Some(port) => {
                payload.push(1);
                payload.extend_from_slice(&port.to_be_bytes());
            }
        }
        append_length_prefixed(&mut payload, self.remote_path.as_bytes())?;
        Ok(payload)
    }
}

fn bounded_component(value: &str, allow_empty: bool) -> Result<String, ApplicationError> {
    let value = value.trim();
    if (!allow_empty && value.is_empty())
        || value.len() > MAX_REMOTE_COMPONENT_BYTES
        || value.chars().any(char::is_control)
    {
        return Err(if value.len() > MAX_REMOTE_COMPONENT_BYTES {
            resource_limit()
        } else {
            invalid_state()
        });
    }
    Ok(value.to_string())
}

fn validate_bracketed_host(host: &str) -> Result<(), ApplicationError> {
    if host.starts_with('[') || host.ends_with(']') {
        if !(host.starts_with('[') && host.ends_with(']')) {
            return Err(invalid_state());
        }
        host[1..host.len() - 1]
            .parse::<std::net::Ipv6Addr>()
            .map_err(|_| invalid_state())?;
    } else if host.contains('[') || host.contains(']') {
        return Err(invalid_state());
    }
    Ok(())
}

fn append_length_prefixed(target: &mut Vec<u8>, value: &[u8]) -> Result<(), ApplicationError> {
    let length = u64::try_from(value.len()).map_err(|_| resource_limit())?;
    target.extend_from_slice(&length.to_be_bytes());
    target.extend_from_slice(value);
    Ok(())
}

#[derive(Clone)]
struct ProjectCandidate {
    identity: RemoteIdentity,
    origin: ZedRemoteProjectOrigin,
    selection_hint: ZedRemoteProjectSelectionHint,
}

fn push_candidate(
    candidates: &mut Vec<ProjectCandidate>,
    candidate: ProjectCandidate,
) -> Result<(), ApplicationError> {
    if candidates.len() >= MAX_INTERNAL_CANDIDATES {
        return Err(resource_limit());
    }
    candidates.push(candidate);
    Ok(())
}

struct ManagedTarget {
    user: String,
    host: String,
    port: Option<u16>,
}

#[derive(Clone)]
struct RemoteProjectMapping {
    project_id: String,
    host_id: String,
    remote_path: String,
}

struct ParsedGlobalState {
    selected_host_id: Option<String>,
    targets: HashMap<String, ManagedTarget>,
    remote_projects: Vec<RemoteProjectMapping>,
    candidates: Vec<ProjectCandidate>,
}

impl ParsedGlobalState {
    fn host_for_path<'a>(&'a self, preferred: Option<&'a str>, path: &str) -> Option<&'a str> {
        preferred.filter(|value| !value.is_empty()).or_else(|| {
            self.remote_projects
                .iter()
                .find(|project| path_matches_project(path, &project.remote_path))
                .map(|project| project.host_id.as_str())
                .or(self.selected_host_id.as_deref())
        })
    }

    fn candidate(
        &self,
        host_id: &str,
        remote_path: &str,
        origin: ZedRemoteProjectOrigin,
    ) -> Result<ProjectCandidate, ApplicationError> {
        let target = self.targets.get(host_id).ok_or_else(invalid_state)?;
        Ok(ProjectCandidate {
            identity: RemoteIdentity::new(&target.user, &target.host, target.port, remote_path)?,
            origin,
            selection_hint: if self.selected_host_id.as_deref() == Some(host_id) {
                ZedRemoteProjectSelectionHint::SelectedHostHint
            } else {
                ZedRemoteProjectSelectionHint::NotObserved
            },
        })
    }

    fn candidate_from_cwd(&self, cwd: &str) -> Result<Option<ProjectCandidate>, ApplicationError> {
        let Some(host_id) = self.host_for_path(None, cwd) else {
            return Ok(None);
        };
        self.candidate(host_id, cwd, ZedRemoteProjectOrigin::SqliteThreadCwd)
            .map(Some)
    }
}

fn parse_global_state(value: &Value) -> Result<ParsedGlobalState, ApplicationError> {
    let object = value.as_object().ok_or_else(invalid_state)?;
    let selected_host_id = optional_string(object, "selected-remote-host-id")?;
    let targets = parse_targets(object)?;
    let remote_projects = parse_remote_projects(object)?;
    let remote_projects = order_remote_projects(object, remote_projects)?;
    let mut state = ParsedGlobalState {
        selected_host_id,
        targets,
        remote_projects,
        candidates: Vec::new(),
    };

    for project in state.remote_projects.clone() {
        let candidate = state.candidate(
            &project.host_id,
            &project.remote_path,
            ZedRemoteProjectOrigin::CodexRemoteProject,
        )?;
        push_candidate(&mut state.candidates, candidate)?;
    }

    if let Some(hints) = optional_object(object, "thread-workspace-root-hints")? {
        if hints.len() > MAX_INTERNAL_CANDIDATES {
            return Err(resource_limit());
        }
        for hint in hints.values() {
            let (preferred_host, remote_path) = parse_workspace_hint(hint)?;
            if !remote_path.starts_with('/') {
                continue;
            }
            let Some(host_id) = state.host_for_path(preferred_host.as_deref(), &remote_path) else {
                continue;
            };
            let candidate = state.candidate(
                host_id,
                &remote_path,
                ZedRemoteProjectOrigin::ThreadWorkspaceHint,
            )?;
            push_candidate(&mut state.candidates, candidate)?;
        }
    }
    Ok(state)
}

fn parse_targets(
    object: &Map<String, Value>,
) -> Result<HashMap<String, ManagedTarget>, ApplicationError> {
    let Some(connections) = optional_array(object, "codex-managed-remote-connections")? else {
        return Ok(HashMap::new());
    };
    if connections.len() > MAX_INTERNAL_CANDIDATES {
        return Err(resource_limit());
    }
    let mut targets = HashMap::with_capacity(connections.len());
    for connection in connections {
        let connection = connection.as_object().ok_or_else(invalid_state)?;
        let host_id = required_string(connection, "hostId")?;
        let ssh_host = first_string(connection, &["sshHost", "hostname"])?;
        let ssh_alias = first_string(connection, &["sshAlias", "alias"])?;
        let (authority_user, authority_host, authority_port) = split_ssh_authority(&ssh_host)?;
        let host = if authority_host.is_empty() {
            ssh_alias
        } else {
            authority_host
        };
        let user = first_string(connection, &["sshUser", "user"])?;
        let user = if user.is_empty() {
            authority_user
        } else {
            user
        };
        let port = parse_port(connection.get("sshPort"), authority_port)?;
        let identity = RemoteIdentity::new(&user, &host, port, "/")?;
        let target = ManagedTarget {
            user: identity.user,
            host: identity.host,
            port: identity.port,
        };
        if targets.insert(host_id, target).is_some() {
            return Err(invalid_state());
        }
    }
    Ok(targets)
}

fn parse_remote_projects(
    object: &Map<String, Value>,
) -> Result<Vec<RemoteProjectMapping>, ApplicationError> {
    let Some(projects) = optional_array(object, "remote-projects")? else {
        return Ok(Vec::new());
    };
    if projects.len() > MAX_INTERNAL_CANDIDATES {
        return Err(resource_limit());
    }
    projects
        .iter()
        .map(|project| {
            let project = project.as_object().ok_or_else(invalid_state)?;
            Ok(RemoteProjectMapping {
                project_id: required_string(project, "id")?,
                host_id: required_string(project, "hostId")?,
                remote_path: required_remote_path(project, "remotePath")?,
            })
        })
        .collect()
}

fn order_remote_projects(
    object: &Map<String, Value>,
    projects: Vec<RemoteProjectMapping>,
) -> Result<Vec<RemoteProjectMapping>, ApplicationError> {
    let Some(order) = optional_array(object, "project-order")? else {
        return Ok(projects);
    };
    let mut by_id = HashMap::with_capacity(projects.len());
    for project in projects {
        if by_id.insert(project.project_id.clone(), project).is_some() {
            return Err(invalid_state());
        }
    }
    let mut ordered = Vec::with_capacity(by_id.len());
    let mut seen = HashSet::new();
    for project_id in order {
        let project_id = json_string(project_id)?;
        if !seen.insert(project_id.clone()) {
            return Err(invalid_state());
        }
        if let Some(project) = by_id.remove(&project_id) {
            ordered.push(project);
        }
    }
    let mut remaining = by_id.into_values().collect::<Vec<_>>();
    remaining.sort_by(|left, right| left.project_id.cmp(&right.project_id));
    ordered.extend(remaining);
    Ok(ordered)
}

fn parse_workspace_hint(value: &Value) -> Result<(Option<String>, String), ApplicationError> {
    match value {
        Value::String(path) => Ok((None, bounded_component(path, false)?)),
        Value::Object(object) => {
            let host = first_optional_string(object, &["hostId", "remoteHostId"])?;
            let path = first_string(
                object,
                &[
                    "remotePath",
                    "remoteWorkspaceRoot",
                    "workspaceRoot",
                    "path",
                    "cwd",
                ],
            )?;
            if path.is_empty() {
                return Err(invalid_state());
            }
            Ok((host, bounded_component(&path, false)?))
        }
        _ => Err(invalid_state()),
    }
}

fn split_ssh_authority(value: &str) -> Result<(String, String, Option<u16>), ApplicationError> {
    let mut authority = value.trim();
    let mut user = "";
    if let Some(index) = authority.rfind('@') {
        user = &authority[..index];
        authority = &authority[index + 1..];
    }
    if authority.starts_with('[') {
        let close = authority.find(']').ok_or_else(invalid_state)?;
        let host = authority[..=close].to_string();
        let suffix = &authority[close + 1..];
        let port = if suffix.is_empty() {
            None
        } else {
            parse_port_string(suffix.strip_prefix(':').ok_or_else(invalid_state)?)?
        };
        return Ok((user.trim().to_string(), host, port));
    }
    if authority.matches(':').count() == 1 {
        let (host, port) = authority.rsplit_once(':').ok_or_else(invalid_state)?;
        if !port.is_empty() && port.bytes().all(|byte| byte.is_ascii_digit()) {
            return Ok((
                user.trim().to_string(),
                host.trim().to_string(),
                parse_port_string(port)?,
            ));
        }
    }
    Ok((user.trim().to_string(), authority.trim().to_string(), None))
}

fn parse_port(
    value: Option<&Value>,
    fallback: Option<u16>,
) -> Result<Option<u16>, ApplicationError> {
    match value {
        None | Some(Value::Null) => Ok(fallback),
        Some(Value::String(value)) if value.trim().is_empty() => Ok(fallback),
        Some(Value::String(value)) => parse_port_string(value.trim()),
        Some(Value::Number(value)) => value
            .as_u64()
            .and_then(|value| u16::try_from(value).ok())
            .filter(|value| *value > 0)
            .map(Some)
            .ok_or_else(invalid_state),
        _ => Err(invalid_state()),
    }
}

fn parse_port_string(value: &str) -> Result<Option<u16>, ApplicationError> {
    if value.is_empty() {
        return Ok(None);
    }
    if !value.bytes().all(|byte| byte.is_ascii_digit()) {
        return Err(invalid_state());
    }
    value
        .parse::<u16>()
        .ok()
        .filter(|value| *value > 0)
        .map(Some)
        .ok_or_else(invalid_state)
}

fn optional_array<'a>(
    object: &'a Map<String, Value>,
    key: &str,
) -> Result<Option<&'a Vec<Value>>, ApplicationError> {
    match object.get(key) {
        None | Some(Value::Null) => Ok(None),
        Some(Value::Array(values)) => Ok(Some(values)),
        _ => Err(invalid_state()),
    }
}

fn optional_object<'a>(
    object: &'a Map<String, Value>,
    key: &str,
) -> Result<Option<&'a Map<String, Value>>, ApplicationError> {
    match object.get(key) {
        None | Some(Value::Null) => Ok(None),
        Some(Value::Object(values)) => Ok(Some(values)),
        _ => Err(invalid_state()),
    }
}

fn optional_string(
    object: &Map<String, Value>,
    key: &str,
) -> Result<Option<String>, ApplicationError> {
    match object.get(key) {
        None | Some(Value::Null) => Ok(None),
        Some(Value::String(value)) if value.trim().is_empty() => Ok(None),
        Some(Value::String(value)) => bounded_component(value, false).map(Some),
        _ => Err(invalid_state()),
    }
}

fn required_string(object: &Map<String, Value>, key: &str) -> Result<String, ApplicationError> {
    optional_string(object, key)?.ok_or_else(invalid_state)
}

fn required_remote_path(
    object: &Map<String, Value>,
    key: &str,
) -> Result<String, ApplicationError> {
    let path = required_string(object, key)?;
    if !path.starts_with('/') {
        return Err(invalid_state());
    }
    Ok(path)
}

fn first_optional_string(
    object: &Map<String, Value>,
    keys: &[&str],
) -> Result<Option<String>, ApplicationError> {
    for key in keys {
        if let Some(value) = optional_string(object, key)? {
            return Ok(Some(value));
        }
    }
    Ok(None)
}

fn first_string(object: &Map<String, Value>, keys: &[&str]) -> Result<String, ApplicationError> {
    Ok(first_optional_string(object, keys)?.unwrap_or_default())
}

fn json_string(value: &Value) -> Result<String, ApplicationError> {
    let Value::String(value) = value else {
        return Err(invalid_state());
    };
    bounded_component(value, false)
}

fn path_matches_project(path: &str, project_path: &str) -> bool {
    let project_path = project_path.trim_end_matches('/');
    !project_path.is_empty()
        && (path == project_path
            || path
                .strip_prefix(project_path)
                .is_some_and(|suffix| suffix.starts_with('/')))
}

struct SqliteDiscovery {
    candidates: Vec<PathBuf>,
    directory: Option<PathBuf>,
    directory_snapshot: Option<Vec<OsString>>,
}

impl SqliteDiscovery {
    fn ensure_unchanged(&self) -> Result<(), ApplicationError> {
        if let (Some(directory), Some(expected)) = (&self.directory, &self.directory_snapshot)
            && &bounded_directory_names(directory)? != expected
        {
            return Err(unavailable());
        }
        Ok(())
    }
}

fn discover_sqlite_databases(root: &Path) -> Result<SqliteDiscovery, ApplicationError> {
    validate_directory_tree(root).map_err(|_| unavailable())?;
    let directory = root.join(SQLITE_DIRECTORY);
    let (mut candidates, directory_snapshot) = match fs::symlink_metadata(&directory) {
        Ok(metadata) => {
            if !metadata.is_dir() || metadata_is_link_or_reparse(&metadata) {
                return Err(unavailable());
            }
            validate_directory_tree(&directory).map_err(|_| unavailable())?;
            let names = bounded_directory_names(&directory)?;
            let mut candidates = Vec::new();
            for name in &names {
                let path = directory.join(name);
                if !is_sqlite_candidate(&path) {
                    continue;
                }
                let metadata = fs::symlink_metadata(&path).map_err(|_| unavailable())?;
                if !metadata.is_file() || metadata_is_link_or_reparse(&metadata) {
                    return Err(unavailable());
                }
                candidates.push(path);
            }
            (candidates, Some(names))
        }
        Err(error) if error.kind() == io::ErrorKind::NotFound => (Vec::new(), None),
        Err(_) => return Err(unavailable()),
    };

    let legacy = root.join(LEGACY_DATABASE);
    match fs::symlink_metadata(&legacy) {
        Ok(metadata) => {
            if !metadata.is_file() || metadata_is_link_or_reparse(&metadata) {
                return Err(unavailable());
            }
            candidates.push(legacy);
        }
        Err(error) if error.kind() == io::ErrorKind::NotFound => {}
        Err(_) => return Err(unavailable()),
    }
    candidates.sort();
    candidates.dedup();
    if candidates.len() > MAX_SQLITE_DATABASES {
        return Err(resource_limit());
    }
    Ok(SqliteDiscovery {
        candidates,
        directory: directory_snapshot.as_ref().map(|_| directory),
        directory_snapshot,
    })
}

fn bounded_directory_names(path: &Path) -> Result<Vec<OsString>, ApplicationError> {
    let mut names = Vec::new();
    for entry in fs::read_dir(path).map_err(|_| unavailable())? {
        if names.len() >= MAX_SQLITE_DIRECTORY_ENTRIES {
            return Err(resource_limit());
        }
        names.push(entry.map_err(|_| unavailable())?.file_name());
    }
    names.sort();
    Ok(names)
}

fn is_sqlite_candidate(path: &Path) -> bool {
    path.extension()
        .and_then(OsStr::to_str)
        .is_some_and(|value| {
            value.eq_ignore_ascii_case("db")
                || value.eq_ignore_ascii_case("sqlite")
                || value.eq_ignore_ascii_case("sqlite3")
        })
}

fn read_sqlite_cwds(
    path: &Path,
    policy: ZedRemoteProjectObservationPolicy,
    interruption: &InterruptionControl,
) -> Result<Vec<String>, ApplicationError> {
    interruption.ensure_active()?;
    let parent = path.parent().ok_or_else(unavailable)?;
    validate_directory_tree(parent).map_err(|_| unavailable())?;
    let before = fs::symlink_metadata(path).map_err(|_| unavailable())?;
    if !before.is_file()
        || metadata_is_link_or_reparse(&before)
        || before.len() > MAX_SQLITE_DATABASE_BYTES
    {
        return Err(if before.len() > MAX_SQLITE_DATABASE_BYTES {
            resource_limit()
        } else {
            unavailable()
        });
    }

    let flags = OpenFlags::SQLITE_OPEN_READ_ONLY
        | OpenFlags::SQLITE_OPEN_NO_MUTEX
        | OpenFlags::SQLITE_OPEN_NOFOLLOW;
    let connection = Connection::open_with_flags(path, flags)
        .map_err(|error| map_sqlite_error(&error, interruption, unavailable()))?;
    let after_open = fs::symlink_metadata(path).map_err(|_| unavailable())?;
    if !after_open.is_file() || !same_file_snapshot(&before, &after_open) {
        return Err(unavailable());
    }
    connection
        .busy_timeout(policy.busy_timeout)
        .map_err(|error| map_sqlite_error(&error, interruption, unavailable()))?;
    let progress = interruption.clone();
    connection
        .progress_handler(
            policy.progress_steps.max(1),
            Some(move || progress.should_interrupt()),
        )
        .map_err(|error| map_sqlite_error(&error, interruption, unavailable()))?;
    connection
        .pragma_update(None, "query_only", true)
        .map_err(|error| map_sqlite_error(&error, interruption, unavailable()))?;
    let query_only = connection
        .pragma_query_value(None, "query_only", |row| row.get::<_, i64>(0))
        .map_err(|error| map_sqlite_error(&error, interruption, unavailable()))?;
    if query_only != 1 {
        return Err(unavailable());
    }
    let has_cwd = connection
        .query_row(
            "SELECT 1 FROM pragma_table_info('threads') WHERE name = 'cwd' LIMIT 1",
            [],
            |_| Ok(()),
        )
        .optional()
        .map_err(|error| map_sqlite_error(&error, interruption, unavailable()))?
        .is_some();
    if !has_cwd {
        return Err(ApplicationError::unsupported(UNSUPPORTED_SCHEMA_CODE));
    }

    let max_bytes = i64::try_from(MAX_REMOTE_COMPONENT_BYTES).map_err(|_| resource_limit())?;
    let row_limit =
        i64::try_from(MAX_SQLITE_CWDS_PER_DATABASE + 1).map_err(|_| resource_limit())?;
    let mut statement = connection
        .prepare(
            "SELECT typeof(cwd), octet_length(cwd),\n\
                    CASE WHEN typeof(cwd) = 'text' AND octet_length(cwd) <= ?1\n\
                         THEN cwd ELSE NULL END\n\
             FROM threads\n\
             WHERE cwd IS NOT NULL AND cwd != ''\n\
             GROUP BY cwd\n\
             ORDER BY cwd COLLATE BINARY\n\
             LIMIT ?2",
        )
        .map_err(|error| map_sqlite_error(&error, interruption, unavailable()))?;
    let mut rows = statement
        .query(params![max_bytes, row_limit])
        .map_err(|error| map_sqlite_error(&error, interruption, unavailable()))?;
    let mut cwds = Vec::new();
    while let Some(row) = rows
        .next()
        .map_err(|error| map_sqlite_error(&error, interruption, unavailable()))?
    {
        interruption.ensure_active()?;
        if cwds.len() >= MAX_SQLITE_CWDS_PER_DATABASE {
            return Err(resource_limit());
        }
        let kind = row
            .get::<_, String>(0)
            .map_err(|error| map_sqlite_error(&error, interruption, unavailable()))?;
        let byte_count = row
            .get::<_, i64>(1)
            .map_err(|error| map_sqlite_error(&error, interruption, unavailable()))?;
        if kind != "text" || byte_count < 0 || byte_count > max_bytes {
            return Err(if byte_count > max_bytes {
                resource_limit()
            } else {
                unavailable()
            });
        }
        let value = match row
            .get_ref(2)
            .map_err(|error| map_sqlite_error(&error, interruption, unavailable()))?
        {
            ValueRef::Text(value) => std::str::from_utf8(value)
                .map_err(|_| unavailable())?
                .trim()
                .to_string(),
            _ => return Err(unavailable()),
        };
        if value.is_empty() {
            continue;
        }
        if value.len() > MAX_REMOTE_COMPONENT_BYTES {
            return Err(resource_limit());
        }
        if value.chars().any(char::is_control) {
            return Err(unavailable());
        }
        if value.starts_with('/') {
            cwds.push(value);
        }
    }
    drop(rows);
    drop(statement);
    drop(connection);

    validate_directory_tree(parent).map_err(|_| unavailable())?;
    let after = fs::symlink_metadata(path).map_err(|_| unavailable())?;
    if !after.is_file() || !same_file_snapshot(&before, &after) {
        return Err(unavailable());
    }
    Ok(cwds)
}

fn build_projects(
    candidates: Vec<ProjectCandidate>,
    digest: &impl IdentityDigest,
) -> Result<Vec<ZedRemoteProjectEntry>, ApplicationError> {
    struct StoredProject {
        identity: RemoteIdentity,
        origin: ZedRemoteProjectOrigin,
        selection_hint: ZedRemoteProjectSelectionHint,
    }

    let mut projects: BTreeMap<String, StoredProject> = BTreeMap::new();
    for candidate in candidates {
        let id = stable_project_id_with_digest(&candidate.identity, digest)?;
        if let Some(existing) = projects.get_mut(id.as_str()) {
            if existing.identity != candidate.identity {
                return Err(ApplicationError::internal(COLLISION_CODE));
            }
            if origin_priority(candidate.origin) < origin_priority(existing.origin) {
                existing.origin = candidate.origin;
            }
            if candidate.selection_hint == ZedRemoteProjectSelectionHint::SelectedHostHint {
                existing.selection_hint = ZedRemoteProjectSelectionHint::SelectedHostHint;
            }
            continue;
        }
        if projects.len() >= MAX_ZED_REMOTE_PROJECTS {
            return Err(resource_limit());
        }
        projects.insert(
            id.as_str().to_string(),
            StoredProject {
                identity: candidate.identity,
                origin: candidate.origin,
                selection_hint: candidate.selection_hint,
            },
        );
    }

    projects
        .into_iter()
        .map(|(id, stored)| {
            ZedRemoteProjectId::new(id)
                .map(|id| ZedRemoteProjectEntry::new(id, stored.origin, stored.selection_hint))
                .map_err(|_| ApplicationError::internal(UNAVAILABLE_CODE))
        })
        .collect()
}

const fn origin_priority(origin: ZedRemoteProjectOrigin) -> u8 {
    match origin {
        ZedRemoteProjectOrigin::CodexRemoteProject => 0,
        ZedRemoteProjectOrigin::ThreadWorkspaceHint => 1,
        ZedRemoteProjectOrigin::SqliteThreadCwd => 2,
    }
}

#[derive(Clone)]
struct InterruptionControl {
    cancellation: ZedRemoteProjectObservationCancellation,
    deadline: Instant,
    reason: Arc<AtomicU8>,
}

impl InterruptionControl {
    fn new(cancellation: ZedRemoteProjectObservationCancellation, deadline: Instant) -> Self {
        Self {
            cancellation,
            deadline,
            reason: Arc::new(AtomicU8::new(INTERRUPTION_NONE)),
        }
    }

    fn ensure_active(&self) -> Result<(), ApplicationError> {
        match self.current_error() {
            Some(error) => Err(error),
            None => Ok(()),
        }
    }

    fn should_interrupt(&self) -> bool {
        if self.cancellation.is_cancelled() {
            self.reason.store(INTERRUPTION_CANCELLED, Ordering::Release);
        } else if Instant::now() >= self.deadline {
            let _ = self.reason.compare_exchange(
                INTERRUPTION_NONE,
                INTERRUPTION_TIMEOUT,
                Ordering::AcqRel,
                Ordering::Acquire,
            );
        }
        self.reason.load(Ordering::Acquire) != INTERRUPTION_NONE
    }

    fn current_error(&self) -> Option<ApplicationError> {
        let _ = self.should_interrupt();
        match self.reason.load(Ordering::Acquire) {
            INTERRUPTION_CANCELLED => Some(ApplicationError::cancelled(CANCELLED_CODE)),
            INTERRUPTION_TIMEOUT => Some(ApplicationError::timeout(TIMEOUT_CODE)),
            _ => None,
        }
    }
}

fn map_sqlite_error(
    error: &rusqlite::Error,
    interruption: &InterruptionControl,
    fallback: ApplicationError,
) -> ApplicationError {
    if let Some(error) = interruption.current_error() {
        return error;
    }
    match error.sqlite_error_code() {
        Some(ErrorCode::OperationInterrupted) => interruption.current_error().unwrap_or(fallback),
        _ => fallback,
    }
}

fn validate_directory_tree(path: &Path) -> io::Result<()> {
    if !path.is_absolute() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "relative path"));
    }
    let mut components = path.ancestors().collect::<Vec<_>>();
    components.reverse();
    for component in components {
        let metadata = fs::symlink_metadata(component)?;
        if !metadata.is_dir() || metadata_is_link_or_reparse(&metadata) {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "linked path"));
        }
    }
    Ok(())
}

#[cfg(target_os = "windows")]
fn metadata_is_link_or_reparse(metadata: &Metadata) -> bool {
    use std::os::windows::fs::MetadataExt;

    const FILE_ATTRIBUTE_REPARSE_POINT: u32 = 0x0000_0400;
    metadata.file_type().is_symlink()
        || metadata.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT != 0
}

#[cfg(not(target_os = "windows"))]
fn metadata_is_link_or_reparse(metadata: &Metadata) -> bool {
    metadata.file_type().is_symlink()
}

#[cfg(target_os = "windows")]
fn open_file_no_follow(path: &Path) -> io::Result<File> {
    use std::os::windows::fs::OpenOptionsExt;

    const FILE_FLAG_OPEN_REPARSE_POINT: u32 = 0x0020_0000;
    File::options()
        .read(true)
        .custom_flags(FILE_FLAG_OPEN_REPARSE_POINT)
        .open(path)
}

#[cfg(target_os = "macos")]
fn open_file_no_follow(path: &Path) -> io::Result<File> {
    use std::os::unix::fs::OpenOptionsExt;

    const O_NOFOLLOW: i32 = 0x0000_0100;
    File::options()
        .read(true)
        .custom_flags(O_NOFOLLOW)
        .open(path)
}

#[cfg(all(unix, not(target_os = "macos")))]
fn open_file_no_follow(path: &Path) -> io::Result<File> {
    use std::os::unix::fs::OpenOptionsExt;

    const O_NOFOLLOW: i32 = 0x0002_0000;
    File::options()
        .read(true)
        .custom_flags(O_NOFOLLOW)
        .open(path)
}

#[cfg(not(any(unix, target_os = "windows")))]
fn open_file_no_follow(path: &Path) -> io::Result<File> {
    File::open(path)
}

fn same_file_snapshot(left: &Metadata, right: &Metadata) -> bool {
    same_file_identity(left, right)
        && left.len() == right.len()
        && left.modified().ok() == right.modified().ok()
}

#[cfg(target_os = "windows")]
fn same_file_identity(left: &Metadata, right: &Metadata) -> bool {
    use std::os::windows::fs::MetadataExt;

    left.file_attributes() == right.file_attributes()
        && left.creation_time() == right.creation_time()
        && left.file_size() == right.file_size()
}

#[cfg(unix)]
fn same_file_identity(left: &Metadata, right: &Metadata) -> bool {
    use std::os::unix::fs::MetadataExt;

    left.dev() == right.dev() && left.ino() == right.ino()
}

#[cfg(not(any(unix, target_os = "windows")))]
fn same_file_identity(left: &Metadata, right: &Metadata) -> bool {
    left.len() == right.len() && left.modified().ok() == right.modified().ok()
}

const fn invalid_state() -> ApplicationError {
    ApplicationError::invalid_input(INVALID_STATE_CODE)
}

const fn unavailable() -> ApplicationError {
    ApplicationError::unavailable(UNAVAILABLE_CODE)
}

const fn resource_limit() -> ApplicationError {
    ApplicationError::unavailable(RESOURCE_LIMIT_CODE)
}
