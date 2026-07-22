#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationCode {
    InvalidFeatureId,
    DuplicateFeatureId,
    FeatureDomainMismatch,
    DuplicateContractId,
    ContractIdentityMismatch,
    MissingLoadingState,
    DanglingFeatureReference,
    DanglingFixtureReference,
    CrossFeatureFixtureReference,
    DuplicateFixtureId,
    FixtureFeatureMismatch,
    InvalidFixturePath,
    InvalidFixturePayload,
    PrivateAbsolutePath,
    SensitiveFixtureValue,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationIssue {
    code: ValidationCode,
    location: String,
}

impl ValidationIssue {
    pub(crate) fn new(code: ValidationCode, location: impl Into<String>) -> Self {
        Self {
            code,
            location: location.into(),
        }
    }

    #[must_use]
    pub const fn code(&self) -> ValidationCode {
        self.code
    }

    #[must_use]
    pub fn location(&self) -> &str {
        &self.location
    }
}
