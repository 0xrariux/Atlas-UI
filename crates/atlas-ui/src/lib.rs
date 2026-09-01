//! Main Rust entry point for the Atlas UI design system.
//!
//! Use this crate as a build dependency to configure Slint's named Atlas
//! libraries without relying on a monorepo directory layout.

use std::{collections::HashMap, path::PathBuf};

pub use atlas_ui_components as components;
pub use atlas_ui_core as core;
#[cfg(feature = "documents")]
pub use atlas_ui_documents as documents;
pub use atlas_ui_icons as icons;
pub use atlas_ui_tokens as tokens;

/// Named Slint library containing the stable, preview, non-responsive preview,
/// and aggregate facades.
pub const COMPONENT_LIBRARY: &str = "atlas-ui";

/// Returns registry-safe named library paths for `slint-build`.
///
/// Pass the result to
/// [`slint_build::CompilerConfiguration::with_library_paths`] from `build.rs`.
/// Consumer markup can then import `@atlas-ui/stable.slint`,
/// `@atlas-ui/preview-nonresponsive.slint`, `@atlas-ui/preview.slint`, or
/// `@atlas-ui/components.slint`.
#[must_use]
pub fn slint_library_paths() -> HashMap<String, PathBuf> {
    HashMap::from([
        (COMPONENT_LIBRARY.to_owned(), components::ui_path()),
        ("atlas-ui-core".to_owned(), core::ui_path()),
        ("atlas-ui-icons".to_owned(), icons::ui_path()),
        ("atlas-ui-tokens".to_owned(), tokens::ui_path()),
    ])
}

/// Returns the stable Slint facade for tools that require a concrete path.
#[must_use]
pub fn stable_slint_path() -> PathBuf {
    components::ui_path().join("stable.slint")
}

/// Returns the preview Slint facade for tools that require a concrete path.
#[must_use]
pub fn preview_slint_path() -> PathBuf {
    components::ui_path().join("preview.slint")
}

/// Returns the non-responsive preview Slint facade for tools that require a
/// concrete path. This facade does not require experimental Slint features.
#[must_use]
pub fn preview_nonresponsive_slint_path() -> PathBuf {
    components::ui_path().join("preview-nonresponsive.slint")
}

#[cfg(test)]
mod tests {
    use super::{
        preview_nonresponsive_slint_path, preview_slint_path, slint_library_paths,
        stable_slint_path,
    };

    #[test]
    fn exported_slint_paths_exist() {
        assert!(stable_slint_path().is_file());
        assert!(preview_slint_path().is_file());
        assert!(preview_nonresponsive_slint_path().is_file());
        assert!(slint_library_paths().values().all(|path| path.is_dir()));
    }
}
