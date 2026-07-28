use std::ffi::OsString;

#[cfg(not(any(target_os = "windows", target_os = "macos")))]
use inputcodex_application::RuntimeEnvironmentObservationRequest;
use inputcodex_application::{ErrorKind, RuntimeEnvironmentObservationPort};
use inputcodex_domain::EnvironmentValuePresence;
use inputcodex_platform::{
    SystemRuntimeEnvironmentObservation, observe_macos_runtime_environment,
    observe_windows_runtime_environment,
};

#[cfg(unix)]
fn non_unicode_candidate_name(prefix: &[u8]) -> OsString {
    use std::os::unix::ffi::OsStringExt;

    let mut bytes = prefix.to_vec();
    bytes.push(0xff);
    OsString::from_vec(bytes)
}

#[cfg(windows)]
fn non_unicode_candidate_name(prefix: &[u8]) -> OsString {
    use std::os::windows::ffi::OsStringExt;

    let mut wide: Vec<u16> = prefix.iter().copied().map(u16::from).collect();
    wide.push(0xd800);
    OsString::from_wide(&wide)
}

#[test]
fn 系统适配器实现运行时环境观察端口() {
    fn assert_port<T: RuntimeEnvironmentObservationPort + Default>() {}

    assert_port::<SystemRuntimeEnvironmentObservation>();
}

#[test]
fn windows_大小写不敏感而_macos_大小写敏感() {
    let pairs = vec![
        (OsString::from("openai_api_key"), OsString::from("secret")),
        (OsString::from("OPENAI_BASE_URL"), OsString::new()),
    ];

    let windows = observe_windows_runtime_environment(pairs.clone()).expect("Windows 观察应成功");
    assert_eq!(windows.scanned_entry_count(), 2);
    assert_eq!(windows.conflict_count(), 2);
    assert_eq!(windows.conflicts()[0].name().as_str(), "OPENAI_API_KEY");
    assert_eq!(windows.conflicts()[1].name().as_str(), "OPENAI_BASE_URL");

    let macos = observe_macos_runtime_environment(pairs).expect("macOS 观察应成功");
    assert_eq!(macos.scanned_entry_count(), 2);
    assert_eq!(macos.conflict_count(), 1);
    assert_eq!(macos.conflicts()[0].name().as_str(), "OPENAI_BASE_URL");
}

#[test]
fn 名称不修剪且排除非真实前缀() {
    let pairs = vec![
        (OsString::from(" OPENAI_API_KEY"), OsString::from("a")),
        (OsString::from("OPENAI_API_KEY "), OsString::from("b")),
        (OsString::from("CUSTOM_OPENAI_API_KEY"), OsString::from("c")),
        (OsString::from("X_OPENAI_API_KEY"), OsString::from("d")),
        (OsString::from("OPENAI_VALID"), OsString::from("e")),
    ];

    let windows = observe_windows_runtime_environment(pairs.clone()).expect("Windows 观察应成功");
    let macos = observe_macos_runtime_environment(pairs).expect("macOS 观察应成功");

    for observation in [&windows, &macos] {
        assert_eq!(observation.scanned_entry_count(), 5);
        assert_eq!(observation.conflict_count(), 2);
        assert_eq!(
            observation.conflicts()[0].name().as_str(),
            "OPENAI_API_KEY "
        );
        assert_eq!(observation.conflicts()[1].name().as_str(), "OPENAI_VALID");
    }
}

#[test]
fn 空值与重复项只保留存在状态且非空优先() {
    let observation = observe_windows_runtime_environment(vec![
        (OsString::from("OPENAI_API_KEY"), OsString::new()),
        (
            OsString::from("openai_api_key"),
            OsString::from("sk-private-sentinel"),
        ),
    ])
    .expect("Windows 重复项观察应成功");

    assert_eq!(observation.scanned_entry_count(), 2);
    assert_eq!(observation.conflict_count(), 1);
    assert_eq!(
        observation.conflicts()[0].value_presence(),
        EnvironmentValuePresence::NonEmpty
    );
    assert!(!format!("{observation:?}").contains("sk-private-sentinel"));
}

#[test]
fn 实际环境值不会进入结果或_debug() {
    let observation = observe_macos_runtime_environment(vec![(
        OsString::from("OPENAI_API_KEY"),
        OsString::from("sk-secret-value-must-not-appear"),
    )])
    .expect("macOS 观察应成功");

    assert_eq!(observation.conflict_count(), 1);
    assert!(!format!("{observation:?}").contains("sk-secret-value-must-not-appear"));
}

#[test]
fn 命中名称不可无损表示时整次观察明确失败() {
    for result in [
        observe_windows_runtime_environment(vec![(
            non_unicode_candidate_name(b"OPENAI_"),
            OsString::from("secret"),
        )]),
        observe_macos_runtime_environment(vec![(
            non_unicode_candidate_name(b"OPENAI_"),
            OsString::from("secret"),
        )]),
    ] {
        let error = result.expect_err("命中名称不可表示时必须失败");
        assert_eq!(error.kind(), ErrorKind::Internal);
        assert_eq!(
            error.code().as_str(),
            "RUNTIME_ENVIRONMENT_NAME_UNREPRESENTABLE"
        );
        assert!(!format!("{error:?}").contains("secret"));
    }
}

#[test]
fn 非候选的不可表示名称无需转换即可忽略() {
    let pairs = vec![(
        non_unicode_candidate_name(b"CUSTOM_OPENAI_"),
        OsString::from("secret"),
    )];

    let windows = observe_windows_runtime_environment(pairs.clone()).expect("非候选应被忽略");
    let macos = observe_macos_runtime_environment(pairs).expect("非候选应被忽略");

    assert_eq!(windows.scanned_entry_count(), 1);
    assert_eq!(windows.conflict_count(), 0);
    assert_eq!(macos.scanned_entry_count(), 1);
    assert_eq!(macos.conflict_count(), 0);
}

#[cfg(not(any(target_os = "windows", target_os = "macos")))]
#[test]
fn 非发布目标系统入口明确返回_unsupported() {
    let error = SystemRuntimeEnvironmentObservation
        .observe(&RuntimeEnvironmentObservationRequest)
        .expect_err("非发布目标必须明确失败");

    assert_eq!(error.kind(), ErrorKind::Unsupported);
    assert_eq!(
        error.code().as_str(),
        "RUNTIME_ENVIRONMENT_OBSERVATION_UNSUPPORTED"
    );
}
