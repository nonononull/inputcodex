use inputcodex_application::SettingsObservationPort;
#[cfg(not(any(target_os = "windows", target_os = "macos")))]
use inputcodex_application::{ErrorKind, SettingsObservationRequest};
use inputcodex_platform::SystemSettingsObservation;

#[test]
fn 系统适配器实现设置观察端口() {
    fn assert_port<T: SettingsObservationPort + Default>() {}

    assert_port::<SystemSettingsObservation>();
}

#[cfg(not(any(target_os = "windows", target_os = "macos")))]
#[test]
fn 非发布目标明确返回_settings_observation_unsupported() {
    let error = SystemSettingsObservation
        .observe(&SettingsObservationRequest)
        .expect_err("非发布目标必须明确失败");

    assert_eq!(error.kind(), ErrorKind::Unsupported);
    assert_eq!(error.code().as_str(), "SETTINGS_OBSERVATION_UNSUPPORTED");
}
