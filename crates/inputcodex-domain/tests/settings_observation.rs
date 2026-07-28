use inputcodex_domain::SettingsDocumentObservation;

#[test]
fn 设置文档观察只公开顶层条目数量() {
    let observation = SettingsDocumentObservation::new(3);

    assert_eq!(observation.top_level_entry_count(), 3);
    assert_eq!(
        format!("{observation:?}"),
        "SettingsDocumentObservation { top_level_entry_count: 3 }"
    );
}

#[test]
fn 合法空对象仍保留为零条目观察事实() {
    let observation = SettingsDocumentObservation::new(0);

    assert_eq!(observation.top_level_entry_count(), 0);
}
