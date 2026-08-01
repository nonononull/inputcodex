use inputcodex_application::WatcherPreferenceObservationPort;
#[cfg(not(any(target_os = "windows", target_os = "macos")))]
use inputcodex_application::{ErrorKind, WatcherPreferenceObservationRequest};
use inputcodex_platform::SystemWatcherPreferenceObservation;

#[test]
fn 系统适配器实现_watcher_偏好观察端口() {
    fn assert_port<T: WatcherPreferenceObservationPort + Default>() {}

    assert_port::<SystemWatcherPreferenceObservation>();
}

#[cfg(not(any(target_os = "windows", target_os = "macos")))]
#[test]
fn 非发布目标明确返回_watcher_preference_unsupported() {
    let error = SystemWatcherPreferenceObservation
        .observe(&WatcherPreferenceObservationRequest)
        .expect_err("非发布目标必须明确失败");

    assert_eq!(error.kind(), ErrorKind::Unsupported);
    assert_eq!(error.code().as_str(), "WATCHER_PREFERENCE_UNSUPPORTED");
}
