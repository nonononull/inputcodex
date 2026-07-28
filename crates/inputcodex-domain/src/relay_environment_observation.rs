#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ProxyEnvironmentVariableName {
    HttpProxy,
    HttpsProxy,
    AllProxy,
    NoProxy,
    FtpProxy,
}

impl ProxyEnvironmentVariableName {
    #[must_use]
    pub fn from_name(value: &str) -> Option<Self> {
        match value.to_ascii_uppercase().as_str() {
            "HTTP_PROXY" => Some(Self::HttpProxy),
            "HTTPS_PROXY" => Some(Self::HttpsProxy),
            "ALL_PROXY" => Some(Self::AllProxy),
            "NO_PROXY" => Some(Self::NoProxy),
            "FTP_PROXY" => Some(Self::FtpProxy),
            _ => None,
        }
    }

    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HttpProxy => "HTTP_PROXY",
            Self::HttpsProxy => "HTTPS_PROXY",
            Self::AllProxy => "ALL_PROXY",
            Self::NoProxy => "NO_PROXY",
            Self::FtpProxy => "FTP_PROXY",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ProxyEnvironmentSource {
    RuntimeProcess,
    PersistentUser,
    PersistentSystem,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObservationCoverageStatus {
    Observed,
    NotObserved,
    Unavailable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProxyEnvironmentCoverage {
    runtime_process: ObservationCoverageStatus,
    persistent_user: ObservationCoverageStatus,
    persistent_system: ObservationCoverageStatus,
}

impl ProxyEnvironmentCoverage {
    #[must_use]
    pub const fn new(
        runtime_process: ObservationCoverageStatus,
        persistent_user: ObservationCoverageStatus,
        persistent_system: ObservationCoverageStatus,
    ) -> Self {
        Self {
            runtime_process,
            persistent_user,
            persistent_system,
        }
    }

    #[must_use]
    pub const fn runtime_process(self) -> ObservationCoverageStatus {
        self.runtime_process
    }

    #[must_use]
    pub const fn persistent_user(self) -> ObservationCoverageStatus {
        self.persistent_user
    }

    #[must_use]
    pub const fn persistent_system(self) -> ObservationCoverageStatus {
        self.persistent_system
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProxyEnvironmentVariableObservation {
    name: ProxyEnvironmentVariableName,
    sources: Vec<ProxyEnvironmentSource>,
}

impl ProxyEnvironmentVariableObservation {
    #[must_use]
    pub fn new(
        name: ProxyEnvironmentVariableName,
        mut sources: Vec<ProxyEnvironmentSource>,
    ) -> Self {
        sources.sort_unstable();
        sources.dedup();
        Self { name, sources }
    }

    #[must_use]
    pub const fn name(&self) -> ProxyEnvironmentVariableName {
        self.name
    }

    #[must_use]
    pub fn sources(&self) -> &[ProxyEnvironmentSource] {
        &self.sources
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CodexDotenvStatus {
    Absent,
    Present,
    Unavailable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClashConfigSource {
    PlatformData,
    PlatformConfig,
    HomeAppConfig,
    HomeLegacyConfig,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClashTunCandidateStatus {
    Absent,
    Disabled,
    Enabled,
    Unreadable,
    Invalid,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ClashTunObservation {
    platform_data: ClashTunCandidateStatus,
    platform_config: ClashTunCandidateStatus,
    home_app_config: ClashTunCandidateStatus,
    home_legacy_config: ClashTunCandidateStatus,
}

impl ClashTunObservation {
    #[must_use]
    pub const fn new(
        platform_data: ClashTunCandidateStatus,
        platform_config: ClashTunCandidateStatus,
        home_app_config: ClashTunCandidateStatus,
        home_legacy_config: ClashTunCandidateStatus,
    ) -> Self {
        Self {
            platform_data,
            platform_config,
            home_app_config,
            home_legacy_config,
        }
    }

    #[must_use]
    pub const fn status(self, source: ClashConfigSource) -> ClashTunCandidateStatus {
        match source {
            ClashConfigSource::PlatformData => self.platform_data,
            ClashConfigSource::PlatformConfig => self.platform_config,
            ClashConfigSource::HomeAppConfig => self.home_app_config,
            ClashConfigSource::HomeLegacyConfig => self.home_legacy_config,
        }
    }

    #[must_use]
    pub const fn has_enabled_tun(self) -> bool {
        matches!(self.platform_data, ClashTunCandidateStatus::Enabled)
            || matches!(self.platform_config, ClashTunCandidateStatus::Enabled)
            || matches!(self.home_app_config, ClashTunCandidateStatus::Enabled)
            || matches!(self.home_legacy_config, ClashTunCandidateStatus::Enabled)
    }

    #[must_use]
    pub const fn has_incomplete_candidate(self) -> bool {
        is_incomplete_candidate(self.platform_data)
            || is_incomplete_candidate(self.platform_config)
            || is_incomplete_candidate(self.home_app_config)
            || is_incomplete_candidate(self.home_legacy_config)
    }
}

const fn is_incomplete_candidate(status: ClashTunCandidateStatus) -> bool {
    matches!(
        status,
        ClashTunCandidateStatus::Unreadable | ClashTunCandidateStatus::Invalid
    )
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RelayEnvironmentObservation {
    proxy_variables: Vec<ProxyEnvironmentVariableObservation>,
    coverage: ProxyEnvironmentCoverage,
    codex_dotenv: CodexDotenvStatus,
    clash_tun: ClashTunObservation,
}

impl RelayEnvironmentObservation {
    #[must_use]
    pub fn new(
        proxy_variables: Vec<ProxyEnvironmentVariableObservation>,
        coverage: ProxyEnvironmentCoverage,
        codex_dotenv: CodexDotenvStatus,
        clash_tun: ClashTunObservation,
    ) -> Self {
        let mut merged = BTreeMap::<_, BTreeSet<_>>::new();
        for variable in proxy_variables {
            merged
                .entry(variable.name)
                .or_default()
                .extend(variable.sources);
        }
        let proxy_variables = merged
            .into_iter()
            .map(|(name, sources)| {
                ProxyEnvironmentVariableObservation::new(name, sources.into_iter().collect())
            })
            .collect();

        Self {
            proxy_variables,
            coverage,
            codex_dotenv,
            clash_tun,
        }
    }

    #[must_use]
    pub fn proxy_variables(&self) -> &[ProxyEnvironmentVariableObservation] {
        &self.proxy_variables
    }

    #[must_use]
    pub const fn coverage(&self) -> ProxyEnvironmentCoverage {
        self.coverage
    }

    #[must_use]
    pub const fn codex_dotenv(&self) -> CodexDotenvStatus {
        self.codex_dotenv
    }

    #[must_use]
    pub const fn clash_tun(&self) -> ClashTunObservation {
        self.clash_tun
    }

    #[must_use]
    pub fn has_detected_risk(&self) -> bool {
        !self.proxy_variables.is_empty()
            || matches!(self.codex_dotenv, CodexDotenvStatus::Present)
            || self.clash_tun.has_enabled_tun()
    }

    #[must_use]
    pub const fn has_observation_gap(&self) -> bool {
        !matches!(
            self.coverage.runtime_process,
            ObservationCoverageStatus::Observed
        ) || !matches!(
            self.coverage.persistent_user,
            ObservationCoverageStatus::Observed
        ) || !matches!(
            self.coverage.persistent_system,
            ObservationCoverageStatus::Observed
        ) || matches!(self.codex_dotenv, CodexDotenvStatus::Unavailable)
            || self.clash_tun.has_incomplete_candidate()
    }
}
use std::collections::{BTreeMap, BTreeSet};
