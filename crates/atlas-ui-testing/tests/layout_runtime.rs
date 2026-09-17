//! Runtime geometry checks for Slint 1.18 and Atlas responsive compositions.

#![allow(missing_docs)] // Generated fixture bindings are documented in Slint.

slint::include_modules!();

use slint::platform::software_renderer::{MinimalSoftwareWindow, RepaintBufferType};
use slint::platform::{Platform, PlatformError, WindowAdapter};
use slint::{ComponentHandle, Model, ModelRc, VecModel};
use std::rc::Rc;

struct FixturePlatform {
    window: Rc<MinimalSoftwareWindow>,
}

impl Platform for FixturePlatform {
    fn create_window_adapter(&self) -> Result<Rc<dyn WindowAdapter>, PlatformError> {
        Ok(self.window.clone())
    }
}

fn item(text: &str) -> DocumentListItem {
    DocumentListItem {
        marker: "1.".into(),
        text: text.into(),
    }
}

fn render(fixture: &LayoutRuntimeFixture, width: f32) {
    fixture
        .window()
        .set_size(slint::LogicalSize::new(width, 760.0));
    slint::platform::update_timers_and_animations();
    fixture
        .window()
        .take_snapshot()
        .expect("render live layout");
}

fn near(actual: f32, expected: f32) {
    assert!(
        (actual - expected).abs() <= 1.0,
        "expected {expected:.2}px, got {actual:.2}px"
    );
}

#[test]
fn resize_and_model_mutations_recompute_real_atlas_geometry() {
    slint::platform::set_platform(Box::new(FixturePlatform {
        window: MinimalSoftwareWindow::new(RepaintBufferType::ReusedBuffer),
    }))
    .expect("fixture platform");

    let fixture = LayoutRuntimeFixture::new().expect("layout fixture");
    let items = Rc::new(VecModel::from(vec![item("Short list item")]));
    fixture.set_items(ModelRc::from(items.clone()));
    fixture.show().expect("show fixture");

    render(&fixture, 1360.0);
    assert!(!fixture.get_stacked());
    assert_eq!(
        fixture.get_grid_columns(),
        2,
        "second width = {}",
        fixture.get_second_width()
    );
    assert!(fixture.get_first_width() >= 180.0);
    assert!(fixture.get_first_width() <= 620.0);
    assert!(fixture.get_second_width() >= 180.0);
    assert!(fixture.get_second_width() <= 620.0);
    assert!(
        fixture.get_first_x() + fixture.get_first_width() <= fixture.get_second_x() + 1.0,
        "wide panes overlap"
    );
    near(fixture.get_first_y(), fixture.get_second_y());

    render(&fixture, 420.0);
    assert!(fixture.get_stacked());
    assert_eq!(fixture.get_grid_columns(), 1);
    assert!(
        fixture.get_first_y() + fixture.get_first_height() <= fixture.get_second_y() + 1.0,
        "stacked panes overlap"
    );
    near(fixture.get_first_x(), fixture.get_second_x());
    let short_height = fixture.get_list_height();

    let long_text = "A repeated document row must grow when a long explanation wraps over several lines at a narrow viewport. This text intentionally continues through many words so the runtime layout has to measure the assigned width rather than an unconstrained preferred width.";
    items.set_row_data(0, item(long_text));
    render(&fixture, 420.0);
    let wrapped_height = fixture.get_list_height();
    assert!(
        wrapped_height > short_height + 20.0,
        "wrapped row did not grow: short={short_height}, wrapped={wrapped_height}"
    );

    items.set_row_data(0, item("Short list item"));
    render(&fixture, 420.0);
    near(fixture.get_list_height(), short_height);

    items.insert(1, item(long_text));
    render(&fixture, 420.0);
    assert!(fixture.get_list_height() > short_height + 20.0);
    items.remove(1);
    render(&fixture, 420.0);
    near(fixture.get_list_height(), short_height);

    render(&fixture, 1360.0);
    assert!(!fixture.get_stacked());
    assert_eq!(fixture.get_grid_columns(), 2);
    assert!(
        fixture.get_first_x() + fixture.get_first_width() <= fixture.get_second_x() + 1.0,
        "wide layout did not recover after narrowing"
    );
}
