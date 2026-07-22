#![forbid(unsafe_code)]

mod catalog;
mod contract;
mod fixture;
mod validation;

use inputcodex_application::{ApplicationError, ErrorKind};
use inputcodex_domain::DiagnosticCode;

pub use catalog::{
    FeatureCatalog, FeatureDefinition, FeatureDomain, ParityStatus, SourceDisposition, SourceEntry,
    SourceIndex, SourceKind, SourcePlatform, parse_feature_catalog, parse_source_index,
    validate_feature_catalog, validate_source_index,
};
pub use contract::{
    BehaviorContract, ContractCatalog, LoadingContract, LoadingState, parse_contract_catalog,
    validate_contract_catalog, validate_contract_catalog_domain,
};
pub use fixture::{
    FixtureManifest, parse_fixture_manifest, validate_fixture_manifest, validate_fixture_payload,
};
pub use validation::{
    RepositorySummary, RepositoryValidationError, ValidationCode, ValidationIssue,
    validate_feature_repository, validate_repository,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ErrorSignature {
    kind: ErrorKind,
    code: DiagnosticCode,
}

impl ErrorSignature {
    #[must_use]
    pub const fn from_error(error: ApplicationError) -> Self {
        Self {
            kind: error.kind(),
            code: error.code(),
        }
    }

    #[must_use]
    pub const fn kind(self) -> ErrorKind {
        self.kind
    }

    #[must_use]
    pub const fn code(self) -> DiagnosticCode {
        self.code
    }
}
