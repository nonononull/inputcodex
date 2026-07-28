use inputcodex_domain::{ApplicationVersion, StartupIntent, VersionStartupSnapshot};

#[test]
fn 版本启动快照复用既有应用版本类型() {
    let version =
        ApplicationVersion::new("1.2.43-inputcodex.1".to_owned()).expect("合法版本应创建成功");
    let snapshot = VersionStartupSnapshot::new(version.clone(), StartupIntent::Default);

    assert_eq!(snapshot.inputcodex_version(), &version);
    assert_eq!(snapshot.startup_intent(), StartupIntent::Default);
}

#[test]
fn 版本启动快照可表达展示更新意图() {
    let version = ApplicationVersion::new("1.2.43".to_owned()).expect("合法版本应创建成功");
    let snapshot = VersionStartupSnapshot::new(version, StartupIntent::ShowUpdate);

    assert_eq!(snapshot.startup_intent(), StartupIntent::ShowUpdate);
}
