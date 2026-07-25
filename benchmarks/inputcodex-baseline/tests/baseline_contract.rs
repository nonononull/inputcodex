use inputcodex_baseline::{BaselineError, run_scenario};
use std::path::{Path, PathBuf};

fn repository_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("应能定位 inputcodex 仓库根目录")
}

#[test]
fn rejects_unknown_scenario() {
    let error = run_scenario("unknown", &repository_root(), 1).expect_err("未知场景必须失败");

    assert_eq!(error, BaselineError::UnknownScenario("unknown".to_owned()));
}

#[test]
fn rejects_zero_iterations() {
    let error = run_scenario("application-load-complete", &repository_root(), 0)
        .expect_err("零迭代必须失败");

    assert_eq!(error, BaselineError::ZeroIterations);
}

#[test]
fn rejects_invalid_repository_root() {
    let missing_root = repository_root().join("target/issue-32-missing-repository-root");
    let error = run_scenario("parity-repository-validation", &missing_root, 1)
        .expect_err("无效仓库根必须失败");

    assert_eq!(error, BaselineError::InvalidRepositoryRoot(missing_root));
}

#[test]
fn measures_application_load_completion() {
    let measurement = run_scenario("application-load-complete", &repository_root(), 3)
        .expect("加载完成场景应可测量");

    assert_eq!(measurement.name(), "application-load-complete");
    assert_eq!(measurement.iterations(), 3);
    assert_ne!(measurement.checksum(), 0);
    assert_eq!(measurement.to_csv().split(',').count(), 5);
}

#[test]
fn keeps_cancellation_state_when_stale_completion_arrives() {
    let measurement = run_scenario("application-cancel-stale", &repository_root(), 1)
        .expect("取消后的陈旧完成场景应可测量");

    assert_eq!(measurement.name(), "application-cancel-stale");
    assert_eq!(measurement.iterations(), 1);
    assert_eq!(measurement.checksum(), 3_488_675_146_315_662_320);
}

#[test]
fn measures_repository_validation() {
    let measurement = run_scenario("parity-repository-validation", &repository_root(), 1)
        .expect("仓库验证场景应可测量");

    assert_eq!(measurement.name(), "parity-repository-validation");
    assert_eq!(measurement.iterations(), 1);
    assert_ne!(measurement.checksum(), 0);
}
