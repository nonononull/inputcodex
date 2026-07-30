use std::fmt;

pub const MAX_LOCAL_SESSION_ID_BYTES: usize = 256;
pub const MAX_LOCAL_SESSION_TITLE_CHARS: usize = 256;
pub const MAX_LOCAL_SESSION_DIRECTORY_PAGE_SIZE: usize = 100;

#[derive(Clone, PartialEq, Eq)]
pub struct LocalSessionTitle {
    value: String,
    character_count: usize,
    truncated: bool,
}

impl LocalSessionTitle {
    #[must_use]
    pub fn from_raw(raw: &str) -> Option<Self> {
        let mut value = String::new();
        let mut character_count = 0;
        let mut pending_space = false;
        let mut truncated = false;

        for character in raw.chars() {
            if character.is_whitespace() {
                pending_space = !value.is_empty();
                continue;
            }
            if character.is_control() {
                continue;
            }

            if pending_space {
                if character_count + 1 >= MAX_LOCAL_SESSION_TITLE_CHARS {
                    truncated = true;
                    break;
                }
                value.push(' ');
                character_count += 1;
                pending_space = false;
            }

            if character_count >= MAX_LOCAL_SESSION_TITLE_CHARS {
                truncated = true;
                break;
            }

            value.push(character);
            character_count += 1;
        }

        (!value.is_empty()).then_some(Self {
            value,
            character_count,
            truncated,
        })
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.value
    }

    #[must_use]
    pub const fn was_truncated(&self) -> bool {
        self.truncated
    }
}

impl fmt::Debug for LocalSessionTitle {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("LocalSessionTitle")
            .field("character_count", &self.character_count)
            .field("truncated", &self.truncated)
            .finish()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalSessionDirectoryEntryError {
    EmptySessionId,
    InvalidSessionId,
    SessionIdTooLong,
}

#[derive(Clone, PartialEq, Eq)]
pub struct LocalSessionDirectoryEntry {
    session_id: String,
    display_title: Option<LocalSessionTitle>,
    archived: bool,
    updated_at_ms: Option<i64>,
}

impl LocalSessionDirectoryEntry {
    pub fn new(
        session_id: String,
        display_title: Option<LocalSessionTitle>,
        archived: bool,
        updated_at_ms: Option<i64>,
    ) -> Result<Self, LocalSessionDirectoryEntryError> {
        if session_id.trim().is_empty() {
            return Err(LocalSessionDirectoryEntryError::EmptySessionId);
        }
        if session_id.trim() != session_id
            || session_id.chars().any(char::is_whitespace)
            || session_id.chars().any(char::is_control)
        {
            return Err(LocalSessionDirectoryEntryError::InvalidSessionId);
        }
        if session_id.len() > MAX_LOCAL_SESSION_ID_BYTES {
            return Err(LocalSessionDirectoryEntryError::SessionIdTooLong);
        }

        Ok(Self {
            session_id,
            display_title,
            archived,
            updated_at_ms,
        })
    }

    #[must_use]
    pub fn session_id(&self) -> &str {
        &self.session_id
    }

    #[must_use]
    pub const fn display_title(&self) -> Option<&LocalSessionTitle> {
        self.display_title.as_ref()
    }

    #[must_use]
    pub const fn is_archived(&self) -> bool {
        self.archived
    }

    #[must_use]
    pub const fn updated_at_ms(&self) -> Option<i64> {
        self.updated_at_ms
    }
}

impl fmt::Debug for LocalSessionDirectoryEntry {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("LocalSessionDirectoryEntry")
            .field("has_title", &self.display_title.is_some())
            .field(
                "title_truncated",
                &self
                    .display_title
                    .as_ref()
                    .is_some_and(LocalSessionTitle::was_truncated),
            )
            .field("archived", &self.archived)
            .field("updated_at_ms", &self.updated_at_ms)
            .finish()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalSessionSourceCoverage {
    Complete,
    Partial,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalSessionSourceSummaryError {
    NoSources,
    CountMismatch,
    NoReadableSources,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LocalSessionSourceSummary {
    discovered: usize,
    readable: usize,
    failed: usize,
}

impl LocalSessionSourceSummary {
    pub const fn new(
        discovered: usize,
        readable: usize,
        failed: usize,
    ) -> Result<Self, LocalSessionSourceSummaryError> {
        if discovered == 0 {
            return Err(LocalSessionSourceSummaryError::NoSources);
        }
        if readable + failed != discovered {
            return Err(LocalSessionSourceSummaryError::CountMismatch);
        }
        if readable == 0 {
            return Err(LocalSessionSourceSummaryError::NoReadableSources);
        }

        Ok(Self {
            discovered,
            readable,
            failed,
        })
    }

    #[must_use]
    pub const fn discovered(self) -> usize {
        self.discovered
    }

    #[must_use]
    pub const fn readable(self) -> usize {
        self.readable
    }

    #[must_use]
    pub const fn failed(self) -> usize {
        self.failed
    }

    #[must_use]
    pub const fn coverage(self) -> LocalSessionSourceCoverage {
        if self.failed == 0 {
            LocalSessionSourceCoverage::Complete
        } else {
            LocalSessionSourceCoverage::Partial
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalSessionDirectoryPageError {
    EmptyEntries,
    InvalidLimit,
    TooManyEntries,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalSessionDirectoryPage {
    entries: Vec<LocalSessionDirectoryEntry>,
    offset: usize,
    limit: usize,
    has_more: bool,
    sources: LocalSessionSourceSummary,
}

impl LocalSessionDirectoryPage {
    pub fn new(
        entries: Vec<LocalSessionDirectoryEntry>,
        offset: usize,
        limit: usize,
        has_more: bool,
        sources: LocalSessionSourceSummary,
    ) -> Result<Self, LocalSessionDirectoryPageError> {
        if entries.is_empty() {
            return Err(LocalSessionDirectoryPageError::EmptyEntries);
        }
        if limit == 0 || limit > MAX_LOCAL_SESSION_DIRECTORY_PAGE_SIZE {
            return Err(LocalSessionDirectoryPageError::InvalidLimit);
        }
        if entries.len() > limit {
            return Err(LocalSessionDirectoryPageError::TooManyEntries);
        }

        Ok(Self {
            entries,
            offset,
            limit,
            has_more,
            sources,
        })
    }

    #[must_use]
    pub fn entries(&self) -> &[LocalSessionDirectoryEntry] {
        &self.entries
    }

    #[must_use]
    pub const fn offset(&self) -> usize {
        self.offset
    }

    #[must_use]
    pub const fn limit(&self) -> usize {
        self.limit
    }

    #[must_use]
    pub const fn has_more(&self) -> bool {
        self.has_more
    }

    #[must_use]
    pub const fn sources(&self) -> LocalSessionSourceSummary {
        self.sources
    }
}
