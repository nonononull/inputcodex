use std::{
    fmt,
    sync::{
        Arc,
        atomic::{AtomicU8, Ordering},
    },
};

use inputcodex_domain::{
    DiagnosticCode, WatcherPreference, WatcherPreferenceFinalObservation,
    WatcherPreferenceMarkerCommit, WatcherPreferenceMutationId, WatcherPreferenceMutationOutcome,
    WatcherPreferenceMutationReceipt, WatcherPreferenceSetupCommit,
};

const PHASE_PENDING: u8 = 0;
const PHASE_RUNNING: u8 = 1;
const PHASE_CANCELLED: u8 = 2;
const PHASE_CANCELLATION_ACCEPTED: u8 = 3;
const PHASE_COMMIT_REACHED: u8 = 4;
const PHASE_FINISHED: u8 = 5;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WatcherPreferenceMutationRequest {
    request_id: WatcherPreferenceMutationId,
    expected: WatcherPreference,
    desired: WatcherPreference,
}

impl WatcherPreferenceMutationRequest {
    #[must_use]
    pub const fn new(
        request_id: WatcherPreferenceMutationId,
        expected: WatcherPreference,
        desired: WatcherPreference,
    ) -> Self {
        Self {
            request_id,
            expected,
            desired,
        }
    }

    #[must_use]
    pub const fn request_id(self) -> WatcherPreferenceMutationId {
        self.request_id
    }

    #[must_use]
    pub const fn expected(self) -> WatcherPreference {
        self.expected
    }

