use inputcodex_application::ApplicationOverviewPort;
use inputcodex_platform::SystemApplicationOverview;

#[test]
fn 系统应用概览适配器实现统一应用端口且不需要_unsafe() {
    fn assert_port<T: ApplicationOverviewPort + Default>() {}

    assert_port::<SystemApplicationOverview>();
}
