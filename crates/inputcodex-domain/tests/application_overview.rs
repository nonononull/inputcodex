use inputcodex_domain::{
    ApplicationInstallSource, ApplicationOverview, ApplicationVersion, ApplicationVersionError,
    CodexInstallation, CollectedAtUnixMs, InstallationState, InstalledVersion,
    InstalledVersionUnknownReason, LiveProcessState, PrivatePath,
};

fn installation(name: &str) -> CodexInstallation {
    let root = std::env::temp_dir().join(name);
    CodexInstallation::new(
        PrivatePath::new(root.clone()).expect("测试安装根必须是绝对路径"),
        PrivatePath::new(root.join("Codex.exe")).expect("测试可执行文件必须是绝对路径"),
        ApplicationInstallSource::Explicit,
    )
}

#[test]
fn 应用版本去除首尾空白并拒绝非法文本() {
    let version = ApplicationVersion::new("  1.2.3  ".to_owned()).expect("合法版本应成功");

    assert_eq!(version.as_str(), "1.2.3");
    assert_eq!(
        ApplicationVersion::new("   ".to_owned()),
        Err(ApplicationVersionError::Empty)
    );
    assert_eq!(
        ApplicationVersion::new("a".repeat(129)),
        Err(ApplicationVersionError::TooLong)
    );
    assert_eq!(
        ApplicationVersion::new("1.2\n3".to_owned()),
        Err(ApplicationVersionError::ControlCharacter)
    );
}

#[test]
fn 已安装版本未知仍保留安装事实且实时状态明确未观察() {
    let overview = ApplicationOverview::new(
        InstallationState::Installed {
            installation: installation("inputcodex-overview-installed"),
            version: InstalledVersion::Unknown(InstalledVersionUnknownReason::MetadataUnreadable),
        },
        ApplicationVersion::new("0.1.0".to_owned()).expect("构建版本合法"),
        LiveProcessState::NotObserved,
        CollectedAtUnixMs::new(1234),
    );

    assert!(matches!(
        overview.installation(),
        InstallationState::Installed {
            version: InstalledVersion::Unknown(InstalledVersionUnknownReason::MetadataUnreadable),
            ..
        }
    ));
    assert_eq!(overview.inputcodex_version().as_str(), "0.1.0");
    assert_eq!(overview.live_process_state(), LiveProcessState::NotObserved);
    assert_eq!(overview.collected_at().value(), 1234);
    assert!(!format!("{overview:?}").contains("inputcodex-overview-installed"));
}

#[test]
fn 未安装是完整成功快照而不是缺失值() {
    let overview = ApplicationOverview::new(
        InstallationState::NotInstalled,
        ApplicationVersion::new("0.1.0".to_owned()).expect("构建版本合法"),
        LiveProcessState::NotObserved,
        CollectedAtUnixMs::new(5678),
    );

    assert_eq!(overview.installation(), &InstallationState::NotInstalled);
    assert_eq!(overview.live_process_state(), LiveProcessState::NotObserved);
    assert_eq!(overview.collected_at(), CollectedAtUnixMs::new(5678));
}

#[test]
fn 已知版本保留经过校验的来源文本() {
    let version = ApplicationVersion::new("1.2.43".to_owned()).expect("合法版本应成功");
    let overview = ApplicationOverview::new(
        InstallationState::Installed {
            installation: installation("inputcodex-overview-known"),
            version: InstalledVersion::Known(version.clone()),
        },
        ApplicationVersion::new("0.1.0".to_owned()).expect("构建版本合法"),
        LiveProcessState::NotObserved,
        CollectedAtUnixMs::new(9012),
    );

    assert!(matches!(
        overview.installation(),
        InstallationState::Installed {
            version: InstalledVersion::Known(actual),
            ..
        } if actual == &version
    ));
}
