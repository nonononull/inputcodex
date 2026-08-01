use inputcodex_domain::{WatcherPreference, WatcherPreferenceObservation};

#[test]
fn watcher_偏好观察保留两个批准状态() {
    for preference in [
        WatcherPreference::EnabledByDefault,
        WatcherPreference::ExplicitlyDisabled,
    ] {
        let observation = WatcherPreferenceObservation::new(preference);
        let copied = observation;

        assert_eq!(copied, observation);
        assert_eq!(observation.preference(), preference);
    }
}

#[test]
fn debug_只公开稳定偏好而不泄漏路径或运行状态() {
    let observation = WatcherPreferenceObservation::new(WatcherPreference::ExplicitlyDisabled);
    let debug = format!("{observation:?}");

    assert!(debug.contains("ExplicitlyDisabled"));
    for forbidden in [
        "watcher.disabled",
        "inputcodex_state_root",
        "installed",
        "running",
        "process",
        "C:\\\\",
        "/Users/",
    ] {
        assert!(
            !debug.contains(forbidden),
            "debug 泄漏禁止内容: {forbidden}"
        );
    }
}
