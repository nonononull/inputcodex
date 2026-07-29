#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ContextEntryKind {
    McpServer,
    Skill,
    Plugin,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContextEntryObservationError {
    EmptyId,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContextEntryObservation {
    id: String,
    kind: ContextEntryKind,
    enabled: bool,
}

impl ContextEntryObservation {
    pub fn new(
        id: String,
        kind: ContextEntryKind,
        enabled: bool,
    ) -> Result<Self, ContextEntryObservationError> {
        if id.trim().is_empty() {
            return Err(ContextEntryObservationError::EmptyId);
        }

        Ok(Self { id, kind, enabled })
    }

    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }

    #[must_use]
    pub const fn kind(&self) -> ContextEntryKind {
        self.kind
    }

    #[must_use]
    pub const fn is_enabled(&self) -> bool {
        self.enabled
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ContextEntryCategorySummary {
    total: usize,
    enabled: usize,
    disabled: usize,
}

impl ContextEntryCategorySummary {
    const fn empty() -> Self {
        Self {
            total: 0,
            enabled: 0,
            disabled: 0,
        }
    }

    fn record(&mut self, enabled: bool) {
        self.total += 1;
        if enabled {
            self.enabled += 1;
        } else {
            self.disabled += 1;
        }
    }

    #[must_use]
    pub const fn total(self) -> usize {
        self.total
    }

    #[must_use]
    pub const fn enabled(self) -> usize {
        self.enabled
    }

    #[must_use]
    pub const fn disabled(self) -> usize {
        self.disabled
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContextEntryCatalogObservation {
    entries: Vec<ContextEntryObservation>,
    mcp_servers: ContextEntryCategorySummary,
    skills: ContextEntryCategorySummary,
    plugins: ContextEntryCategorySummary,
}

impl ContextEntryCatalogObservation {
    #[must_use]
    pub fn new(entries: Vec<ContextEntryObservation>) -> Self {
        let mut observation = Self {
            entries,
            mcp_servers: ContextEntryCategorySummary::empty(),
            skills: ContextEntryCategorySummary::empty(),
            plugins: ContextEntryCategorySummary::empty(),
        };

        for entry in &observation.entries {
            match entry.kind() {
                ContextEntryKind::McpServer => observation.mcp_servers.record(entry.is_enabled()),
                ContextEntryKind::Skill => observation.skills.record(entry.is_enabled()),
                ContextEntryKind::Plugin => observation.plugins.record(entry.is_enabled()),
            }
        }

        observation
    }

    #[must_use]
    pub fn entries(&self) -> &[ContextEntryObservation] {
        &self.entries
    }

    #[must_use]
    pub const fn summary(&self, kind: ContextEntryKind) -> ContextEntryCategorySummary {
        match kind {
            ContextEntryKind::McpServer => self.mcp_servers,
            ContextEntryKind::Skill => self.skills,
            ContextEntryKind::Plugin => self.plugins,
        }
    }
}
