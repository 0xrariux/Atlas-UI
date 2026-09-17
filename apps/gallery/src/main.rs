//! Native Atlas UI component gallery.

// Slint generates public Rust bindings from the declarative gallery. Their
// documentation lives in the .slint facade rather than generated Rust.
#![allow(missing_docs)]

slint::include_modules!();

use atlas_ui::core::tracks::{
    TrackAllocation, TrackConstraint, TrackWidthOverrides, allocate_tracks,
};
use slint::platform::software_renderer::{MinimalSoftwareWindow, RepaintBufferType};
use slint::platform::{Platform, PlatformError, WindowAdapter};
use slint::{Model, ModelRc, VecModel};
use std::cell::RefCell;
use std::rc::Rc;

struct CapturePlatform {
    window: Rc<MinimalSoftwareWindow>,
}

// Slint lengths are f32; integer logical pixels remain exactly representable
// through 2^24. The checked range makes these conversions intentional.
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
fn logical_pixels(value: f32) -> Option<u32> {
    (value.is_finite() && (0.0..=16_777_216.0).contains(&value)).then(|| value.round() as u32)
}

#[allow(clippy::cast_precision_loss)]
fn allocation_model(allocation: Option<TrackAllocation>) -> ModelRc<f32> {
    let widths: Vec<f32> = allocation
        .map(|allocation| {
            allocation
                .widths
                .into_iter()
                .map(|width| width as f32)
                .collect()
        })
        .unwrap_or_default();
    ModelRc::from(Rc::new(VecModel::from(widths)))
}

fn allocate_table_widths(
    viewport: f32,
    padding: f32,
    gap: f32,
    columns: &ModelRc<DataColumn>,
) -> ModelRc<f32> {
    let allocation = (|| {
        let tracks = columns
            .iter()
            .map(|column| {
                Some(TrackConstraint {
                    preferred: logical_pixels(column.width)?,
                    minimum: logical_pixels(column.min_width)?,
                    maximum: if column.max_width > 0.0 {
                        logical_pixels(column.max_width)?
                    } else {
                        u32::MAX
                    },
                    grow: logical_pixels(column.grow)?,
                })
            })
            .collect::<Option<Vec<_>>>()?;
        allocate_tracks(
            logical_pixels(viewport)?,
            logical_pixels(padding)?,
            logical_pixels(gap)?,
            &tracks,
        )
        .ok()
    })();

    // An empty model lets AtlasDataTable use its native layout constraints.
    allocation_model(allocation)
}

fn allocate_document_widths(
    viewport: f32,
    padding: f32,
    minimum: f32,
    columns: &ModelRc<DocumentTableColumn>,
) -> ModelRc<f32> {
    let allocation = (|| {
        let minimum = logical_pixels(minimum)?;
        let tracks = columns
            .iter()
            .map(|column| {
                let preferred = logical_pixels(column.width)?;
                Some(TrackConstraint {
                    preferred,
                    minimum: preferred.min(minimum),
                    maximum: u32::MAX,
                    grow: 1,
                })
            })
            .collect::<Option<Vec<_>>>()?;
        allocate_tracks(
            logical_pixels(viewport)?,
            logical_pixels(padding)?,
            0,
            &tracks,
        )
        .ok()
    })();
    allocation_model(allocation)
}

fn allocate_pair_widths(viewport: f32, padding: f32, gap: f32) -> ModelRc<f32> {
    let tracks = [
        TrackConstraint {
            preferred: 200,
            minimum: 120,
            maximum: u32::MAX,
            grow: 1,
        },
        TrackConstraint {
            preferred: 300,
            minimum: 120,
            maximum: u32::MAX,
            grow: 2,
        },
    ];
    allocation_model((|| {
        allocate_tracks(
            logical_pixels(viewport)?,
            logical_pixels(padding)?,
            logical_pixels(gap)?,
            &tracks,
        )
        .ok()
    })())
}

