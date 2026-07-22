use std::path::PathBuf;

use inputcodex_parity::validate_repository;

#[test]
fn 仓库功能目录通过完整引用与安全验证() {
    let repository_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|path| path.parent())
        .expect("parity crate 应位于仓库 crates 目录")
        .to_path_buf();

    let summary = validate_repository(&repository_root).expect("仓库功能目录应通过验证");

    assert!(summary.source_entry_count() > 0);
    assert!(summary.feature_count() > 0);
    assert!(summary.contract_count() > 0);
}
