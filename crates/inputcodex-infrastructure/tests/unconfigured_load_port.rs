use inputcodex_application::{ErrorKind, LoadPort, RequestId};
use inputcodex_infrastructure::UnconfiguredLoadPort;

#[test]
fn 未配置数据源明确失败而不伪造空结果() {
    let port = UnconfiguredLoadPort;
    let result: Result<Option<()>, _> = port.load(RequestId::new(1));
    let error = result.expect_err("未配置数据源必须失败");

    assert_eq!(error.kind(), ErrorKind::Unavailable);
    assert_eq!(error.code().as_str(), "LOAD_SOURCE_UNCONFIGURED");
}
