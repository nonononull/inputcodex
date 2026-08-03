use std::{
    cell::Cell,
    rc::Rc,
    sync::{
        Arc, Barrier,
        atomic::{AtomicUsize, Ordering},
    },
    thread,
};

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
    cancel_before_return: Option<Rc<Cell<Option<WatcherPreferenceCancellationOutcome>>>>,
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
        if let Some(outcome) = &self.cancel_before_return {
            outcome.set(Some(control.cancel()));
        }
        self.receipt
    }
}

#[derive(Clone)]
struct BlockingPort {
    calls: Arc<AtomicUsize>,
    first_call_entered: Arc<Barrier>,
    release_first_call: Arc<Barrier>,
}

impl WatcherPreferenceMutationPort for BlockingPort {
    fn mutate(
        &self,
        _request: &WatcherPreferenceMutationRequest,
        control: &WatcherPreferenceMutationControl,
    ) -> WatcherPreferenceMutationReceipt {
        let call = self.calls.fetch_add(1, Ordering::SeqCst);
        if call == 0 {
            assert!(control.try_reach_commit());
            self.first_call_entered.wait();
            self.release_first_call.wait();
        }
        receipt(WatcherPreferenceMutationOutcome::Applied)
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
        cancel_before_return: None,
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
        cancel_before_return: None,
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
        cancel_before_return: None,
    };
    let use_case = MutateWatcherPreference::new(port);
    let control = WatcherPreferenceMutationControl::new();

    let result = use_case.execute(&request(), &control);

    assert_eq!(calls.get(), 1);
    assert_eq!(result, expected);
    assert_eq!(control.phase(), WatcherPreferenceMutationPhase::Finished);
    assert_eq!(
        control.cancel(),
        WatcherPreferenceCancellationOutcome::Finished
    );
}

#[test]
fn 提交前端口返回时已接受取消必须交付_cancelled_收据() {
    let calls = Rc::new(Cell::new(0));
    let cancellation = Rc::new(Cell::new(None));
    let final_observation =
        WatcherPreferenceFinalObservation::Known(WatcherPreference::EnabledByDefault);
    let port_receipt = WatcherPreferenceMutationReceipt::new(
        WatcherPreferenceMutationId::new(143),
        WatcherPreference::ExplicitlyDisabled,
        WatcherPreferenceSetupCommit::NotRequired,
        WatcherPreferenceMarkerCommit::NotAttempted,
        final_observation,
        WatcherPreferenceMutationOutcome::AlreadySatisfied,
        DiagnosticCode::new("WATCHER_PREFERENCE_MUTATION_ALREADY_SATISFIED"),
    );
    let port = StubPort {
        calls: Rc::clone(&calls),
        receipt: port_receipt,
        reach_commit: false,
        cancel_at_commit: None,
        cancel_before_return: Some(Rc::clone(&cancellation)),
    };
    let use_case = MutateWatcherPreference::new(port);
    let control = WatcherPreferenceMutationControl::new();

    let result = use_case.execute(&request(), &control);

    assert_eq!(calls.get(), 1);
    assert_eq!(
        cancellation.get(),
        Some(WatcherPreferenceCancellationOutcome::Accepted)
    );
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
    assert_eq!(result.final_observation(), final_observation);
    assert_eq!(
        result.diagnostic_code().as_str(),
        "WATCHER_PREFERENCE_MUTATION_CANCELLED"
    );
    assert_eq!(control.phase(), WatcherPreferenceMutationPhase::Finished);
}

#[test]
fn 未被执行占用的_control_不能进入提交阶段() {
    let control = WatcherPreferenceMutationControl::new();
    let cloned = control.clone();

    assert!(!cloned.try_reach_commit());
    assert_eq!(control.phase(), WatcherPreferenceMutationPhase::Pending);
    assert_eq!(
        control.cancel(),
        WatcherPreferenceCancellationOutcome::Accepted
    );
}

#[test]
fn 同一_control_到达提交点后并发执行仍只有首个执行者进入_port() {
    let calls = Arc::new(AtomicUsize::new(0));
    let first_call_entered = Arc::new(Barrier::new(2));
    let release_first_call = Arc::new(Barrier::new(2));
    let use_case = MutateWatcherPreference::new(BlockingPort {
        calls: Arc::clone(&calls),
        first_call_entered: Arc::clone(&first_call_entered),
        release_first_call: Arc::clone(&release_first_call),
    });
    let control = WatcherPreferenceMutationControl::new();

    let first = thread::spawn({
        let use_case = use_case.clone();
        let control = control.clone();
        move || use_case.execute(&request(), &control)
    });
    first_call_entered.wait();

    let second = use_case.execute(&request(), &control);
    release_first_call.wait();
    let first = first.join().expect("首个 mutation 线程必须完成");

    assert_eq!(calls.load(Ordering::SeqCst), 1);
    assert_eq!(first.outcome(), WatcherPreferenceMutationOutcome::Applied);
    assert_eq!(second.outcome(), WatcherPreferenceMutationOutcome::Failed);
    assert_eq!(
        second.diagnostic_code().as_str(),
        "WATCHER_PREFERENCE_MUTATION_CONTROL_IN_USE"
    );
    assert_eq!(control.phase(), WatcherPreferenceMutationPhase::Finished);
}

#[test]
fn 已完成的_control_顺序复用不再调用_port() {
    let calls = Rc::new(Cell::new(0));
    let use_case = MutateWatcherPreference::new(StubPort {
        calls: Rc::clone(&calls),
        receipt: receipt(WatcherPreferenceMutationOutcome::Applied),
        reach_commit: false,
        cancel_at_commit: None,
        cancel_before_return: None,
    });
    let control = WatcherPreferenceMutationControl::new();

    let first = use_case.execute(&request(), &control);
    let second = use_case.execute(&request(), &control);

    assert_eq!(calls.get(), 1);
    assert_eq!(first.outcome(), WatcherPreferenceMutationOutcome::Applied);
    assert_eq!(second.outcome(), WatcherPreferenceMutationOutcome::Failed);
    assert_eq!(
        second.diagnostic_code().as_str(),
        "WATCHER_PREFERENCE_MUTATION_CONTROL_FINISHED"
    );
    assert_eq!(control.phase(), WatcherPreferenceMutationPhase::Finished);
}
