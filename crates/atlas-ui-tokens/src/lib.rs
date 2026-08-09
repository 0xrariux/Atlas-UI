//! Design-token assets and package metadata for Atlas UI.

use std::path::{Path, PathBuf};

/// Returns the directory containing the public Slint token facade.
#[must_use]
pub fn ui_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("ui")
}

/// Public Slint facade filename.
pub const FACADE: &str = "tokens.slint";

/// Persisted theme choice, independent of platform detection and Slint.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ThemePreference {
    /// Follow the current operating-system appearance.
    #[default]
    System,
    /// Force the dark theme.
    Dark,
    /// Force the light theme.
    Light,
}

/// Host-provided preference persistence port.
pub trait ThemePreferenceStore {
    /// Loads the last preference, if any.
    ///
    /// # Errors
    ///
    /// Returns a host-defined error when persistence cannot be read.
    fn load(&self) -> Result<Option<ThemePreference>, String>;

    /// Saves a preference only after an explicit user request.
    ///
    /// # Errors
    ///
    /// Returns a host-defined error when persistence cannot be written.
    fn save(&mut self, preference: ThemePreference) -> Result<(), String>;
}

/// Resolved theme that components can apply.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResolvedTheme {
    /// Dark semantic palette.
    Dark,
    /// Light semantic palette.
    Light,
}

/// Pure controller for preference and live system appearance updates.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ThemeController {
    preference: ThemePreference,
    system_dark: bool,
}

impl ThemeController {
    /// Creates a controller from an explicit initial state.
    #[must_use]
    pub const fn new(preference: ThemePreference, system_dark: bool) -> Self {
        Self {
            preference,
            system_dark,
        }
    }

    /// Returns the selected preference.
    #[must_use]
    pub const fn preference(self) -> ThemePreference {
        self.preference
    }

    /// Resolves `System` against the latest platform appearance.
    #[must_use]
    pub const fn resolved(self) -> ResolvedTheme {
        match self.preference {
            ThemePreference::Dark => ResolvedTheme::Dark,
            ThemePreference::Light => ResolvedTheme::Light,
            ThemePreference::System => {
                if self.system_dark {
                    ResolvedTheme::Dark
                } else {
                    ResolvedTheme::Light
                }
            }
        }
    }

    /// Updates a preference in memory. Persistence remains an explicit host call.
    pub fn set_preference(&mut self, preference: ThemePreference) {
        self.preference = preference;
    }

    /// Applies a live platform appearance notification.
    pub fn update_system_dark(&mut self, system_dark: bool) {
        self.system_dark = system_dark;
    }
}

#[cfg(test)]
mod tests {
    use super::{ResolvedTheme, ThemeController, ThemePreference, ThemePreferenceStore};

    #[derive(Default)]
    struct FixtureStore(Option<ThemePreference>);

    impl ThemePreferenceStore for FixtureStore {
        fn load(&self) -> Result<Option<ThemePreference>, String> {
            Ok(self.0)
        }
        fn save(&mut self, preference: ThemePreference) -> Result<(), String> {
            self.0 = Some(preference);
            Ok(())
        }
    }

    #[test]
    fn system_theme_updates_live_without_overwriting_preference() {
        let mut controller = ThemeController::new(ThemePreference::System, false);
        assert_eq!(controller.resolved(), ResolvedTheme::Light);
        controller.update_system_dark(true);
        assert_eq!(controller.resolved(), ResolvedTheme::Dark);
        assert_eq!(controller.preference(), ThemePreference::System);
    }

    #[test]
    fn persistence_is_explicit_and_replaceable() {
        let mut store = FixtureStore::default();
        assert_eq!(store.load().unwrap(), None);
        store.save(ThemePreference::Dark).unwrap();
        assert_eq!(store.load().unwrap(), Some(ThemePreference::Dark));
    }
}
