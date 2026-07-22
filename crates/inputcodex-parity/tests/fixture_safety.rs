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
fn 路径穿越与绝对路径被拒绝() {
    let traversal = VALID_MANIFEST.replace("path: page.yml", "path: ../private.yml");
    let manifest = parse_fixture_manifest(&traversal).expect("结构仍应可解析");
    let issues = validate_fixture_manifest(
        &manifest,
        Path::new("parity/fixtures/feature.session-data.session-list"),
    );

    assert!(issues
        .iter()
        .any(|issue| issue.code() == ValidationCode::InvalidFixturePath));

    let absolute = validate_fixture_payload(
        "page.yml",
        br#"private_path: C:\Users\alice\AppData\Roaming\Codex\session.json"#,
    );
    assert!(absolute
        .iter()
        .any(|issue| issue.code() == ValidationCode::PrivateAbsolutePath));
}

#[test]
fn 真实凭据形态被拒绝而显式占位值可通过() {
    let unsafe_payload = validate_fixture_payload(
        "page.yml",
        br#"api_token: sk-live-1234567890abcdef"#,
    );
    assert!(unsafe_payload
        .iter()
        .any(|issue| issue.code() == ValidationCode::SensitiveFixtureValue));

    let safe_payload = validate_fixture_payload(
        "page.yml",
        br#"api_token: synthetic-redacted-token"#,
    );
    assert!(safe_payload.is_empty());
}
