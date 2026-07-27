use std::path::PathBuf;

use inputcodex_domain::{
    ApplicationInstallSource, CodexInstallation, PlatformPathsSnapshot, PrivatePath,
    PrivatePathError,
};

#[test]
fn 私有路径拒绝空值和相对路径且_debug_脱敏() {
    assert_eq!(
        PrivatePath::new(PathBuf::new()),
        Err(PrivatePathError::Empty)
    );
    assert_eq!(
        PrivatePath::new(PathBuf::from("relative/.codex")),
        Err(PrivatePathError::Relative)
    );

    let absolute = std::env::temp_dir().join("inputcodex-private-path");
    let value = PrivatePath::new(absolute.clone()).expect("临时目录路径应为绝对路径");
    let debug = format!("{value:?}");

    assert_eq!(debug, "PrivatePath(<redacted>)");
    assert!(!debug.contains(&absolute.to_string_lossy().to_string()));
    assert_eq!(value.as_path(), absolute.as_path());
}

#[test]
fn 平台快照保留明确空安装且所有路径继续脱敏() {
    let root = std::env::temp_dir().join("inputcodex-platform-paths-domain");
    let private = |name: &str| PrivatePath::new(root.join(name)).expect("派生路径应为绝对路径");
    let installation = CodexInstallation::new(
        private("Codex.app"),
        private("Codex.app/Contents/MacOS/Codex"),
        ApplicationInstallSource::Explicit,
    );
    let snapshot = PlatformPathsSnapshot::new(
        private(".codex"),
        private("state"),
        private("state/settings.json"),
        private("state/latest-status.json"),
        private("state/inputcodex.log"),
        Some(installation),
    );

    assert!(snapshot.codex_installation().is_some());
    assert!(!format!("{snapshot:?}").contains(&root.to_string_lossy().to_string()));
}
