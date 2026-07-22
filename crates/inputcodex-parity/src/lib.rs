#![forbid(unsafe_code)]

use inputcodex_application::{ApplicationError, ErrorKind};
use inputcodex_domain::DiagnosticCode;

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
