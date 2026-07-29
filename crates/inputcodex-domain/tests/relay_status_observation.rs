use inputcodex_domain::{
    CredentialPresence, RelayConfigurationStatus, RelayDocumentStatus, RelayStatusObservation,
};

#[test]
fn relay_状态观察保留五类脱敏事实() {
    let observation = RelayStatusObservation::new(
        RelayDocumentStatus::Valid,
        RelayDocumentStatus::TooLarge,
        CredentialPresence::Present,
        CredentialPresence::Absent,
        RelayConfigurationStatus::NotObserved,
    );

    assert_eq!(
        observation.auth_document_status(),
        RelayDocumentStatus::Valid
    );
    assert_eq!(
        observation.config_document_status(),
        RelayDocumentStatus::TooLarge
    );
    assert_eq!(
        observation.chatgpt_credentials(),
        CredentialPresence::Present
    );
    assert_eq!(observation.openai_api_key(), CredentialPresence::Absent);
    assert_eq!(
        observation.relay_configuration(),
        RelayConfigurationStatus::NotObserved
    );
}

#[test]
fn 领域枚举覆盖全部批准状态() {
    let document_statuses = [
        RelayDocumentStatus::Missing,
        RelayDocumentStatus::Valid,
        RelayDocumentStatus::Invalid,
        RelayDocumentStatus::TooLarge,
        RelayDocumentStatus::Unreadable,
    ];
    let credential_states = [
        CredentialPresence::Present,
        CredentialPresence::Absent,
        CredentialPresence::NotObserved,
    ];
    let configuration_states = [
        RelayConfigurationStatus::NotConfigured,
        RelayConfigurationStatus::Complete,
        RelayConfigurationStatus::Incomplete,
        RelayConfigurationStatus::NotObserved,
    ];

    assert_eq!(document_statuses.len(), 5);
    assert_eq!(credential_states.len(), 3);
    assert_eq!(configuration_states.len(), 4);
}

#[test]
fn debug_输出不包含凭据路径或上游字段() {
    let observation = RelayStatusObservation::new(
        RelayDocumentStatus::Unreadable,
        RelayDocumentStatus::Invalid,
        CredentialPresence::NotObserved,
        CredentialPresence::NotObserved,
        RelayConfigurationStatus::NotObserved,
    );

    let debug = format!("{observation:?}");
    for forbidden in [
        "auth.json",
        "config.toml",
        "OPENAI_API_KEY",
        "model_provider",
        "experimental_bearer_token",
        "base_url",
        "http://",
        "https://",
    ] {
        assert!(
            !debug.contains(forbidden),
            "debug 泄漏禁止内容: {forbidden}"
        );
    }
}
