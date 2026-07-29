#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DiagnosticLogObservation {
    file_size_bytes: u64,
    sampled_record_count: usize,
    valid_object_record_count: usize,
    malformed_record_count: usize,
    truncated: bool,
    partial_record_discarded: bool,
}

impl DiagnosticLogObservation {
    #[must_use]
    pub const fn new(
        file_size_bytes: u64,
        valid_object_record_count: usize,
        malformed_record_count: usize,
        truncated: bool,
        partial_record_discarded: bool,
    ) -> Self {
        Self {
            file_size_bytes,
            sampled_record_count: valid_object_record_count + malformed_record_count,
            valid_object_record_count,
            malformed_record_count,
            truncated,
            partial_record_discarded,
        }
    }

    #[must_use]
    pub const fn file_size_bytes(self) -> u64 {
        self.file_size_bytes
    }

    #[must_use]
    pub const fn sampled_record_count(self) -> usize {
        self.sampled_record_count
    }

    #[must_use]
    pub const fn valid_object_record_count(self) -> usize {
        self.valid_object_record_count
    }

    #[must_use]
    pub const fn malformed_record_count(self) -> usize {
        self.malformed_record_count
    }

    #[must_use]
    pub const fn truncated(self) -> bool {
        self.truncated
    }

    #[must_use]
    pub const fn partial_record_discarded(self) -> bool {
        self.partial_record_discarded
    }
}
