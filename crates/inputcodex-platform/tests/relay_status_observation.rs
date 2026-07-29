use inputcodex_application::RelayStatusObservationPort;
#[cfg(not(any(target_os = "windows", target_os = "macos")))]
use inputcodex_application::{ErrorKind, RelayStatusObservationRequest};
use inputcodex_platform::SystemRelayStatusObservation;

#[test]
fn 系统适配器实现_relay_状态观察端口() {
    fn assert_port<T: RelayStatusObservationPort + Default>() {}

    assert_port::<SystemRelayStatusObservation>();
}

#[cfg(not(any(target_os = "windows", target_os = "macos")))]
#[test]
fn 非发布目标明确返回_relay_status_observation_unsupported() {
    let error = SystemRelayStatusObservation
        .observe(&RelayStatusObservationRequest)
        .expect_err("非发布目标必须明确失败");

    assert_eq!(error.kind(), ErrorKind::Unsupported);
    assert_eq!(
        error.code().as_str(),
        "RELAY_STATUS_OBSERVATION_UNSUPPORTED"
    );
}