fn resize_gallery_column(
    gallery: &Gallery,
    overrides: &mut TrackWidthOverrides<String>,
    rich: bool,
    id: &str,
    requested: f32,
) {
    let Some(requested) = logical_pixels(requested) else {
        return;
    };
    let mut columns: Vec<DataColumn> = if rich {
        gallery.get_access_columns().iter().collect()
    } else {
        gallery.get_environments_columns().iter().collect()
    };
    let Some(column) = columns
        .iter_mut()
        .find(|column| column.id.as_str() == id && column.resizable)
    else {
        return;
    };
    let Some(minimum) = logical_pixels(column.min_width) else {
        return;
    };
    let Some(maximum) = (if column.max_width > 0.0 {
        logical_pixels(column.max_width)
    } else {
        Some(u32::MAX)
    }) else {
        return;
    };
    let track = TrackConstraint {
        preferred: logical_pixels(column.width).unwrap_or(minimum),
        minimum,
        maximum,
        grow: logical_pixels(column.grow).unwrap_or(0),
    };
    let Ok(width) = overrides.resize(id.to_owned(), requested, track) else {
        return;
    };
    let Ok(width) = u16::try_from(width) else {
        return;
    };
    column.width = f32::from(width);
    let model = ModelRc::from(Rc::new(VecModel::from(columns)));
    if rich {
        gallery.set_access_columns(model);
    } else {
        gallery.set_environments_columns(model);
    }
}

impl Platform for CapturePlatform {
    fn create_window_adapter(&self) -> Result<Rc<dyn WindowAdapter>, PlatformError> {
        Ok(self.window.clone())
    }
}

fn install_table_callbacks(gallery: &Gallery) {
    gallery.on_allocate_table_widths(|viewport, padding, gap, columns| {
        allocate_table_widths(viewport, padding, gap, &columns)
    });
    gallery.on_allocate_document_widths(|viewport, padding, minimum, columns| {
        allocate_document_widths(viewport, padding, minimum, &columns)
    });
    gallery.on_allocate_pair_widths(allocate_pair_widths);
    let resize_state = Rc::new(RefCell::new((
        TrackWidthOverrides::new(),
        TrackWidthOverrides::new(),
    )));
    let resize_gallery = gallery.as_weak();
    gallery.on_resize_table_column(move |rich, id, width| {
        if let Some(gallery) = resize_gallery.upgrade() {
            let mut state = resize_state.borrow_mut();
            let overrides = if rich { &mut state.1 } else { &mut state.0 };
            resize_gallery_column(&gallery, overrides, rich, id.as_str(), width);
        }
    });
}

fn main() -> Result<(), slint::PlatformError> {
    let capture_path = std::env::var_os("ATLAS_UI_GALLERY_CAPTURE");
    if capture_path.is_some() {
        slint::platform::set_platform(Box::new(CapturePlatform {
            window: MinimalSoftwareWindow::new(RepaintBufferType::ReusedBuffer),
        }))
        .map_err(PlatformError::SetPlatformError)?;
    }
    let gallery = Gallery::new()?;
    install_table_callbacks(&gallery);
    if let Some(path) = capture_path {
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
        std::thread::sleep(std::time::Duration::from_millis(delay));
        slint::platform::update_timers_and_animations();
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
        Ok(())
    } else {
        gallery.run()
    }
}

#[cfg(test)]
mod tests {
    use super::{CapturePlatform, Gallery, TrackWidthOverrides, resize_gallery_column};
    use slint::Model;
    use slint::platform::software_renderer::{MinimalSoftwareWindow, RepaintBufferType};

    #[test]
    fn gallery_column_resize_updates_matching_id_within_constraints() {
        slint::platform::set_platform(Box::new(CapturePlatform {
            window: MinimalSoftwareWindow::new(RepaintBufferType::ReusedBuffer),
        }))
        .expect("fixture platform");
        let gallery = Gallery::new().expect("gallery");
        let original_name = gallery
            .get_environments_columns()
            .iter()
            .find(|column| column.id.as_str() == "name")
            .unwrap()
            .width;
        let mut overrides = TrackWidthOverrides::new();
        resize_gallery_column(&gallery, &mut overrides, false, "region", 10_000.0);
        let columns: Vec<_> = gallery.get_environments_columns().iter().collect();
        let region = columns
            .iter()
            .find(|column| column.id.as_str() == "region")
            .unwrap();
        assert!((region.width - region.max_width).abs() < 0.01);
        let name = columns
            .iter()
            .find(|column| column.id.as_str() == "name")
            .unwrap();
        assert!((name.width - original_name).abs() < 0.01);
        let before_status = columns
            .iter()
            .find(|column| column.id.as_str() == "status")
            .unwrap()
            .width;
        resize_gallery_column(&gallery, &mut overrides, false, "status", 10_000.0);
        let status = gallery
            .get_environments_columns()
            .iter()
            .find(|column| column.id.as_str() == "status")
            .unwrap();
        assert!((status.width - before_status).abs() < 0.01);
    }
}