    #[must_use]
    pub const fn desired(self) -> WatcherPreference {
        self.desired
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WatcherPreferenceMutationPhase {
    Pending,
    Cancelled,
    CommitReached,
    Finished,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WatcherPreferenceCancellationOutcome {
    Accepted,
    TooLate,
    Finished,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum WatcherPreferenceExecutionClaim {
    Claimed,
    Cancelled,
    InUse,
    Finished,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum WatcherPreferenceFinishOutcome {
    Completed,
    CancellationWon,
}

#[derive(Clone)]
pub struct WatcherPreferenceMutationControl {
    phase: Arc<AtomicU8>,
}

impl Default for WatcherPreferenceMutationControl {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for WatcherPreferenceMutationControl {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("WatcherPreferenceMutationControl")
            .field("phase", &self.phase())
            .finish()
    }
}

impl WatcherPreferenceMutationControl {
    #[must_use]
    pub fn new() -> Self {
        Self {
            phase: Arc::new(AtomicU8::new(PHASE_PENDING)),
        }
    }

    #[must_use]
    pub fn phase(&self) -> WatcherPreferenceMutationPhase {
        match self.phase.load(Ordering::Acquire) {
            PHASE_PENDING | PHASE_RUNNING => WatcherPreferenceMutationPhase::Pending,
            PHASE_CANCELLED | PHASE_CANCELLATION_ACCEPTED => {
                WatcherPreferenceMutationPhase::Cancelled
            }
            PHASE_COMMIT_REACHED => WatcherPreferenceMutationPhase::CommitReached,
            PHASE_FINISHED => WatcherPreferenceMutationPhase::Finished,
            _ => unreachable!("mutation phase 只由本类型写入"),
        }
    }

    pub fn cancel(&self) -> WatcherPreferenceCancellationOutcome {
        loop {
            match self.phase.load(Ordering::Acquire) {
                PHASE_PENDING => {
                    if self
                        .phase
                        .compare_exchange_weak(
                            PHASE_PENDING,
                            PHASE_CANCELLED,
                            Ordering::AcqRel,
                            Ordering::Acquire,
                        )
                        .is_ok()
                    {
                        return WatcherPreferenceCancellationOutcome::Accepted;
                    }
                }
                PHASE_RUNNING => {
                    if self
                        .phase
                        .compare_exchange_weak(
                            PHASE_RUNNING,
                            PHASE_CANCELLATION_ACCEPTED,
                            Ordering::AcqRel,
                            Ordering::Acquire,
                        )
                        .is_ok()
                    {
                        return WatcherPreferenceCancellationOutcome::Accepted;
                    }
                }
                PHASE_CANCELLED | PHASE_CANCELLATION_ACCEPTED => {
                    return WatcherPreferenceCancellationOutcome::Accepted;
                }
                PHASE_COMMIT_REACHED => return WatcherPreferenceCancellationOutcome::TooLate,
                PHASE_FINISHED => return WatcherPreferenceCancellationOutcome::Finished,
                _ => unreachable!("mutation phase 只由本类型写入"),
            }
        }
    }

    #[must_use]
    pub fn try_reach_commit(&self) -> bool {
        loop {
            match self.phase.load(Ordering::Acquire) {
                PHASE_RUNNING => {
                    if self
                        .phase
                        .compare_exchange_weak(
                            PHASE_RUNNING,
                            PHASE_COMMIT_REACHED,
                            Ordering::AcqRel,
                            Ordering::Acquire,
                        )
                        .is_ok()
                    {
                        return true;
                    }
                }
                PHASE_PENDING | PHASE_CANCELLED | PHASE_CANCELLATION_ACCEPTED | PHASE_FINISHED => {
                    return false;
                }
                PHASE_COMMIT_REACHED => return true,
                _ => unreachable!("mutation phase 只由本类型写入"),
            }
        }
    }

    fn claim_execution(&self) -> WatcherPreferenceExecutionClaim {
        loop {
            match self.phase.load(Ordering::Acquire) {
                PHASE_PENDING => {
                    if self
                        .phase
                        .compare_exchange_weak(
                            PHASE_PENDING,
                            PHASE_RUNNING,
                            Ordering::AcqRel,
                            Ordering::Acquire,
                        )
                        .is_ok()
                    {
                        return WatcherPreferenceExecutionClaim::Claimed;
                    }
                }
                PHASE_CANCELLED => {
                    if self
                        .phase
                        .compare_exchange_weak(
                            PHASE_CANCELLED,
                            PHASE_FINISHED,
                            Ordering::AcqRel,
                            Ordering::Acquire,
                        )
                        .is_ok()
                    {
                        return WatcherPreferenceExecutionClaim::Cancelled;
                    }
                }
                PHASE_RUNNING | PHASE_CANCELLATION_ACCEPTED | PHASE_COMMIT_REACHED => {
                    return WatcherPreferenceExecutionClaim::InUse;
                }
                PHASE_FINISHED => return WatcherPreferenceExecutionClaim::Finished,
                _ => unreachable!("mutation phase 只由本类型写入"),
            }
        }
    }

    fn finish_claimed(&self) -> WatcherPreferenceFinishOutcome {
        loop {
            let (current, outcome) = match self.phase.load(Ordering::Acquire) {
                PHASE_RUNNING => (PHASE_RUNNING, WatcherPreferenceFinishOutcome::Completed),
                PHASE_CANCELLATION_ACCEPTED => (
                    PHASE_CANCELLATION_ACCEPTED,
                    WatcherPreferenceFinishOutcome::CancellationWon,
                ),
                PHASE_COMMIT_REACHED => (
                    PHASE_COMMIT_REACHED,
                    WatcherPreferenceFinishOutcome::Completed,
                ),
                PHASE_PENDING | PHASE_CANCELLED | PHASE_FINISHED => {
                    unreachable!("只有占用 control 的执行者可以结束 mutation")
                }
                _ => unreachable!("mutation phase 只由本类型写入"),
            };
            if self
                .phase
                .compare_exchange_weak(current, PHASE_FINISHED, Ordering::AcqRel, Ordering::Acquire)
                .is_ok()
            {
                return outcome;
            }
        }
    }
}

pub trait WatcherPreferenceMutationPort {
    fn mutate(
        &self,
        request: &WatcherPreferenceMutationRequest,
        control: &WatcherPreferenceMutationControl,
    ) -> WatcherPreferenceMutationReceipt;
}

#[derive(Clone)]
pub struct MutateWatcherPreference<P> {
    port: P,
}

impl<P> MutateWatcherPreference<P> {
    #[must_use]
    pub const fn new(port: P) -> Self {
        Self { port }
    }
}

impl<P: WatcherPreferenceMutationPort> MutateWatcherPreference<P> {
    #[must_use]
    pub fn execute(
        &self,
        request: &WatcherPreferenceMutationRequest,
        control: &WatcherPreferenceMutationControl,
    ) -> WatcherPreferenceMutationReceipt {
        match control.claim_execution() {
            WatcherPreferenceExecutionClaim::Cancelled => terminal_receipt(
                request,
                WatcherPreferenceMutationOutcome::Cancelled,
                "WATCHER_PREFERENCE_MUTATION_CANCELLED",
            ),
            WatcherPreferenceExecutionClaim::InUse => terminal_receipt(
                request,
                WatcherPreferenceMutationOutcome::Failed,
                "WATCHER_PREFERENCE_MUTATION_CONTROL_IN_USE",
            ),
            WatcherPreferenceExecutionClaim::Finished => terminal_receipt(
                request,
                WatcherPreferenceMutationOutcome::Failed,
                "WATCHER_PREFERENCE_MUTATION_CONTROL_FINISHED",
            ),
            WatcherPreferenceExecutionClaim::Claimed => {
                let receipt = self.port.mutate(request, control);
                if control.finish_claimed() == WatcherPreferenceFinishOutcome::CancellationWon {
                    cancellation_won_receipt(request, receipt.final_observation())
                } else {
                    receipt
                }
            }
        }
    }
}

fn cancellation_won_receipt(
    request: &WatcherPreferenceMutationRequest,
    final_observation: WatcherPreferenceFinalObservation,
) -> WatcherPreferenceMutationReceipt {
    WatcherPreferenceMutationReceipt::new(
        request.request_id(),
        request.desired(),
        WatcherPreferenceSetupCommit::NotRequired,
        WatcherPreferenceMarkerCommit::NotAttempted,
        final_observation,
        WatcherPreferenceMutationOutcome::Cancelled,
        DiagnosticCode::new("WATCHER_PREFERENCE_MUTATION_CANCELLED"),
    )
}

fn terminal_receipt(
    request: &WatcherPreferenceMutationRequest,
    outcome: WatcherPreferenceMutationOutcome,
    diagnostic_code: &'static str,
) -> WatcherPreferenceMutationReceipt {
    WatcherPreferenceMutationReceipt::new(
        request.request_id(),
        request.desired(),
        WatcherPreferenceSetupCommit::NotRequired,
        WatcherPreferenceMarkerCommit::NotAttempted,
        WatcherPreferenceFinalObservation::Unknown,
        outcome,
        DiagnosticCode::new(diagnostic_code),
    )
}
