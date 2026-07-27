use inputcodex_application::PlatformPathsPort;
#[cfg(not(any(target_os = "windows", target_os = "macos")))]
use inputcodex_application::{ErrorKind, PlatformPathsRequest};
use inputcodex_platform::SystemPlatformPaths;

#[test]
fn 系统平台路径解析器实现应用端口且不需要_unsafe() {
    fn assert_port<T: PlatformPathsPort + Default>() {}

    assert_port::<SystemPlatformPaths>();
}

#[cfg(not(any(target_os = "windows", target_os = "macos")))]
#[test]
fn 非发布目标明确返回_platform_paths_unsupported() {
    let error = SystemPlatformPaths
        .resolve(&PlatformPathsRequest::default())
        .expect_err("非发布目标必须明确失败");

    assert_eq!(error.kind(), ErrorKind::Unsupported);
    assert_eq!(error.code().as_str(), "PLATFORM_PATHS_UNSUPPORTED");
}
