use std::{collections::BTreeSet, fs, path::PathBuf};

use inputcodex_parity::{
    AdmissionDecision, AdmissionOwnerState, RequiredOwnerKind, SideEffectBucket,
    parse_side_effect_admission_matrix,
};

const RELEASE_TAG: &str = "v1.2.44";
const RELEASE_COMMIT: &str = "77091ccaee4423f35a1b2c51c4ecd703e6201092";

fn repository_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|path| path.parent())
        .expect("parity crate 应位于仓库 crates 目录")
        .to_path_buf()
}

fn matrix_text() -> String {
    fs::read_to_string(repository_root().join("parity/admission/side-effect-admission-matrix.yml"))
        .expect("应能读取副作用准入矩阵")
}

#[test]
fn 副作用准入矩阵根对象与_release_对象拒绝未知字段() {
    let valid = matrix_text().replace("\r\n", "\n");
    let root_unknown = format!("{valid}unexpected_root_field: true\n");
    assert!(
        parse_side_effect_admission_matrix(&root_unknown).is_err(),
        "根对象未知字段必须由 serde fail closed"
    );

    let release_unknown = valid.replacen(
        &format!("  tag_commit: {RELEASE_COMMIT}\n"),
        &format!("  tag_commit: {RELEASE_COMMIT}\n  unexpected_release_field: true\n"),
        1,
    );
    assert_ne!(release_unknown, valid, "测试必须真实变异 Release 对象");
    assert!(
        parse_side_effect_admission_matrix(&release_unknown).is_err(),
        "Release 对象未知字段必须由 serde fail closed"
    );
}

#[test]
fn 副作用准入矩阵精确覆盖八十三个阻断来源且零实现授权() {
    let matrix = parse_side_effect_admission_matrix(&matrix_text()).expect("准入矩阵应可解析");
    assert_eq!(matrix.release_tag(), RELEASE_TAG);
    assert_eq!(matrix.release_commit(), RELEASE_COMMIT);
    assert_eq!(matrix.entries().len(), 83);

    let mut ids = BTreeSet::new();
    let mut write_sources = 0;
    let mut process_sources = 0;
    let mut network_sources = 0;
    let mut write_features = BTreeSet::new();
    let mut process_features = BTreeSet::new();
    let mut network_features = BTreeSet::new();

    for entry in matrix.entries() {
        assert!(ids.insert(entry.source_id()), "来源 ID 必须唯一");
        assert_eq!(entry.owner_state(), AdmissionOwnerState::Missing);
        assert_eq!(entry.admission(), AdmissionDecision::Blocked);
        assert!(!entry.implementation_authorized());
        assert!(!entry.required_owner_kinds().is_empty());
        assert!(!entry.blocker_refs().is_empty());

        match entry.bucket() {
            SideEffectBucket::Write => {
                write_sources += 1;
                write_features.insert(entry.feature_id());
            }
            SideEffectBucket::Process => {
                process_sources += 1;
                process_features.insert(entry.feature_id());
            }
            SideEffectBucket::Network => {
                network_sources += 1;
                network_features.insert(entry.feature_id());
            }
        }

        if entry.feature_id() == "feature.provider-network.model-catalog" {
            assert!(
                entry
                    .required_owner_kinds()
                    .contains(&RequiredOwnerKind::CredentialProfile),
                "model-catalog 必须显式等待 credential-profile owner"
            );
        }
    }

    assert_eq!(
        (
            write_features.len(),
            write_sources,
            process_features.len(),
            process_sources,
            network_features.len(),
            network_sources,
        ),
        (16, 70, 2, 5, 4, 8)
    );
}
