use std::ffi::OsString;

#[cfg(not(any(target_os = "windows", target_os = "macos")))]
use inputcodex_application::VersionStartupRequest;
use inputcodex_application::{ErrorKind, VersionStartupPort};
use inputcodex_domain::StartupIntent;
use inputcodex_platform::{SystemVersionStartup, resolve_version_startup};

fn process_arguments(arguments: &[&str]) -> Vec<OsString> {
    std::iter::once(OsString::from("inputcodex"))
        .chain(arguments.iter().map(OsString::from))
        .collect()
}

#[cfg(unix)]
fn non_unicode_value() -> OsString {
    use std::os::unix::ffi::OsStringExt;

    OsString::from_vec(vec![0xff])
}

#[cfg(windows)]
fn non_unicode_value() -> OsString {
    use std::os::windows::ffi::OsStringExt;

    OsString::from_wide(&[0xd800])
}

#[test]
fn 系统版本启动适配器实现统一应用端口() {
    fn assert_port<T: VersionStartupPort + Default>() {}

    assert_port::<SystemVersionStartup>();
}

#[test]
fn 默认输入返回编译期版本与默认启动意图() {
    let snapshot =
        resolve_version_startup(process_arguments(&[]), None).expect("默认输入必须成功解析");

    assert_eq!(
        snapshot.inputcodex_version().as_str(),
        env!("CARGO_PKG_VERSION")
    );
    assert_eq!(snapshot.startup_intent(), StartupIntent::Default);
}

#[test]
fn 只有精确命令行参数请求展示更新() {
    let show = resolve_version_startup(process_arguments(&["--show-update"]), None)
        .expect("精确参数必须成功解析");
    assert_eq!(show.startup_intent(), StartupIntent::ShowUpdate);

    for value in ["--show-update=true", "--SHOW-UPDATE", "show-update"] {
        let snapshot = resolve_version_startup(process_arguments(&[value]), None)
            .expect("其他参数不应破坏默认启动");
        assert_eq!(snapshot.startup_intent(), StartupIntent::Default);
    }
}

#[test]
fn 环境变量只接受未设置_空值_零或一() {
    for value in [None, Some(OsString::from("")), Some(OsString::from("0"))] {
        let snapshot =
            resolve_version_startup(process_arguments(&[]), value).expect("默认环境值必须成功解析");
        assert_eq!(snapshot.startup_intent(), StartupIntent::Default);
    }

    let snapshot = resolve_version_startup(process_arguments(&[]), Some(OsString::from("1")))
        .expect("值一必须成功解析");
    assert_eq!(snapshot.startup_intent(), StartupIntent::ShowUpdate);

    let snapshot = resolve_version_startup(
        process_arguments(&["--show-update"]),
        Some(OsString::from("0")),
    )
    .expect("默认环境值必须继续允许精确命令行参数");
    assert_eq!(snapshot.startup_intent(), StartupIntent::ShowUpdate);
}

#[test]
fn 非法环境值返回稳定_invalid_input_且不泄露原值() {
    for value in [
        OsString::from("true"),
        OsString::from(" 1 "),
        non_unicode_value(),
    ] {
        let error = resolve_version_startup(process_arguments(&[]), Some(value))
            .expect_err("非法显式环境值必须失败");

        assert_eq!(error.kind(), ErrorKind::InvalidInput);
        assert_eq!(error.code().as_str(), "INVALID_STARTUP_OPTION");
        assert!(!format!("{error:?}").contains("true"));
    }
}

#[test]
fn 非法环境值优先于命令行展示更新参数失败() {
    let error = resolve_version_startup(
        process_arguments(&["--show-update"]),
        Some(OsString::from("invalid")),
    )
    .expect_err("命令行参数不得掩盖非法显式环境值");

    assert_eq!(error.kind(), ErrorKind::InvalidInput);
    assert_eq!(error.code().as_str(), "INVALID_STARTUP_OPTION");
}

#[cfg(not(any(target_os = "windows", target_os = "macos")))]
#[test]
fn 非发布目标系统入口明确返回_version_and_startup_unsupported() {
    let error = SystemVersionStartup
        .load(&VersionStartupRequest)
        .expect_err("非发布目标必须明确失败");

    assert_eq!(error.kind(), ErrorKind::Unsupported);
    assert_eq!(error.code().as_str(), "VERSION_AND_STARTUP_UNSUPPORTED");
}
