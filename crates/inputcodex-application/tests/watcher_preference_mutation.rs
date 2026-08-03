use std::{cell::Cell, rc::Rc};

use inputcodex_application::{
    MutateWatcherPreference, WatcherPreferenceCancellationOutcome,
    WatcherPreferenceMutationControl, WatcherPreferenceMutationPhase,
    WatcherPreferenceMutationPort, WatcherPreferenceMutationRequest,
};
use inputcodex_domain::{
    DiagnosticCode, WatcherPreference, WatcherPreferenceFinalObservation,
    WatcherPreferenceMarkerCommit, WatcherPreferenceMutationId, WatcherPreferenceMutationOutcome,
    WatcherPreferenceMutationReceipt, WatcherPreferenceSetupCommit,
};

struct StubPort {
    calls: Rc<Cell<usize>>,
    receipt: WatcherPreferenceMutationReceipt,
    reach_commit: bool,
    cancel_at_commit: Option<Rc<Cell<Option<WatcherPreferenceCancellationOutcome>>>>,
}

impl WatcherPreferenceMutationPort for StubPort {
    fn mutate(
        &self,
        _request: &WatcherPreferenceMutationRequest,
        control: &WatcherPreferenceMutationControl,
    ) -> WatcherPreferenceMutationReceipt {
        self.calls.set(self.calls.get() + 1);
        if self.reach_commit {
            assert!(control.try_reach_commit());
            if let Some(outcome) = &self.cancel_at_commit {
                outcome.set(Some(control.cancel()));
            }
        }
        self.receipt
    }
}

fn request() -> WatcherPreferenceMutationRequest {
    WatcherPreferenceMutationRequest::new(
        WatcherPreferenceMutationId::new(143),
        WatcherPreference::EnabledByDefault,
        WatcherPreference::ExplicitlyDisabled,
    )
}

fn receipt(outcome: WatcherPreferenceMutationOutcome) -> WatcherPreferenceMutationReceipt {
    WatcherPreferenceMutationReceipt::new(
        WatcherPreferenceMutationId::new(143),
        WatcherPreference::ExplicitlyDisabled,
        WatcherPreferenceSetupCommit::NotRequired,
        WatcherPreferenceMarkerCommit::Created,
        WatcherPreferenceFinalObservation::Known(WatcherPreference::ExplicitlyDisabled),
        outcome,
        DiagnosticCode::new("WATCHER_PREFERENCE_MUTATION_TEST"),
    )
}

#[test]
fn 请求必须同时保留_expected_与_desired() {
    let request = request();

    assert_eq!(request.request_id(), WatcherPreferenceMutationId::new(143));
    assert_eq!(request.expected(), WatcherPreference::EnabledByDefault);
    assert_eq!(request.desired(), WatcherPreference::ExplicitlyDisabled);

    let debug = format!("{request:?}");
    for forbidden in [
        "watcher.disabled",
        "inputcodex_state_root",
        "C:\\",
        "/Users/",
    ] {
        assert!(!debug.contains(forbidden));
    }
}

#[test]
fn 提交前取消不调用_port_并返回_cancelled_收据() {
    let calls = Rc::new(Cell::new(0));
    let port = StubPort {
        calls: Rc::clone(&calls),
        receipt: receipt(WatcherPreferenceMutationOutcome::Applied),
        reach_commit: false,
        cancel_at_commit: None,
    };
    let use_case = MutateWatcherPreference::new(port);
    let control = WatcherPreferenceMutationControl::new();

    assert_eq!(
        control.cancel(),
        WatcherPreferenceCancellationOutcome::Accepted
    );
    assert_eq!(control.phase(), WatcherPreferenceMutationPhase::Cancelled);

    let result = use_case.execute(&request(), &control);

    assert_eq!(calls.get(), 0);
    assert_eq!(
        result.outcome(),
        WatcherPreferenceMutationOutcome::Cancelled
    );
    assert_eq!(
        result.setup_commit(),
        WatcherPreferenceSetupCommit::NotRequired
    );
    assert_eq!(
        result.marker_commit(),
        WatcherPreferenceMarkerCommit::NotAttempted
    );
    assert_eq!(
        result.final_observation(),
        WatcherPreferenceFinalObservation::Unknown
    );
    assert_eq!(
        result.diagnostic_code().as_str(),
        "WATCHER_PREFERENCE_MUTATION_CANCELLED"
    );
    assert_eq!(control.phase(), WatcherPreferenceMutationPhase::Finished);
}

#[test]
fn 提交点后的取消返回_too_late_但收据不丢弃() {
    let calls = Rc::new(Cell::new(0));
    let cancellation = Rc::new(Cell::new(None));
    let expected = receipt(WatcherPreferenceMutationOutcome::Applied);
    let port = StubPort {
        calls: Rc::clone(&calls),
        receipt: expected,
        reach_commit: true,
        cancel_at_commit: Some(Rc::clone(&cancellation)),
    };
    let use_case = MutateWatcherPreference::new(port);
    let control = WatcherPreferenceMutationControl::new();

    let result = use_case.execute(&request(), &control);

    assert_eq!(calls.get(), 1);
    assert_eq!(
        cancellation.get(),
        Some(WatcherPreferenceCancellationOutcome::TooLate)
    );
    assert_eq!(result, expected);
    assert_eq!(control.phase(), WatcherPreferenceMutationPhase::Finished);
    assert_eq!(
        control.cancel(),
        WatcherPreferenceCancellationOutcome::Finished
    );
}

#[test]
fn 无提交结果也在交付后进入_finished() {
    let calls = Rc::new(Cell::new(0));
    let expected = receipt(WatcherPreferenceMutationOutcome::AlreadySatisfied);
    let port = StubPort {
        calls: Rc::clone(&calls),
        receipt: expected,
        reach_commit: false,
        cancel_at_commit: None,
    };
    let use_case = MutateWatcherPreference::new(port);
    let control = WatcherPreferenceMutationControl::new();

    let result = use_case.execute(&request(), &control);

    assert_eq!(calls.get(), 1);
    assert_eq!(result, expected);
    assert_eq!(control.phase(), WatcherPreferenceMutationPhase::Finished);
}

#[test]
fn control_clone_共享同一提交阶段() {
    let control = WatcherPreferenceMutationControl::new();
    let cloned = control.clone();

    assert!(cloned.try_reach_commit());
    assert_eq!(
        control.phase(),
        WatcherPreferenceMutationPhase::CommitReached
    );
    assert_eq!(
        control.cancel(),
        WatcherPreferenceCancellationOutcome::TooLate
    );
}
