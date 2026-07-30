use std::fmt;

use crate::LocalSessionTitle;

pub const MAX_MARKDOWN_FILENAME_BYTES: usize = 160;
pub const MAX_MARKDOWN_MESSAGE_COUNT: usize = 20_000;
pub const MAX_MARKDOWN_OUTPUT_BYTES: usize = 16 * 1024 * 1024;

const UNTITLED_SESSION_TITLE: &str = "Untitled session";
const FILENAME_PREFIX: &str = "session-";
const FILENAME_SUFFIX: &str = ".md";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MarkdownGenerationError {
    InvalidUtcTimestamp,
    EmptyMessageBody,
    NoMessages,
    TooManyMessages,
    MarkdownTooLarge,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MarkdownMessageRole {
    User,
    Assistant,
}

impl MarkdownMessageRole {
    const fn heading(self) -> &'static str {
        match self {
            Self::User => "User",
            Self::Assistant => "Assistant",
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct MarkdownUtcTimestamp(String);

impl MarkdownUtcTimestamp {
    pub fn new(value: String) -> Result<Self, MarkdownGenerationError> {
        if is_canonical_utc_timestamp(&value) {
            Ok(Self(value))
        } else {
            Err(MarkdownGenerationError::InvalidUtcTimestamp)
        }
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for MarkdownUtcTimestamp {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("MarkdownUtcTimestamp")
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct MarkdownMessage {
    role: MarkdownMessageRole,
    timestamp: Option<MarkdownUtcTimestamp>,
    body: String,
}

impl MarkdownMessage {
    pub fn new(
        role: MarkdownMessageRole,
        timestamp: Option<MarkdownUtcTimestamp>,
        body: String,
    ) -> Result<Self, MarkdownGenerationError> {
        let body = normalize_newlines(&body).trim_end().to_owned();
        if body.trim().is_empty() {
            return Err(MarkdownGenerationError::EmptyMessageBody);
        }

        Ok(Self {
            role,
            timestamp,
            body,
        })
    }

    #[must_use]
    pub const fn role(&self) -> MarkdownMessageRole {
        self.role
    }

    #[must_use]
    pub const fn timestamp(&self) -> Option<&MarkdownUtcTimestamp> {
        self.timestamp.as_ref()
    }

    #[must_use]
    pub fn body(&self) -> &str {
        &self.body
    }
}

impl fmt::Debug for MarkdownMessage {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("MarkdownMessage")
            .field("role", &self.role)
            .field("has_timestamp", &self.timestamp.is_some())
            .field("body_bytes", &self.body.len())
            .finish()
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct SessionMarkdownDocument {
    suggested_filename: String,
    markdown: String,
    message_count: usize,
}

impl SessionMarkdownDocument {
    pub fn generate(
        title: Option<&LocalSessionTitle>,
        messages: Vec<MarkdownMessage>,
    ) -> Result<Self, MarkdownGenerationError> {
        if messages.is_empty() {
            return Err(MarkdownGenerationError::NoMessages);
        }
        if messages.len() > MAX_MARKDOWN_MESSAGE_COUNT {
            return Err(MarkdownGenerationError::TooManyMessages);
        }

        let display_title = title.map_or(UNTITLED_SESSION_TITLE, LocalSessionTitle::as_str);
        let suggested_filename = build_suggested_filename(display_title);
        let markdown = render_markdown(display_title, &messages)?;

        Ok(Self {
            suggested_filename,
            markdown,
            message_count: messages.len(),
        })
    }

    #[must_use]
    pub fn suggested_filename(&self) -> &str {
        &self.suggested_filename
    }

    #[must_use]
    pub fn markdown(&self) -> &str {
        &self.markdown
    }

    #[must_use]
    pub const fn message_count(&self) -> usize {
        self.message_count
    }
}

impl fmt::Debug for SessionMarkdownDocument {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("SessionMarkdownDocument")
            .field("message_count", &self.message_count)
            .field("markdown_bytes", &self.markdown.len())
            .field("filename_bytes", &self.suggested_filename.len())
            .finish()
    }
}

fn render_markdown(
    title: &str,
    messages: &[MarkdownMessage],
) -> Result<String, MarkdownGenerationError> {
    let mut markdown = String::new();
    push_bounded(&mut markdown, "# ")?;
    push_bounded(&mut markdown, title)?;
    push_bounded(&mut markdown, "\n\n")?;

    for (index, message) in messages.iter().enumerate() {
        push_bounded(&mut markdown, "### ")?;
        push_bounded(&mut markdown, message.role().heading())?;
        push_bounded(&mut markdown, "\n")?;
        if let Some(timestamp) = message.timestamp() {
            push_bounded(&mut markdown, "_")?;
            push_bounded(&mut markdown, timestamp.as_str())?;
            push_bounded(&mut markdown, "_\n")?;
        }
        push_bounded(&mut markdown, "\n")?;
        push_bounded(&mut markdown, message.body())?;
        push_bounded(
            &mut markdown,
            if index + 1 == messages.len() {
                "\n"
            } else {
                "\n\n"
            },
        )?;
    }

    Ok(markdown)
}

fn push_bounded(output: &mut String, value: &str) -> Result<(), MarkdownGenerationError> {
    if output.len().saturating_add(value.len()) > MAX_MARKDOWN_OUTPUT_BYTES {
        return Err(MarkdownGenerationError::MarkdownTooLarge);
    }
    output.push_str(value);
    Ok(())
}

fn build_suggested_filename(title: &str) -> String {
    let mut cleaned = String::new();
    let mut separator_pending = false;

    for character in title.chars() {
        if character.is_whitespace()
            || character.is_control()
            || matches!(
                character,
                '<' | '>' | ':' | '"' | '/' | '\\' | '|' | '?' | '*'
            )
        {
            separator_pending = !cleaned.is_empty();
            continue;
        }

        if separator_pending && !cleaned.ends_with('-') {
            cleaned.push('-');
        }
        separator_pending = false;
        cleaned.push(character);
    }

    let cleaned = cleaned.trim_matches([' ', '.', '-']);
    let component = if cleaned.is_empty() {
        "Untitled-session"
    } else {
        cleaned
    };
    let max_component_bytes =
        MAX_MARKDOWN_FILENAME_BYTES - FILENAME_PREFIX.len() - FILENAME_SUFFIX.len();
    let mut end = component.len().min(max_component_bytes);
    while !component.is_char_boundary(end) {
        end -= 1;
    }
    let component = component[..end].trim_end_matches([' ', '.', '-']);
    let component = if component.is_empty() {
        "Untitled-session"
    } else {
        component
    };

    format!("{FILENAME_PREFIX}{component}{FILENAME_SUFFIX}")
}

fn normalize_newlines(value: &str) -> String {
    value.replace("\r\n", "\n").replace('\r', "\n")
}

fn is_canonical_utc_timestamp(value: &str) -> bool {
    let bytes = value.as_bytes();
    if !(20..=30).contains(&bytes.len()) || !value.is_ascii() || bytes.last() != Some(&b'Z') {
        return false;
    }
    if bytes[4] != b'-'
        || bytes[7] != b'-'
        || bytes[10] != b'T'
        || bytes[13] != b':'
        || bytes[16] != b':'
    {
        return false;
    }
    for index in [0, 1, 2, 3, 5, 6, 8, 9, 11, 12, 14, 15, 17, 18] {
        if !bytes[index].is_ascii_digit() {
            return false;
        }
    }

    if bytes.len() > 20
        && (bytes[19] != b'.'
            || bytes[20..bytes.len() - 1].is_empty()
            || !bytes[20..bytes.len() - 1].iter().all(u8::is_ascii_digit))
    {
        return false;
    }

    let year = parse_decimal(&bytes[0..4]);
    let month = parse_decimal(&bytes[5..7]);
    let day = parse_decimal(&bytes[8..10]);
    let hour = parse_decimal(&bytes[11..13]);
    let minute = parse_decimal(&bytes[14..16]);
    let second = parse_decimal(&bytes[17..19]);
    let max_day = days_in_month(year, month);

    year != 0
        && max_day != 0
        && (1..=max_day).contains(&day)
        && hour <= 23
        && minute <= 59
        && is_supported_utc_second(year, month, day, hour, minute, second)
}

const fn is_supported_utc_second(
    year: u32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
    second: u32,
) -> bool {
    second <= 59
        || (second == 60
            && hour == 23
            && minute == 59
            && matches!(month, 6 | 12)
            && day == days_in_month(year, month))
}

fn parse_decimal(bytes: &[u8]) -> u32 {
    bytes
        .iter()
        .fold(0, |value, digit| value * 10 + u32::from(digit - b'0'))
}

const fn days_in_month(year: u32, month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if is_leap_year(year) => 29,
        2 => 28,
        _ => 0,
    }
}

const fn is_leap_year(year: u32) -> bool {
    year.is_multiple_of(4) && (!year.is_multiple_of(100) || year.is_multiple_of(400))
}
