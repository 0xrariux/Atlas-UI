//! Shared visual and interaction primitives for Atlas UI.

use std::path::{Path, PathBuf};

/// Returns the directory containing the public Slint core facade.
#[must_use]
pub fn ui_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("ui")
}

/// Returns the token facade used by this package.
#[must_use]
pub fn token_ui_path() -> PathBuf {
    atlas_ui_tokens::ui_path()
}

/// Public Slint facade filename.
pub const FACADE: &str = "core.slint";
