use inputcodex_domain::{
    LocalSessionTitle, MAX_MARKDOWN_FILENAME_BYTES, MAX_MARKDOWN_MESSAGE_COUNT,
    MAX_MARKDOWN_OUTPUT_BYTES, MarkdownGenerationError, MarkdownMessage, MarkdownMessageRole,
    MarkdownUtcTimestamp, SessionMarkdownDocument,
};

fn message(role: MarkdownMessageRole, timestamp: Option<&str>, body: &str) -> MarkdownMessage {
    MarkdownMessage::new(
        role,
        timestamp.map(|value| {
            MarkdownUtcTimestamp::new(value.to_owned()).expect("测试时间戳应为规范 UTC")
        }),
        body.to_owned(),
    )
    .expect("测试消息应合法")
}

#[test]
fn 文档生成固定角色时间戳_lf_和图片占位() {
    let title = LocalSessionTitle::from_raw("  发布\r\n计划  ").expect("标题应合法");
    let document = SessionMarkdownDocument::generate(
        Some(&title),
        vec![
            message(
                MarkdownMessageRole::User,
                Some("2026-07-30T11:14:29.123Z"),
                "第一行\r\n第二行",
            ),
            message(
                MarkdownMessageRole::Assistant,
                None,
                "处理完成\r\n\r\n> Image attachment omitted",
            ),
        ],
    )
    .expect("文档应生成");

    assert_eq!(document.suggested_filename(), "session-发布-计划.md");
    assert_eq!(document.message_count(), 2);
    assert_eq!(
        document.markdown(),
        concat!(
            "# 发布 计划\n\n",
            "### User\n",
            "_2026-07-30T11:14:29.123Z_\n\n",
            "第一行\n第二行\n\n",
            "### Assistant\n\n",
            "处理完成\n\n",
            "> Image attachment omitted\n",
        )
    );
    assert!(!document.markdown().contains('\r'));
}

#[test]
fn utc_时间戳只接受有效的规范零时区格式() {
    let timestamp = MarkdownUtcTimestamp::new("2024-02-29T23:59:59.123456789Z".to_owned())
        .expect("闰日与纳秒精度应合法");
    assert_eq!(timestamp.as_str(), "2024-02-29T23:59:59.123456789Z");
    let leap_second =
        MarkdownUtcTimestamp::new("2016-12-31T23:59:60Z".to_owned()).expect("UTC 月末闰秒应合法");
    assert_eq!(leap_second.as_str(), "2016-12-31T23:59:60Z");

    for invalid in [
        "2026-02-29T00:00:00Z",
        "2026-07-30T11:14:29+08:00",
        "2026-07-30 11:14:29Z",
        "2026-07-30T11:14:60Z",
        "2026-07-30T11:14:61Z",
        "2026-07-30T11:14:29z",
        "",
    ] {
        assert_eq!(
            MarkdownUtcTimestamp::new(invalid.to_owned()),
            Err(MarkdownGenerationError::InvalidUtcTimestamp),
            "应拒绝非规范 UTC：{invalid}"
        );
    }
}

#[test]
fn 建议文件名清洗跨平台非法字符并按_utf8_边界截断() {
    let dirty = LocalSessionTitle::from_raw("  Bad<>:\"/\\|?*  Title...  ").expect("标题应合法");
    let dirty_document = SessionMarkdownDocument::generate(
        Some(&dirty),
        vec![message(MarkdownMessageRole::User, None, "正文")],
    )
    .expect("文档应生成");
    assert_eq!(dirty_document.suggested_filename(), "session-Bad-Title.md");

    let long_title = LocalSessionTitle::from_raw(&"会".repeat(256)).expect("长标题应合法");
    let document = SessionMarkdownDocument::generate(
        Some(&long_title),
        vec![message(MarkdownMessageRole::User, None, "正文")],
    )
    .expect("文档应生成");
    let filename = document.suggested_filename();

    assert!(filename.starts_with("session-"));
    assert!(filename.ends_with(".md"));
    assert!(filename.len() <= MAX_MARKDOWN_FILENAME_BYTES);
    assert!(!filename.contains(['<', '>', ':', '"', '/', '\\', '|', '?', '*']));
    assert!(filename.is_char_boundary(filename.len()));
}

#[test]
fn 标题缺失使用稳定英文回退名() {
    let document = SessionMarkdownDocument::generate(
        None,
        vec![message(MarkdownMessageRole::Assistant, None, "Ready")],
    )
    .expect("无标题文档应生成");

    assert_eq!(document.suggested_filename(), "session-Untitled-session.md");
    assert!(document.markdown().starts_with("# Untitled session\n\n"));
}

#[test]
fn 空正文和零消息被拒绝() {
    assert_eq!(
        MarkdownMessage::new(MarkdownMessageRole::User, None, "\r\n\t ".to_owned()),
        Err(MarkdownGenerationError::EmptyMessageBody)
    );
    assert_eq!(
        SessionMarkdownDocument::generate(None, Vec::new()),
        Err(MarkdownGenerationError::NoMessages)
    );
}

#[test]
fn 消息数量和_markdown_输出均受固定上限约束() {
    let item = message(MarkdownMessageRole::User, None, "x");
    assert_eq!(
        SessionMarkdownDocument::generate(None, vec![item.clone(); MAX_MARKDOWN_MESSAGE_COUNT + 1],),
        Err(MarkdownGenerationError::TooManyMessages)
    );

    let oversized = message(
        MarkdownMessageRole::Assistant,
        None,
        &"x".repeat(MAX_MARKDOWN_OUTPUT_BYTES),
    );
    assert_eq!(
        SessionMarkdownDocument::generate(None, vec![oversized]),
        Err(MarkdownGenerationError::MarkdownTooLarge)
    );
}

#[test]
fn 消息与结果调试输出只保留结构事实() {
    let item = message(
        MarkdownMessageRole::User,
        Some("2026-07-30T11:14:29Z"),
        "客户密钥轮换计划",
    );
    let message_debug = format!("{item:?}");
    assert!(message_debug.contains("role: User"));
    assert!(message_debug.contains("has_timestamp: true"));
    assert!(message_debug.contains("body_bytes:"));
    assert!(!message_debug.contains("客户密钥轮换计划"));
    assert!(!message_debug.contains("2026-07-30"));

    let title = LocalSessionTitle::from_raw("内部项目标题").expect("标题应合法");
    let document = SessionMarkdownDocument::generate(Some(&title), vec![item]).expect("应生成");
    let document_debug = format!("{document:?}");
    assert!(document_debug.contains("message_count: 1"));
    assert!(document_debug.contains("markdown_bytes:"));
    assert!(document_debug.contains("filename_bytes:"));
    assert!(!document_debug.contains("内部项目标题"));
    assert!(!document_debug.contains("客户密钥轮换计划"));
    assert!(!document_debug.contains("session-"));
}
