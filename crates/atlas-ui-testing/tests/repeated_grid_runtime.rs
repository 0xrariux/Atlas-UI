//! Slint 1.18 repeated and conditional grid rows retain measured height.

#![allow(missing_docs)] // Generated fixture bindings are documented in Slint.

slint::include_modules!();

use slint::platform::software_renderer::{MinimalSoftwareWindow, RepaintBufferType};
use slint::platform::{Platform, PlatformError, WindowAdapter};
use slint::{ComponentHandle, Model, ModelRc, SharedString, VecModel};
use std::rc::Rc;

struct FixturePlatform {
    window: Rc<MinimalSoftwareWindow>,
}

impl Platform for FixturePlatform {
    fn create_window_adapter(&self) -> Result<Rc<dyn WindowAdapter>, PlatformError> {
        Ok(self.window.clone())
    }
}

#[test]
fn repeated_and_conditional_rows_resize_after_model_mutation() {
    slint::platform::set_platform(Box::new(FixturePlatform {
        window: MinimalSoftwareWindow::new(RepaintBufferType::ReusedBuffer),
    }))
    .expect("fixture platform");

    let fixture = RepeatedGridRuntimeFixture::new().expect("grid fixture");
    let rows = Rc::new(VecModel::from(vec![SharedString::from("Short")]));
    fixture.set_rows(ModelRc::from(rows.clone()));
    fixture.show().expect("show fixture");
    fixture
        .window()
        .set_size(slint::LogicalSize::new(280.0, 400.0));
    fixture.window().take_snapshot().expect("render short row");
    let short = fixture.get_grid_height();

    rows.set_row_data(0, SharedString::from("A long repeated grid cell must wrap across several lines after its model changes under a fixed, narrow grid width."));
    fixture
        .window()
        .take_snapshot()
        .expect("render wrapped row");
    let wrapped = fixture.get_grid_height();
    assert!(wrapped > short, "wrapped cell did not increase row height");

    rows.push(SharedString::from("Another row"));
    fixture
        .window()
        .take_snapshot()
        .expect("render inserted row");
    let inserted = fixture.get_grid_height();
    assert!(
        inserted > wrapped,
        "inserted row did not increase grid height"
    );

    fixture.set_show_summary(true);
    fixture
        .window()
        .take_snapshot()
        .expect("render conditional span");
    let with_summary = fixture.get_grid_height();
    assert!(
        with_summary > inserted,
        "conditional span did not add a row"
    );

    fixture.set_show_summary(false);
    rows.remove(1);
    rows.set_row_data(0, SharedString::from("Short"));
    fixture
        .window()
        .take_snapshot()
        .expect("render restored grid");
    assert!((fixture.get_grid_height() - short).abs() <= 0.01);
}
