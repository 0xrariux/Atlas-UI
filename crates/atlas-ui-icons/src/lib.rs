//! Icon assets and package metadata for Atlas UI.

use std::path::{Path, PathBuf};

/// Returns the directory containing the public Slint icon facade.
#[must_use]
pub fn ui_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("ui")
}

/// Public Slint facade filename.
pub const FACADE: &str = "icons.slint";
