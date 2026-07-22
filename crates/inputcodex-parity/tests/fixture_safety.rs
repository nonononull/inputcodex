use std::path::Path;

use inputcodex_parity::{
    ValidationCode, parse_fixture_manifest, validate_fixture_manifest, validate_fixture_payload,
};

const VALID_MANIFEST: &str = r#"
schema_version: inputcodex.fixture-manifest.v1
feature_id: feature.session-data.session-list
fixtures:
  - id: fixture.feature.session-data.session-list.synthetic-page
    scenario: synthetic-page
    kind: synthetic
    files:
      - path: page.yml
        format: yaml
        description: 合成会话分页边界数据。
"#;

#[test]
fn 合成夹具_manifest_使用仓库相对路径() {
    let manifest = parse_fixture_manifest(VALID_MANIFEST).expect("合法 manifest 应可解析");
    let issues = validate_fixture_manifest(
        &manifest,
        Path::new("parity/fixtures/feature.session-data.session-list"),
    );

    assert!(issues.is_empty());
}

#[test]
fn fixture_目录必须与_manifest_feature_一致() {
    let manifest = parse_fixture_manifest(VALID_MANIFEST).expect("合法 manifest 应可解析");
    let issues = validate_fixture_manifest(
        &manifest,
        Path::new("parity/fixtures/feature.provider-network.other"),
    );

    assert!(
        issues
            .iter()
            .any(|issue| issue.code() == ValidationCode::FixtureDirectoryMismatch)
    );
}

#[test]
fn 路径穿越与绝对路径被拒绝() {
    let traversal = VALID_MANIFEST.replace("path: page.yml", "path: ../private.yml");
    let manifest = parse_fixture_manifest(&traversal).expect("结构仍应可解析");
    let issues = validate_fixture_manifest(
        &manifest,
        Path::new("parity/fixtures/feature.session-data.session-list"),
    );

    assert!(
        issues
            .iter()
            .any(|issue| issue.code() == ValidationCode::InvalidFixturePath)
    );

    let absolute = validate_fixture_payload(
        "page.yml",
        br#"private_path: C:\Users\alice\AppData\Roaming\Codex\session.json"#,
    );
    assert!(
        absolute
            .iter()
            .any(|issue| issue.code() == ValidationCode::PrivateAbsolutePath)
    );

    let posix_absolute = validate_fixture_payload(
        "page.yml",
        br#"private_path: /Users/alice/Library/Application Support/Codex/session.json"#,
    );
    assert!(
        posix_absolute
            .iter()
            .any(|issue| issue.code() == ValidationCode::PrivateAbsolutePath)
    );
}

#[test]
fn 真实凭据形态被拒绝而显式占位值可通过() {
    let unsafe_payload =
        validate_fixture_payload("page.yml", br#"api_token: sk-live-1234567890abcdef"#);
    assert!(
        unsafe_payload
            .iter()
            .any(|issue| issue.code() == ValidationCode::SensitiveFixtureValue)
    );

    let safe_payload =
        validate_fixture_payload("page.yml", br#"api_token: synthetic-redacted-token"#);
    assert!(safe_payload.is_empty());
}

#[test]
fn 重复_fixture_id_被拒绝() {
    let entry = VALID_MANIFEST
        .split_once("fixtures:\n")
        .expect("测试 manifest 必须包含 fixtures")
        .1;
    let invalid = format!("{VALID_MANIFEST}{entry}");
    let manifest = parse_fixture_manifest(&invalid).expect("重复 fixture 仍应可解析");
    let issues = validate_fixture_manifest(
        &manifest,
        Path::new("parity/fixtures/feature.session-data.session-list"),
    );

    assert!(
        issues
            .iter()
            .any(|issue| issue.code() == ValidationCode::DuplicateFixtureId)
    );
}

#[test]
fn fixture_id_必须归属_manifest_feature() {
    let invalid = VALID_MANIFEST.replace(
        "fixture.feature.session-data.session-list.synthetic-page",
        "fixture.feature.provider-network.session-list.synthetic-page",
    );
    let manifest = parse_fixture_manifest(&invalid).expect("结构仍应可解析");
    let issues = validate_fixture_manifest(
        &manifest,
        Path::new("parity/fixtures/feature.session-data.session-list"),
    );

    assert!(
        issues
            .iter()
            .any(|issue| issue.code() == ValidationCode::FixtureFeatureMismatch)
    );
}

#[test]
fn 反斜杠路径被拒绝() {
    let invalid = VALID_MANIFEST.replace("path: page.yml", r"path: nested\page.yml");
    let manifest = parse_fixture_manifest(&invalid).expect("结构仍应可解析");
    let issues = validate_fixture_manifest(
        &manifest,
        Path::new("parity/fixtures/feature.session-data.session-list"),
    );

    assert!(
        issues
            .iter()
            .any(|issue| issue.code() == ValidationCode::InvalidFixturePath)
    );
}

#[test]
fn fixture_manifest_必填字段不可缺失() {
    let required_fragments = [
        (
            "schema_version",
            "schema_version: inputcodex.fixture-manifest.v1\n",
        ),
        (
            "feature_id",
            "feature_id: feature.session-data.session-list\n",
        ),
        (
            "id",
            "  - id: fixture.feature.session-data.session-list.synthetic-page\n",
        ),
        ("scenario", "    scenario: synthetic-page\n"),
        ("kind", "    kind: synthetic\n"),
        (
            "files",
            "    files:\n      - path: page.yml\n        format: yaml\n        description: 合成会话分页边界数据。\n",
        ),
    ];

    for (field, fragment) in required_fragments {
        assert!(
            VALID_MANIFEST.contains(fragment),
            "测试片段 {field} 必须存在"
        );
        let invalid = VALID_MANIFEST.replace(fragment, "");
        assert!(
            parse_fixture_manifest(&invalid).is_err(),
            "缺少 {field} 时必须拒绝解析"
        );
    }
}
