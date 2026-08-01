use std::{collections::HashSet, fmt};

const ZED_REMOTE_PROJECT_ID_PREFIX: &str = "zed-remote-project:v1:sha256:";
const SHA256_HEX_BYTES: usize = 64;

pub const MAX_ZED_REMOTE_PROJECTS: usize = 256;
pub const MAX_ZED_REMOTE_PROJECT_SOURCES: usize = 33;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ZedRemoteProjectIdError {
    InvalidFormat,
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ZedRemoteProjectId(String);

impl ZedRemoteProjectId {
    pub fn new(value: String) -> Result<Self, ZedRemoteProjectIdError> {
        let Some(digest) = value.strip_prefix(ZED_REMOTE_PROJECT_ID_PREFIX) else {
            return Err(ZedRemoteProjectIdError::InvalidFormat);
        };
        if digest.len() != SHA256_HEX_BYTES
            || !digest
                .bytes()
                .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
        {
            return Err(ZedRemoteProjectIdError::InvalidFormat);
        }

        Ok(Self(value))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for ZedRemoteProjectId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ZedRemoteProjectId")
            .finish_non_exhaustive()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ZedRemoteProjectOrigin {
    CodexRemoteProject,
    ThreadWorkspaceHint,
    SqliteThreadCwd,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ZedRemoteProjectSelectionHint {
    SelectedHostHint,
    NotObserved,
}

#[derive(Clone, PartialEq, Eq)]
pub struct ZedRemoteProjectEntry {
    id: ZedRemoteProjectId,
    origin: ZedRemoteProjectOrigin,
    selection_hint: ZedRemoteProjectSelectionHint,
}

impl ZedRemoteProjectEntry {
    #[must_use]
    pub const fn new(
        id: ZedRemoteProjectId,
        origin: ZedRemoteProjectOrigin,
        selection_hint: ZedRemoteProjectSelectionHint,
    ) -> Self {
        Self {
            id,
            origin,
            selection_hint,
        }
    }

    #[must_use]
    pub const fn id(&self) -> &ZedRemoteProjectId {
        &self.id
    }

    #[must_use]
    pub const fn origin(&self) -> ZedRemoteProjectOrigin {
        self.origin
    }

    #[must_use]
    pub const fn selection_hint(&self) -> ZedRemoteProjectSelectionHint {
        self.selection_hint
    }
}

impl fmt::Debug for ZedRemoteProjectEntry {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ZedRemoteProjectEntry")
            .field("id", &"<redacted>")
            .field("origin", &self.origin)
            .field("selection_hint", &self.selection_hint)
            .finish()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ZedRemoteProjectSourceCoverage {
    Complete,
    Partial,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ZedRemoteProjectSourceSummaryError {
    NoSources,
    TooManySources,
    CountMismatch,
    NoReadableSources,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ZedRemoteProjectSourceSummary {
    discovered: usize,
    readable: usize,
    failed: usize,
}

impl ZedRemoteProjectSourceSummary {
    pub fn new(
        discovered: usize,
        readable: usize,
        failed: usize,
    ) -> Result<Self, ZedRemoteProjectSourceSummaryError> {
        if discovered == 0 {
            return Err(ZedRemoteProjectSourceSummaryError::NoSources);
        }
        if discovered > MAX_ZED_REMOTE_PROJECT_SOURCES {
            return Err(ZedRemoteProjectSourceSummaryError::TooManySources);
        }
        if readable.checked_add(failed) != Some(discovered) {
            return Err(ZedRemoteProjectSourceSummaryError::CountMismatch);
        }
        if readable == 0 {
            return Err(ZedRemoteProjectSourceSummaryError::NoReadableSources);
        }

        Ok(Self {
            discovered,
            readable,
            failed,
        })
    }

    #[must_use]
    pub const fn discovered(self) -> usize {
        self.discovered
    }

    #[must_use]
    pub const fn readable(self) -> usize {
        self.readable
    }

    #[must_use]
    pub const fn failed(self) -> usize {
        self.failed
    }

    #[must_use]
    pub const fn coverage(self) -> ZedRemoteProjectSourceCoverage {
        if self.failed == 0 {
            ZedRemoteProjectSourceCoverage::Complete
        } else {
            ZedRemoteProjectSourceCoverage::Partial
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ZedRemoteProjectObservationError {
    EmptyProjects,
    TooManyProjects,
    DuplicateProjectId,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ZedRemoteProjectObservation {
    projects: Vec<ZedRemoteProjectEntry>,
    sources: ZedRemoteProjectSourceSummary,
}

impl ZedRemoteProjectObservation {
    pub fn new(
        projects: Vec<ZedRemoteProjectEntry>,
        sources: ZedRemoteProjectSourceSummary,
    ) -> Result<Self, ZedRemoteProjectObservationError> {
        if projects.is_empty() {
            return Err(ZedRemoteProjectObservationError::EmptyProjects);
        }
        if projects.len() > MAX_ZED_REMOTE_PROJECTS {
            return Err(ZedRemoteProjectObservationError::TooManyProjects);
        }

        let mut ids = HashSet::with_capacity(projects.len());
        if projects.iter().any(|project| !ids.insert(project.id())) {
            return Err(ZedRemoteProjectObservationError::DuplicateProjectId);
        }

        Ok(Self { projects, sources })
    }

    #[must_use]
    pub fn projects(&self) -> &[ZedRemoteProjectEntry] {
        &self.projects
    }

    #[must_use]
    pub fn project_count(&self) -> usize {
        self.projects.len()
    }

    #[must_use]
    pub const fn sources(&self) -> ZedRemoteProjectSourceSummary {
        self.sources
    }
}
