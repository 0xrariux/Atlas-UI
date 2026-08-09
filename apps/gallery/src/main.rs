//! Native Atlas UI component gallery.

// Slint generates public Rust bindings from the declarative gallery. Their
// documentation lives in the .slint facade rather than generated Rust.
#![allow(missing_docs)]

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let gallery = Gallery::new()?;
    if let Some(path) = std::env::var_os("ATLAS_UI_GALLERY_CAPTURE") {
        gallery.set_preview_page(
            std::env::var("ATLAS_UI_GALLERY_PAGE")
                .unwrap_or_else(|_| "foundations".into())
                .into(),
        );
        gallery.set_show_grid(std::env::var_os("ATLAS_UI_GALLERY_GRID").is_some());
        if std::env::var_os("ATLAS_UI_GALLERY_LIGHT").is_some() {
            gallery
                .global::<AtlasSettings>()
                .set_theme_mode(ThemeMode::Light);
        }
        if std::env::var_os("ATLAS_UI_GALLERY_SYSTEM_THEME").is_some() {
            gallery
                .global::<AtlasSettings>()
                .set_theme_mode(ThemeMode::System);
            gallery.global::<AtlasSettings>().set_system_dark(
                std::env::var("ATLAS_UI_GALLERY_SYSTEM_DARK").as_deref() == Ok("1"),
            );
        }
        match std::env::var("ATLAS_UI_GALLERY_DENSITY").as_deref() {
            Ok("compact") => gallery
                .global::<AtlasSettings>()
                .set_density(Density::Compact),
            Ok("comfortable") => gallery
                .global::<AtlasSettings>()
                .set_density(Density::Comfortable),
            _ => gallery
                .global::<AtlasSettings>()
                .set_density(Density::Normal),
        }
        if std::env::var("ATLAS_UI_GALLERY_MOTION").as_deref() == Ok("reduced") {
            gallery
                .global::<AtlasSettings>()
                .set_motion(MotionPreference::Reduced);
        }
        match std::env::var("ATLAS_UI_GALLERY_TYPOGRAPHY_SCALE").as_deref() {
            Ok("compact") => gallery
                .global::<AtlasSettings>()
                .set_typography_scale(TypographyScale::Compact),
            Ok("large") => gallery
                .global::<AtlasSettings>()
                .set_typography_scale(TypographyScale::Large),
            _ => gallery
                .global::<AtlasSettings>()
                .set_typography_scale(TypographyScale::Normal),
        }
        if let (Ok(width), Ok(height)) = (
            std::env::var("ATLAS_UI_GALLERY_WIDTH"),
            std::env::var("ATLAS_UI_GALLERY_HEIGHT"),
        ) {
            let width = width.parse::<f32>().expect("valid logical capture width");
            let height = height.parse::<f32>().expect("valid logical capture height");
            gallery
                .window()
                .set_size(slint::LogicalSize::new(width, height));
        }
        let gallery_weak = gallery.as_weak();
        gallery.show()?;
        let delay = std::env::var("ATLAS_UI_GALLERY_DELAY_MS")
            .ok()
            .and_then(|value| value.parse().ok())
            .unwrap_or(250);
        slint::Timer::single_shot(std::time::Duration::from_millis(delay), move || {
            let gallery = gallery_weak.upgrade().expect("gallery remains alive");
            let pixels = gallery.window().take_snapshot().expect("gallery snapshot");
            image::save_buffer(
                &path,
                pixels.as_bytes(),
                pixels.width(),
                pixels.height(),
                image::ColorType::Rgba8,
            )
            .expect("write gallery snapshot");
            slint::quit_event_loop().expect("quit capture loop");
        });
        slint::run_event_loop()
    } else {
        gallery.run()
    }
}
