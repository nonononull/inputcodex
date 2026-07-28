#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SettingsDocumentObservation {
    top_level_entry_count: usize,
}

impl SettingsDocumentObservation {
    #[must_use]
    pub const fn new(top_level_entry_count: usize) -> Self {
        Self {
            top_level_entry_count,
        }
    }

    #[must_use]
    pub const fn top_level_entry_count(self) -> usize {
        self.top_level_entry_count
    }
}
