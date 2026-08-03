use inputcodex_domain::{
    DiagnosticCode, WatcherPreference, WatcherPreferenceFinalObservation,
    WatcherPreferenceMarkerCommit, WatcherPreferenceMutationId, WatcherPreferenceMutationOutcome,
    WatcherPreferenceMutationReceipt, WatcherPreferenceSetupCommit,
};

#[test]
fn 收据保留请求目标提交和最终观察() {
    let request_id = WatcherPreferenceMutationId::new(143);
    let receipt = WatcherPreferenceMutationReceipt::new(
        request_id,
        WatcherPreference::ExplicitlyDisabled,
        WatcherPreferenceSetupCommit::RootCreated,
        WatcherPreferenceMarkerCommit::Created,
        WatcherPreferenceFinalObservation::Known(WatcherPreference::ExplicitlyDisabled),
        WatcherPreferenceMutationOutcome::Applied,
        DiagnosticCode::new("WATCHER_PREFERENCE_MUTATION_APPLIED"),
    );

    assert_eq!(request_id.value(), 143);
    assert_eq!(receipt.request_id(), request_id);
    assert_eq!(
        receipt.requested_preference(),
        WatcherPreference::ExplicitlyDisabled
    );
    assert_eq!(
        receipt.setup_commit(),
        WatcherPreferenceSetupCommit::RootCreated
    );
    assert_eq!(
        receipt.marker_commit(),
        WatcherPreferenceMarkerCommit::Created
    );
    assert_eq!(
        receipt.final_observation(),
        WatcherPreferenceFinalObservation::Known(WatcherPreference::ExplicitlyDisabled)
    );
    assert_eq!(receipt.outcome(), WatcherPreferenceMutationOutcome::Applied);
    assert_eq!(
        receipt.diagnostic_code().as_str(),
        "WATCHER_PREFERENCE_MUTATION_APPLIED"
    );
}

#[test]
fn 领域类型覆盖全部批准结果和提交状态() {
    let outcomes = [
        WatcherPreferenceMutationOutcome::Applied,
        WatcherPreferenceMutationOutcome::AlreadySatisfied,
        WatcherPreferenceMutationOutcome::Conflict,
        WatcherPreferenceMutationOutcome::Cancelled,
        WatcherPreferenceMutationOutcome::Failed,
        WatcherPreferenceMutationOutcome::Indeterminate,
    ];
    let setup_commits = [
        WatcherPreferenceSetupCommit::NotRequired,
        WatcherPreferenceSetupCommit::RootCreated,
        WatcherPreferenceSetupCommit::Failed,
    ];
    let marker_commits = [
        WatcherPreferenceMarkerCommit::NotAttempted,
        WatcherPreferenceMarkerCommit::Created,
        WatcherPreferenceMarkerCommit::Removed,
        WatcherPreferenceMarkerCommit::Failed,
    ];
    let observations = [
        WatcherPreferenceFinalObservation::Known(WatcherPreference::EnabledByDefault),
        WatcherPreferenceFinalObservation::Known(WatcherPreference::ExplicitlyDisabled),
        WatcherPreferenceFinalObservation::Unknown,
    ];

    assert_eq!(outcomes.len(), 6);
    assert_eq!(setup_commits.len(), 3);
    assert_eq!(marker_commits.len(), 4);
    assert_eq!(observations.len(), 3);
}

#[test]
fn 状态根已创建但标记失败可以显式交付() {
    let receipt = WatcherPreferenceMutationReceipt::new(
        WatcherPreferenceMutationId::new(7),
        WatcherPreference::ExplicitlyDisabled,
        WatcherPreferenceSetupCommit::RootCreated,
        WatcherPreferenceMarkerCommit::Failed,
        WatcherPreferenceFinalObservation::Known(WatcherPreference::EnabledByDefault),
        WatcherPreferenceMutationOutcome::Failed,
        DiagnosticCode::new("WATCHER_PREFERENCE_MUTATION_MARKER_FAILED"),
    );

    assert_eq!(
        receipt.setup_commit(),
        WatcherPreferenceSetupCommit::RootCreated
    );
    assert_eq!(
        receipt.marker_commit(),
        WatcherPreferenceMarkerCommit::Failed
    );
    assert_eq!(
        receipt.final_observation(),
        WatcherPreferenceFinalObservation::Known(WatcherPreference::EnabledByDefault)
    );
    assert_eq!(receipt.outcome(), WatcherPreferenceMutationOutcome::Failed);
}

#[test]
fn debug_不包含固定文件或私人路径语义() {
    let receipt = WatcherPreferenceMutationReceipt::new(
        WatcherPreferenceMutationId::new(9),
        WatcherPreference::EnabledByDefault,
        WatcherPreferenceSetupCommit::NotRequired,
        WatcherPreferenceMarkerCommit::Removed,
        WatcherPreferenceFinalObservation::Known(WatcherPreference::EnabledByDefault),
        WatcherPreferenceMutationOutcome::Applied,
        DiagnosticCode::new("WATCHER_PREFERENCE_MUTATION_APPLIED"),
    );
    let debug = format!("{receipt:?}");

    for forbidden in [
        "watcher.disabled",
        "inputcodex_state_root",
        "C:\\",
        "/Users/",
        "installed",
        "running",
        "process",
    ] {
        assert!(
            !debug.contains(forbidden),
            "debug 泄漏禁止内容: {forbidden}"
        );
    }
}
