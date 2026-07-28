use inputcodex_application::RelayEnvironmentObservationPort;
#[cfg(not(any(target_os = "windows", target_os = "macos")))]
use inputcodex_application::{ErrorKind, RelayEnvironmentObservationRequest};
use inputcodex_platform::SystemRelayEnvironmentObservation;

#[test]
fn 系统适配器实现_relay_环境观察端口() {
    fn assert_port<T: RelayEnvironmentObservationPort + Default>() {}

    assert_port::<SystemRelayEnvironmentObservation>();
}

#[cfg(not(any(target_os = "windows", target_os = "macos")))]
#[test]
fn 非发布目标明确返回_relay_environment_unsupported() {
    let error = SystemRelayEnvironmentObservation
        .observe(&RelayEnvironmentObservationRequest)
        .expect_err("非发布目标必须明确失败");

    assert_eq!(error.kind(), ErrorKind::Unsupported);
    assert_eq!(
        error.code().as_str(),
        "RELAY_ENVIRONMENT_OBSERVATION_UNSUPPORTED"
    );
}
