//! Public component package for Atlas UI.

use std::path::{Path, PathBuf};

/// Returns the directory containing the public Slint component facade.
#[must_use]
pub fn ui_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("ui")
}

/// All lower layers consumed by the Slint facade.
#[must_use]
pub fn dependency_ui_paths() -> [PathBuf; 3] {
    [
        atlas_ui_tokens::ui_path(),
        atlas_ui_core::ui_path(),
        atlas_ui_icons::ui_path(),
    ]
}

/// Public Slint facade filename.
pub const FACADE: &str = "components.slint";

/// Non-responsive preview Slint facade filename.
pub const NONRESPONSIVE_PREVIEW_FACADE: &str = "preview-nonresponsive.slint";
