use inputcodex_application::LoadCoordinator;
use inputcodex_infrastructure::UnconfiguredLoadPort;
use inputcodex_platform::SystemPlatform;

fn main() {
    let _application_state = LoadCoordinator::<()>::default();
    let _load_port = UnconfiguredLoadPort;
    let _platform = SystemPlatform;

    if let Err(error) = inputcodex_presentation::run() {
        eprintln!("{error}");
        std::process::exit(1);
    }
}
