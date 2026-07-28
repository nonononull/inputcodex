use crate::ApplicationVersion;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StartupIntent {
    Default,
    ShowUpdate,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VersionStartupSnapshot {
    inputcodex_version: ApplicationVersion,
    startup_intent: StartupIntent,
}

impl VersionStartupSnapshot {
    #[must_use]
    pub const fn new(
        inputcodex_version: ApplicationVersion,
        startup_intent: StartupIntent,
    ) -> Self {
        Self {
            inputcodex_version,
            startup_intent,
        }
    }

    #[must_use]
    pub const fn inputcodex_version(&self) -> &ApplicationVersion {
        &self.inputcodex_version
    }

    #[must_use]
    pub const fn startup_intent(&self) -> StartupIntent {
        self.startup_intent
    }
}
