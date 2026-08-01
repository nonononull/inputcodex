#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WatcherPreference {
    EnabledByDefault,
    ExplicitlyDisabled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WatcherPreferenceObservation {
    preference: WatcherPreference,
}

impl WatcherPreferenceObservation {
    #[must_use]
    pub const fn new(preference: WatcherPreference) -> Self {
        Self { preference }
    }

    #[must_use]
    pub const fn preference(self) -> WatcherPreference {
        self.preference
    }
}
