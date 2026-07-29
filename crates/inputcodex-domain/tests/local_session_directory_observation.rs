use inputcodex_domain::{
    LocalSessionDirectoryEntry, LocalSessionDirectoryEntryError, LocalSessionDirectoryPage,
    LocalSessionDirectoryPageError, LocalSessionSourceCoverage, LocalSessionSourceSummary,
    LocalSessionSourceSummaryError, LocalSessionTitle, MAX_LOCAL_SESSION_DIRECTORY_PAGE_SIZE,
    MAX_LOCAL_SESSION_ID_BYTES, MAX_LOCAL_SESSION_TITLE_CHARS,
};

fn entry(id: &str, title: Option<&str>, updated_at_ms: Option<i64>) -> LocalSessionDirectoryEntry {
    LocalSessionDirectoryEntry::new(
        id.to_string(),
        title.and_then(LocalSessionTitle::from_raw),
        false,
        updated_at_ms,
    )
    .expect("测试条目应合法")
}

#[test]
fn 标题规范化空白控制字符且调试输出脱敏() {
    let title = LocalSessionTitle::from_raw(" \t  设计\r\n评审\u{0007}  ").expect("应形成标题");

    assert_eq!(title.as_str(), "设计 评审");
    assert!(!title.was_truncated());

    let debug = format!("{title:?}");
    assert!(debug.contains("character_count: 5"));
    assert!(!debug.contains("设计"));
    assert!(!debug.contains("评审"));
}

#[test]
fn 空白或只有控制字符的标题映射为缺失() {
    assert!(LocalSessionTitle::from_raw(" \t\r\n ").is_none());
    assert!(LocalSessionTitle::from_raw("\u{0007}\u{001b}").is_none());
}

#[test]
fn 标题在二百五十六字符边界截断并保留事实() {
    let exact = "会".repeat(MAX_LOCAL_SESSION_TITLE_CHARS);
    let exact_title = LocalSessionTitle::from_raw(&exact).expect("边界标题应合法");
    assert_eq!(
        exact_title.as_str().chars().count(),
        MAX_LOCAL_SESSION_TITLE_CHARS
    );
    assert!(!exact_title.was_truncated());

    let oversized = format!("{exact}尾");
    let truncated = LocalSessionTitle::from_raw(&oversized).expect("超长标题应形成有界结果");
    assert_eq!(truncated.as_str(), exact);
    assert!(truncated.was_truncated());
}

#[test]
fn 会话标识必须非空有界且不含空白或控制字符() {
    assert_eq!(
        LocalSessionDirectoryEntry::new(String::new(), None, false, None),
        Err(LocalSessionDirectoryEntryError::EmptySessionId)
    );
    assert_eq!(
        LocalSessionDirectoryEntry::new(" session-1".to_string(), None, false, None),
        Err(LocalSessionDirectoryEntryError::InvalidSessionId)
    );
    assert_eq!(
        LocalSessionDirectoryEntry::new("session\n1".to_string(), None, false, None),
        Err(LocalSessionDirectoryEntryError::InvalidSessionId)
    );
    assert_eq!(
        LocalSessionDirectoryEntry::new(
            "x".repeat(MAX_LOCAL_SESSION_ID_BYTES + 1),
            None,
            false,
            None,
        ),
        Err(LocalSessionDirectoryEntryError::SessionIdTooLong)
    );
}

#[test]
fn 会话条目只暴露批准字段且调试输出不泄露内容() {
    let item = LocalSessionDirectoryEntry::new(
        "session-private-42".to_string(),
        LocalSessionTitle::from_raw("客户密钥轮换计划"),
        true,
        Some(123_456),
    )
    .expect("条目应合法");

    assert_eq!(item.session_id(), "session-private-42");
    assert_eq!(
        item.display_title().map(LocalSessionTitle::as_str),
        Some("客户密钥轮换计划")
    );
    assert!(item.is_archived());
    assert_eq!(item.updated_at_ms(), Some(123_456));

    let debug = format!("{item:?}");
    assert!(!debug.contains("session-private-42"));
    assert!(!debug.contains("客户密钥轮换计划"));
}

#[test]
fn 来源摘要区分完整与部分覆盖并拒绝不一致计数() {
    let complete = LocalSessionSourceSummary::new(2, 2, 0).expect("完整覆盖应合法");
    assert_eq!(complete.coverage(), LocalSessionSourceCoverage::Complete);
    assert_eq!(complete.discovered(), 2);
    assert_eq!(complete.readable(), 2);
    assert_eq!(complete.failed(), 0);

    let partial = LocalSessionSourceSummary::new(3, 2, 1).expect("部分覆盖应合法");
    assert_eq!(partial.coverage(), LocalSessionSourceCoverage::Partial);

    assert_eq!(
        LocalSessionSourceSummary::new(0, 0, 0),
        Err(LocalSessionSourceSummaryError::NoSources)
    );
    assert_eq!(
        LocalSessionSourceSummary::new(2, 1, 0),
        Err(LocalSessionSourceSummaryError::CountMismatch)
    );
    assert_eq!(
        LocalSessionSourceSummary::new(2, 0, 2),
        Err(LocalSessionSourceSummaryError::NoReadableSources)
    );
}

#[test]
fn 页面必须非空且条目数不得超过合法页大小() {
    let sources = LocalSessionSourceSummary::new(2, 2, 0).expect("来源应合法");
    let page = LocalSessionDirectoryPage::new(
        vec![entry("session-2", Some("第二个"), Some(2))],
        0,
        50,
        true,
        sources,
    )
    .expect("页面应合法");

    assert_eq!(page.entries().len(), 1);
    assert_eq!(page.offset(), 0);
    assert_eq!(page.limit(), 50);
    assert!(page.has_more());
    assert_eq!(page.sources(), sources);

    assert_eq!(
        LocalSessionDirectoryPage::new(Vec::new(), 0, 50, false, sources),
        Err(LocalSessionDirectoryPageError::EmptyEntries)
    );
    assert_eq!(
        LocalSessionDirectoryPage::new(vec![entry("session-1", None, None)], 0, 0, false, sources),
        Err(LocalSessionDirectoryPageError::InvalidLimit)
    );
    assert_eq!(
        LocalSessionDirectoryPage::new(
            vec![entry("session-1", None, None)],
            0,
            MAX_LOCAL_SESSION_DIRECTORY_PAGE_SIZE + 1,
            false,
            sources,
        ),
        Err(LocalSessionDirectoryPageError::InvalidLimit)
    );

    let too_many = (0..=MAX_LOCAL_SESSION_DIRECTORY_PAGE_SIZE)
        .map(|index| entry(&format!("session-{index}"), None, None))
        .collect();
    assert_eq!(
        LocalSessionDirectoryPage::new(
            too_many,
            0,
            MAX_LOCAL_SESSION_DIRECTORY_PAGE_SIZE,
            false,
            sources,
        ),
        Err(LocalSessionDirectoryPageError::TooManyEntries)
    );
}
