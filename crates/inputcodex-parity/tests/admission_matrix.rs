use inputcodex_parity::{
    AdmissionBucket, AdmissionState, OwnerState, ValidationCode,
    parse_side_effect_admission_matrix, validate_repository, validate_side_effect_admission_matrix,
};
use std::{
    collections::BTreeSet,
    fs,
    path::{Path, PathBuf},
    sync::atomic::{AtomicU64, Ordering},
};

static NEXT_FIXTURE_ID: AtomicU64 = AtomicU64::new(0);

const VALID_MATRIX: &str = r#"
schema_version: inputcodex.side-effect-admission-matrix.v1
release:
  tag: v1.2.44
  tag_commit: 77091ccaee4423f35a1b2c51c4ecd703e6201092
entries:
  - source_id: core-module:settings
    feature_id: feature.foundation-platform.settings-management
    primary_bucket: write
    owner_state: missing
    required_owner_kinds: [settings-document]
    blocker_refs: [issue:94]
    admission: blocked
    implementation_authorized: false
"#;

#[test]
fn 规范矩阵保留严格领域值() {
    let matrix = parse_side_effect_admission_matrix(VALID_MATRIX).expect("规范矩阵应可解析");
    let entries = matrix.entries();

    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].source_id(), "core-module:settings");
    assert_eq!(entries[0].primary_bucket(), AdmissionBucket::Write);
    assert_eq!(entries[0].owner_state(), OwnerState::Missing);
    assert_eq!(entries[0].admission(), AdmissionState::Blocked);
    assert!(!entries[0].implementation_authorized());
}

#[test]
fn 未知字段在解析阶段被拒绝() {
    let invalid = VALID_MATRIX.replace(
        "    implementation_authorized: false",
        "    implementation_authorized: false\n    fallback: forbidden",
    );

    assert!(parse_side_effect_admission_matrix(&invalid).is_err());
}

#[test]
fn 重复来源与实现授权在验证阶段被拒绝() {
    let duplicate = VALID_MATRIX.replace(
        "entries:\n",
        "entries:\n  - source_id: core-module:settings\n    feature_id: feature.foundation-platform.settings-management\n    primary_bucket: write\n    owner_state: missing\n    required_owner_kinds: [settings-document]\n    blocker_refs: [issue:94]\n    admission: blocked\n    implementation_authorized: false\n",
    );
    let matrix =
        parse_side_effect_admission_matrix(&duplicate).expect("重复来源应在语义验证阶段拒绝");
    assert!(!validate_side_effect_admission_matrix(&matrix).is_empty());

    let authorized = VALID_MATRIX.replace(
        "implementation_authorized: false",
        "implementation_authorized: true",
    );
    let matrix = parse_side_effect_admission_matrix(&authorized).expect("布尔值形状合法");
    assert!(!validate_side_effect_admission_matrix(&matrix).is_empty());
}

#[test]
fn 仓库矩阵锁定全部未评估来源与三类桶() {
    let text = fs::read_to_string(
        repository_root().join("parity/admission/side-effect-admission-matrix.yml"),
    )
    .expect("应能读取仓库矩阵");
    let matrix = parse_side_effect_admission_matrix(&text).expect("仓库矩阵应可解析");
    assert!(validate_side_effect_admission_matrix(&matrix).is_empty());
    assert_eq!(matrix.entries().len(), 83);

    let mut write_features = BTreeSet::new();
    let mut process_features = BTreeSet::new();
    let mut network_features = BTreeSet::new();
    let mut write_sources = 0;
    let mut process_sources = 0;
    let mut network_sources = 0;
    for entry in matrix.entries() {
        assert_eq!(entry.owner_state(), OwnerState::Missing);
        assert_eq!(entry.admission(), AdmissionState::Blocked);
        assert!(!entry.implementation_authorized());
        assert!(
            entry
                .blocker_refs()
                .iter()
                .any(|value| value == "issue:140")
        );
        match entry.primary_bucket() {
            AdmissionBucket::Write => {
                write_sources += 1;
                write_features.insert(entry.feature_id());
            }
            AdmissionBucket::Process => {
                process_sources += 1;
                process_features.insert(entry.feature_id());
            }
            AdmissionBucket::Network => {
                network_sources += 1;
                network_features.insert(entry.feature_id());
            }
        }
    }

    assert_eq!((write_features.len(), write_sources), (16, 70));
    assert_eq!((process_features.len(), process_sources), (2, 5));
    assert_eq!((network_features.len(), network_sources), (4, 8));
    assert_eq!(
        write_features.len() + process_features.len() + network_features.len(),
        22
    );

    let model_catalog = matrix
        .entries()
        .iter()
        .filter(|entry| entry.feature_id() == "feature.provider-network.model-catalog")
        .collect::<Vec<_>>();
    assert_eq!(model_catalog.len(), 4);
    assert!(model_catalog.iter().all(|entry| {
        entry
            .required_owner_kinds()
            .iter()
            .any(|owner| owner == "credential-profile")
    }));
}

