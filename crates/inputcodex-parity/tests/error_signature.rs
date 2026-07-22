use inputcodex_application::{ApplicationError, ErrorKind};
use inputcodex_domain::DiagnosticCode;
use inputcodex_parity::ErrorSignature;

#[test]
fn 错误签名只包含稳定语义() {
    let error = ApplicationError::new(
        ErrorKind::Unsupported,
        DiagnosticCode::new("PLATFORM_UNSUPPORTED"),
    );
    let signature = ErrorSignature::from_error(error);

    assert_eq!(signature.kind(), ErrorKind::Unsupported);
    assert_eq!(signature.code().as_str(), "PLATFORM_UNSUPPORTED");
}
