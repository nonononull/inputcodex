#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RelayDocumentStatus {
    Missing,
    Valid,
    Invalid,
    TooLarge,
    Unreadable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CredentialPresence {
    Present,
    Absent,
    NotObserved,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RelayConfigurationStatus {
    NotConfigured,
    Complete,
    Incomplete,
    NotObserved,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RelayStatusObservation {
    auth_document_status: RelayDocumentStatus,
    config_document_status: RelayDocumentStatus,
    chatgpt_credentials: CredentialPresence,
    openai_api_key: CredentialPresence,
    relay_configuration: RelayConfigurationStatus,
}

impl RelayStatusObservation {
    #[must_use]
    pub const fn new(
        auth_document_status: RelayDocumentStatus,
        config_document_status: RelayDocumentStatus,
        chatgpt_credentials: CredentialPresence,
        openai_api_key: CredentialPresence,
        relay_configuration: RelayConfigurationStatus,
    ) -> Self {
        Self {
            auth_document_status,
            config_document_status,
            chatgpt_credentials,
            openai_api_key,
            relay_configuration,
        }
    }

    #[must_use]
    pub const fn auth_document_status(self) -> RelayDocumentStatus {
        self.auth_document_status
    }

    #[must_use]
    pub const fn config_document_status(self) -> RelayDocumentStatus {
        self.config_document_status
    }

    #[must_use]
    pub const fn chatgpt_credentials(self) -> CredentialPresence {
        self.chatgpt_credentials
    }

    #[must_use]
    pub const fn openai_api_key(self) -> CredentialPresence {
        self.openai_api_key
    }

    #[must_use]
    pub const fn relay_configuration(self) -> RelayConfigurationStatus {
        self.relay_configuration
    }
}
