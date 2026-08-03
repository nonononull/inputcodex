use crate::{DiagnosticCode, WatcherPreference};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WatcherPreferenceMutationId(u64);

impl WatcherPreferenceMutationId {
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    #[must_use]
    pub const fn value(self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WatcherPreferenceSetupCommit {
    NotRequired,
    RootCreated,
    Failed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WatcherPreferenceMarkerCommit {
    NotAttempted,
    Created,
    Removed,
    Failed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WatcherPreferenceFinalObservation {
    Known(WatcherPreference),
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WatcherPreferenceMutationOutcome {
    Applied,
    AlreadySatisfied,
    Conflict,
    Cancelled,
    Failed,
    Indeterminate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WatcherPreferenceMutationReceipt {
    request_id: WatcherPreferenceMutationId,
    requested_preference: WatcherPreference,
    setup_commit: WatcherPreferenceSetupCommit,
    marker_commit: WatcherPreferenceMarkerCommit,
    final_observation: WatcherPreferenceFinalObservation,
    outcome: WatcherPreferenceMutationOutcome,
    diagnostic_code: DiagnosticCode,
}

impl WatcherPreferenceMutationReceipt {
    #[must_use]
    pub const fn new(
        request_id: WatcherPreferenceMutationId,
        requested_preference: WatcherPreference,
        setup_commit: WatcherPreferenceSetupCommit,
        marker_commit: WatcherPreferenceMarkerCommit,
        final_observation: WatcherPreferenceFinalObservation,
        outcome: WatcherPreferenceMutationOutcome,
        diagnostic_code: DiagnosticCode,
    ) -> Self {
        Self {
            request_id,
            requested_preference,
            setup_commit,
            marker_commit,
            final_observation,
            outcome,
            diagnostic_code,
        }
    }

    #[must_use]
    pub const fn request_id(self) -> WatcherPreferenceMutationId {
        self.request_id
    }

    #[must_use]
    pub const fn requested_preference(self) -> WatcherPreference {
        self.requested_preference
    }

    #[must_use]
    pub const fn setup_commit(self) -> WatcherPreferenceSetupCommit {
        self.setup_commit
    }

    #[must_use]
    pub const fn marker_commit(self) -> WatcherPreferenceMarkerCommit {
        self.marker_commit
    }

    #[must_use]
    pub const fn final_observation(self) -> WatcherPreferenceFinalObservation {
        self.final_observation
    }

    #[must_use]
    pub const fn outcome(self) -> WatcherPreferenceMutationOutcome {
        self.outcome
    }

    #[must_use]
    pub const fn diagnostic_code(self) -> DiagnosticCode {
        self.diagnostic_code
    }
}
