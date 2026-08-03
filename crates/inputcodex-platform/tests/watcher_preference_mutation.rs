use inputcodex_application::WatcherPreferenceMutationPort;
#[cfg(not(any(target_os = "windows", target_os = "macos")))]
use inputcodex_application::{WatcherPreferenceMutationControl, WatcherPreferenceMutationRequest};
#[cfg(not(any(target_os = "windows", target_os = "macos")))]
use inputcodex_domain::{
    WatcherPreference, WatcherPreferenceFinalObservation, WatcherPreferenceMutationId,
    WatcherPreferenceMutationOutcome,
};
use inputcodex_platform::SystemWatcherPreferenceMutation;

#[test]
fn 系统适配器实现_watcher_偏好变更端口() {
    fn assert_port<T: WatcherPreferenceMutationPort + Default>() {}

    assert_port::<SystemWatcherPreferenceMutation>();
}

#[cfg(not(any(target_os = "windows", target_os = "macos")))]
#[test]
fn 非发布目标返回带完整收据的_unsupported() {
    let request = WatcherPreferenceMutationRequest::new(
        WatcherPreferenceMutationId::new(143),
        WatcherPreference::EnabledByDefault,
        WatcherPreference::ExplicitlyDisabled,
    );
    let receipt =
        SystemWatcherPreferenceMutation.mutate(&request, &WatcherPreferenceMutationControl::new());

    assert_eq!(receipt.request_id(), request.request_id());
    assert_eq!(receipt.requested_preference(), request.desired());
    assert_eq!(
        receipt.final_observation(),
        WatcherPreferenceFinalObservation::Unknown
    );
    assert_eq!(receipt.outcome(), WatcherPreferenceMutationOutcome::Failed);
    assert_eq!(
        receipt.diagnostic_code().as_str(),
        "WATCHER_PREFERENCE_MUTATION_UNSUPPORTED"
    );
}
