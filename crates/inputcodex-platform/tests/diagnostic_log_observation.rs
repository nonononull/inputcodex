use inputcodex_application::DiagnosticLogObservationPort;
#[cfg(not(any(target_os = "windows", target_os = "macos")))]
use inputcodex_application::{DiagnosticLogObservationRequest, ErrorKind};
use inputcodex_platform::SystemDiagnosticLogObservation;

#[test]
fn 系统适配器实现诊断日志观察端口() {
    fn assert_port<T: DiagnosticLogObservationPort + Default>() {}

    assert_port::<SystemDiagnosticLogObservation>();
}

#[cfg(not(any(target_os = "windows", target_os = "macos")))]
#[test]
fn 非发布目标明确返回_diagnostic_log_observation_unsupported() {
    let error = SystemDiagnosticLogObservation
        .observe(&DiagnosticLogObservationRequest)
        .expect_err("非发布目标必须明确失败");

    assert_eq!(error.kind(), ErrorKind::Unsupported);
    assert_eq!(
        error.code().as_str(),
        "DIAGNOSTIC_LOG_OBSERVATION_UNSUPPORTED"
    );
}