#[test]
fn 仓库验证拒绝矩阵闭包与owner漂移() {
    let fixture = AdmissionRepositoryFixture::new();
    let original = fs::read_to_string(
        fixture
            .root()
            .join("parity/admission/side-effect-admission-matrix.yml"),
    )
    .expect("应能读取临时矩阵")
    .replace("\r\n", "\n");
    let first_entry = "  - source_id: core-module:ccs_import\n    feature_id: feature.provider-network.provider-import\n    primary_bucket: write\n    owner_state: missing\n    required_owner_kinds: [application-file-mutation, credential-profile]\n    blocker_refs: [issue:126, issue:140]\n    admission: blocked\n    implementation_authorized: false\n";

    for (name, mutated, expected_code) in [
        (
            "遗漏来源",
            original.replacen(first_entry, "", 1),
            ValidationCode::AdmissionSourceCoverageMismatch,
        ),
        (
            "feature 漂移",
            original.replacen(
                "feature_id: feature.provider-network.provider-import",
                "feature_id: feature.provider-network.aggregate-routing",
                1,
            ),
            ValidationCode::AdmissionFeatureMismatch,
        ),
        (
            "bucket 漂移",
            original.replacen("primary_bucket: write", "primary_bucket: network", 1),
            ValidationCode::AdmissionBucketMismatch,
        ),
        (
            "owner 漂移",
            original.replacen(
                "required_owner_kinds: [application-file-mutation, credential-profile]",
                "required_owner_kinds: [application-file-mutation]",
                1,
            ),
            ValidationCode::AdmissionOwnerMismatch,
        ),
        (
            "blocker 漂移",
            original.replacen(
                "blocker_refs: [issue:126, issue:140]",
                "blocker_refs: [issue:140]",
                1,
            ),
            ValidationCode::AdmissionBlockerMismatch,
        ),
    ] {
        assert_ne!(mutated, original, "{name} 变异必须真实修改矩阵");
        fixture.write_matrix(&mutated);
        let error = validate_repository(fixture.root()).expect_err("{name} 必须 fail closed");
        assert!(
            error
                .issues()
                .iter()
                .any(|issue| issue.code() == expected_code),
            "{name} 应报告 {expected_code:?}，实际为 {:?}",
            error.issues()
        );
    }
}

fn repository_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

struct AdmissionRepositoryFixture {
    root: PathBuf,
}

impl AdmissionRepositoryFixture {
    fn new() -> Self {
        let target_root = repository_root().join("target");
        fs::create_dir_all(&target_root).expect("应能创建临时矩阵父目录");
        let root = loop {
            let id = NEXT_FIXTURE_ID.fetch_add(1, Ordering::Relaxed);
            let candidate = target_root.join(format!(
                "inputcodex-admission-matrix-{}-{id}",
                std::process::id()
            ));
            match fs::create_dir(&candidate) {
                Ok(()) => break candidate,
                Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {}
                Err(error) => panic!("应能创建临时矩阵目录：{error}"),
            }
        };
        copy_tree(&repository_root().join("parity"), &root.join("parity"));
        copy_tree(&repository_root().join("upstream"), &root.join("upstream"));
        Self { root }
    }

    fn root(&self) -> &Path {
        &self.root
    }

    fn write_matrix(&self, text: &str) {
        fs::write(
            self.root
                .join("parity/admission/side-effect-admission-matrix.yml"),
            text,
        )
        .expect("应能写入临时矩阵");
    }
}

impl Drop for AdmissionRepositoryFixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

fn copy_tree(source: &Path, destination: &Path) {
    fs::create_dir_all(destination).expect("应能创建临时目录");
    for entry in fs::read_dir(source).expect("应能读取源目录") {
        let entry = entry.expect("应能读取目录项");
        let destination_path = destination.join(entry.file_name());
        let file_type = entry.file_type().expect("应能读取目录项类型");
        if file_type.is_dir() {
            copy_tree(&entry.path(), &destination_path);
        } else if file_type.is_file() {
            fs::copy(entry.path(), destination_path).expect("应能复制临时夹具文件");
        }
    }
}
