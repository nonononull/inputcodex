use inputcodex_domain::{
    ContextEntryCatalogObservation, ContextEntryKind, ContextEntryObservation,
    ContextEntryObservationError,
};

#[test]
fn 上下文条目只公开标识种类与启用状态() {
    let entry =
        ContextEntryObservation::new("context7".to_owned(), ContextEntryKind::McpServer, true)
            .expect("非空 ID 应被接受");

    assert_eq!(entry.id(), "context7");
    assert_eq!(entry.kind(), ContextEntryKind::McpServer);
    assert!(entry.is_enabled());
}

#[test]
fn 空白上下文条目标识被领域层拒绝() {
    assert_eq!(
        ContextEntryObservation::new("   ".to_owned(), ContextEntryKind::Skill, true),
        Err(ContextEntryObservationError::EmptyId)
    );
}

#[test]
fn 上下文目录保留源顺序并由条目计算分类计数() {
    let entries = vec![
        ContextEntryObservation::new("context7".to_owned(), ContextEntryKind::McpServer, true)
            .unwrap(),
        ContextEntryObservation::new("writer".to_owned(), ContextEntryKind::Skill, false).unwrap(),
        ContextEntryObservation::new("local".to_owned(), ContextEntryKind::Plugin, true).unwrap(),
        ContextEntryObservation::new("reviewer".to_owned(), ContextEntryKind::Skill, true).unwrap(),
    ];

    let catalog = ContextEntryCatalogObservation::new(entries);

    assert_eq!(
        catalog
            .entries()
            .iter()
            .map(ContextEntryObservation::id)
            .collect::<Vec<_>>(),
        vec!["context7", "writer", "local", "reviewer"]
    );

    let mcp = catalog.summary(ContextEntryKind::McpServer);
    assert_eq!(mcp.total(), 1);
    assert_eq!(mcp.enabled(), 1);
    assert_eq!(mcp.disabled(), 0);

    let skills = catalog.summary(ContextEntryKind::Skill);
    assert_eq!(skills.total(), 2);
    assert_eq!(skills.enabled(), 1);
    assert_eq!(skills.disabled(), 1);

    let plugins = catalog.summary(ContextEntryKind::Plugin);
    assert_eq!(plugins.total(), 1);
    assert_eq!(plugins.enabled(), 1);
    assert_eq!(plugins.disabled(), 0);
}

#[test]
fn 合法零条目目录仍保留三类零计数() {
    let catalog = ContextEntryCatalogObservation::new(Vec::new());

    for kind in [
        ContextEntryKind::McpServer,
        ContextEntryKind::Skill,
        ContextEntryKind::Plugin,
    ] {
        let summary = catalog.summary(kind);
        assert_eq!(summary.total(), 0);
        assert_eq!(summary.enabled(), 0);
        assert_eq!(summary.disabled(), 0);
    }
}
