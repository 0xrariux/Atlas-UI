//! Minimal native consumer proving generated Rust bindings and controlled intent.

#![allow(missing_docs)]

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let app = App::new()?;
    app.global::<AtlasSettings>()
        .set_theme_mode(ThemeMode::System);
    app.global::<AtlasSettings>()
        .set_typography_scale(TypographyScale::Normal);
    app.on_create_requested(|| {
        // A real host would call its application service here. The example
        // deliberately performs no filesystem, network or persistence action.
    });
    app.run()
}
