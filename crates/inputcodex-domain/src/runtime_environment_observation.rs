use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnvironmentVariableNameError {
    InvalidPrefix,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EnvironmentVariableName(String);

impl EnvironmentVariableName {
    pub fn new(value: String) -> Result<Self, EnvironmentVariableNameError> {
        if value.starts_with("OPENAI_") {
            Ok(Self(value))
        } else {
            Err(EnvironmentVariableNameError::InvalidPrefix)
        }
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnvironmentValuePresence {
    Empty,
    NonEmpty,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnvironmentObservationStatus {
    Observed,
    NotObserved,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnvironmentConflictSource {
    RuntimeProcess,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EnvironmentSourceCoverage {
    runtime_process: EnvironmentObservationStatus,
    persistent_user: EnvironmentObservationStatus,
    persistent_system: EnvironmentObservationStatus,
}

impl EnvironmentSourceCoverage {
    #[must_use]
    pub const fn runtime_only() -> Self {
        Self {
            runtime_process: EnvironmentObservationStatus::Observed,
            persistent_user: EnvironmentObservationStatus::NotObserved,
            persistent_system: EnvironmentObservationStatus::NotObserved,
        }
    }

    #[must_use]
    pub const fn runtime_process(self) -> EnvironmentObservationStatus {
        self.runtime_process
    }

    #[must_use]
    pub const fn persistent_user(self) -> EnvironmentObservationStatus {
        self.persistent_user
    }

    #[must_use]
    pub const fn persistent_system(self) -> EnvironmentObservationStatus {
        self.persistent_system
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeEnvironmentConflict {
    name: EnvironmentVariableName,
    value_presence: EnvironmentValuePresence,
    source: EnvironmentConflictSource,
}

impl RuntimeEnvironmentConflict {
    #[must_use]
    pub const fn runtime_process(
        name: EnvironmentVariableName,
        value_presence: EnvironmentValuePresence,
    ) -> Self {
        Self {
            name,
            value_presence,
            source: EnvironmentConflictSource::RuntimeProcess,
        }
    }

    #[must_use]
    pub const fn name(&self) -> &EnvironmentVariableName {
        &self.name
    }

    #[must_use]
    pub const fn value_presence(&self) -> EnvironmentValuePresence {
        self.value_presence
    }

    #[must_use]
    pub const fn source(&self) -> EnvironmentConflictSource {
        self.source
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeEnvironmentConflictObservation {
    conflicts: Vec<RuntimeEnvironmentConflict>,
    coverage: EnvironmentSourceCoverage,
    scanned_entry_count: usize,
}

impl RuntimeEnvironmentConflictObservation {
    #[must_use]
    pub fn new(scanned_entry_count: usize, conflicts: Vec<RuntimeEnvironmentConflict>) -> Self {
        let mut merged = BTreeMap::new();
        for conflict in conflicts {
            merged
                .entry(conflict.name)
                .and_modify(|presence| {
                    if conflict.value_presence == EnvironmentValuePresence::NonEmpty {
                        *presence = EnvironmentValuePresence::NonEmpty;
                    }
                })
                .or_insert(conflict.value_presence);
        }

        Self {
            conflicts: merged
                .into_iter()
                .map(|(name, value_presence)| {
                    RuntimeEnvironmentConflict::runtime_process(name, value_presence)
                })
                .collect(),
            coverage: EnvironmentSourceCoverage::runtime_only(),
            scanned_entry_count,
        }
    }

    #[must_use]
    pub fn conflicts(&self) -> &[RuntimeEnvironmentConflict] {
        &self.conflicts
    }

    #[must_use]
    pub const fn coverage(&self) -> &EnvironmentSourceCoverage {
        &self.coverage
    }

    #[must_use]
    pub const fn scanned_entry_count(&self) -> usize {
        self.scanned_entry_count
    }

    #[must_use]
    pub fn conflict_count(&self) -> usize {
        self.conflicts.len()
    }
}
