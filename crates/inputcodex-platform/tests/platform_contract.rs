#[cfg(not(any(target_os = "windows", target_os = "macos")))]
use inputcodex_application::ErrorKind;
#[cfg(any(target_os = "windows", target_os = "macos"))]
use inputcodex_application::PlatformKind;
use inputcodex_application::PlatformPort;
use inputcodex_platform::SystemPlatform;

#[test]
fn 当前目标返回统一平台语义() {
    let result = SystemPlatform.current_platform();

    #[cfg(target_os = "windows")]
    assert_eq!(result, Ok(PlatformKind::Windows));

    #[cfg(target_os = "macos")]
    assert_eq!(result, Ok(PlatformKind::Macos));

    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    {
        let error = result.expect_err("非发布目标必须明确 unsupported");
        assert_eq!(error.kind(), ErrorKind::Unsupported);
        assert_eq!(error.code().as_str(), "PLATFORM_UNSUPPORTED");
    }
}
