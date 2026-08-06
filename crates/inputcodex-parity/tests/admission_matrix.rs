use std::{collections::BTreeSet, fs, path::PathBuf};

use inputcodex_parity::{
    AdmissionBucket, AdmissionDecision, AdmissionOwnerState, parse_side_effect_admission_matrix,
};

const VALID_MATRIX: &str = r#"schema_version: inputcodex.side-effect-admission-matrix.v1
release:
  tag: v1.2.44
  commit: 77091ccaee4423f35a1b2c51c4ecd703e6201092
entries:
  - source_id: core-module:settings
    feature_id: feature.foundation-platform.settings-management
    bucket: write
    owner_state: missing
    owner_kinds: [typed-mutation]
    blocker_refs: [https://github.com/nonononull/inputcodex/issues/140]
    admission: blocked
    implementation_authorized: false
"#;

fn repository_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|path| path.parent())
        .expect("parity crate 应位于仓库 crates 目录")
        .to_path_buf()
}

#[test]
fn 合法矩阵严格解析并保留领域类型() {
    let matrix = parse_side_effect_admission_matrix(VALID_MATRIX).expect("合法矩阵应可解析");

    assert_eq!(
        matrix.schema_version(),
        "inputcodex.side-effect-admission-matrix.v1"
    );
    assert_eq!(matrix.release_tag(), "v1.2.44");
    assert_eq!(
        matrix.release_commit(),
        "77091ccaee4423f35a1b2c51c4ecd703e6201092"
    );
    assert_eq!(matrix.entries().len(), 1);
    let entry = &matrix.entries()[0];
    assert_eq!(entry.source_id(), "core-module:settings");
    assert_eq!(
        entry.feature_id(),
        "feature.foundation-platform.settings-management"
    );
    assert_eq!(entry.bucket(), AdmissionBucket::Write);
    assert_eq!(entry.owner_state(), AdmissionOwnerState::Missing);
    assert_eq!(entry.owner_kinds(), ["typed-mutation"]);
    assert_eq!(
        entry.blocker_refs(),
        ["https://github.com/nonononull/inputcodex/issues/140"]
    );
    assert_eq!(entry.admission(), AdmissionDecision::Blocked);
    assert!(!entry.implementation_authorized());
}

#[test]
fn 根_release_与_entry_未知字段分别被拒绝() {
    let mutants = [
        VALID_MATRIX.replacen(
            "schema_version: inputcodex.side-effect-admission-matrix.v1\n",
            "schema_version: inputcodex.side-effect-admission-matrix.v1\nunexpected_root: true\n",
            1,
        ),
        VALID_MATRIX.replacen(
            "  commit: 77091ccaee4423f35a1b2c51c4ecd703e6201092\n",
            "  commit: 77091ccaee4423f35a1b2c51c4ecd703e6201092\n  unexpected_release: true\n",
            1,
        ),
        VALID_MATRIX.replacen(
            "    implementation_authorized: false\n",
            "    implementation_authorized: false\n    unexpected_entry: true\n",
            1,
        ),
    ];

    for mutant in mutants {
        assert!(
            parse_side_effect_admission_matrix(&mutant).is_err(),
            "未知字段必须由生产 serde 边界拒绝"
        );
    }
}

#[test]
fn 仓库矩阵精确覆盖当前未评估来源且全部阻断() {
    let text = fs::read_to_string(
        repository_root().join("parity/admission/side-effect-admission-matrix.yml"),
    )
    .expect("应能读取仓库 admission matrix");
    let matrix = parse_side_effect_admission_matrix(&text).expect("仓库矩阵应可严格解析");

    assert_eq!(matrix.entries().len(), 83);
    let mut source_ids = BTreeSet::new();
    let mut feature_ids = BTreeSet::new();
    let mut write_sources = 0;
    let mut process_sources = 0;
    let mut network_sources = 0;
    let mut write_features = BTreeSet::new();
    let mut process_features = BTreeSet::new();
    let mut network_features = BTreeSet::new();

    for entry in matrix.entries() {
        assert!(source_ids.insert(entry.source_id()));
        feature_ids.insert(entry.feature_id());
        assert_eq!(entry.owner_state(), AdmissionOwnerState::Missing);
        assert_eq!(entry.admission(), AdmissionDecision::Blocked);
        assert!(!entry.implementation_authorized());
        match entry.bucket() {
            AdmissionBucket::Write => {
                write_sources += 1;
                write_features.insert(entry.feature_id());
            }
            AdmissionBucket::ProcessControl => {
                process_sources += 1;
                process_features.insert(entry.feature_id());
            }
            AdmissionBucket::Network => {
                network_sources += 1;
                network_features.insert(entry.feature_id());
            }
        }
    }

    assert_eq!(source_ids.len(), 83);
    assert_eq!(feature_ids.len(), 22);
    assert_eq!((write_features.len(), write_sources), (16, 70));
    assert_eq!((process_features.len(), process_sources), (2, 5));
    assert_eq!((network_features.len(), network_sources), (4, 8));

    for entry in matrix
        .entries()
        .iter()
        .filter(|entry| entry.feature_id() == "feature.provider-network.model-catalog")
    {
        assert!(entry.owner_kinds().contains(&"credential-profile"));
    }
}
