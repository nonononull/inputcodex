use inputcodex_domain::DiagnosticCode;

#[test]
fn 诊断码保持稳定且可复制() {
    let code = DiagnosticCode::new("PLATFORM_UNSUPPORTED");
    let copied = code;

    assert_eq!(copied.as_str(), "PLATFORM_UNSUPPORTED");
    assert_eq!(code, copied);
}
