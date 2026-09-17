//! Local container metrics follow nested widths rather than the window alone.

#![allow(missing_docs)] // Generated fixture bindings are documented in Slint.

slint::include_modules!();

use slint::ComponentHandle;
use slint::platform::software_renderer::{MinimalSoftwareWindow, RepaintBufferType};
use slint::platform::{Platform, PlatformError, WindowAdapter};
use std::rc::Rc;

struct FixturePlatform {
    window: Rc<MinimalSoftwareWindow>,
}

impl Platform for FixturePlatform {
    fn create_window_adapter(&self) -> Result<Rc<dyn WindowAdapter>, PlatformError> {
        Ok(self.window.clone())
    }
}

fn near(actual: f32, expected: f32) {
    assert!(
        (actual - expected).abs() <= 0.01,
        "expected {expected:.2}px, got {actual:.2}px"
    );
}

#[test]
fn nested_container_size_classes_follow_local_content_width() {
    slint::platform::set_platform(Box::new(FixturePlatform {
        window: MinimalSoftwareWindow::new(RepaintBufferType::ReusedBuffer),
    }))
    .expect("fixture platform");

    let fixture = ContainerRuntimeFixture::new().expect("container fixture");
    fixture.show().expect("show fixture");

    fixture
        .window()
        .set_size(slint::LogicalSize::new(1200.0, 260.0));
    fixture
        .window()
        .take_snapshot()
        .expect("render wide fixture");
    near(fixture.get_outer_content_width(), 1128.0);
    near(fixture.get_inner_content_width(), 548.0);
    near(fixture.get_inner_received_viewport(), 1128.0);
    near(fixture.get_inner_content_viewport(), 548.0);
    assert!(fixture.get_outer_wide());
    assert!(fixture.get_inner_compact());

    fixture
        .window()
        .set_size(slint::LogicalSize::new(700.0, 260.0));
    fixture
        .window()
        .take_snapshot()
        .expect("render standard fixture");
    near(fixture.get_outer_content_width(), 628.0);
    assert!(!fixture.get_outer_wide());
    assert!(!fixture.get_outer_compact());
    assert!(fixture.get_inner_compact());

    fixture
        .window()
        .set_size(slint::LogicalSize::new(500.0, 260.0));
    fixture
        .window()
        .take_snapshot()
        .expect("render compact fixture");
    near(fixture.get_outer_content_width(), 428.0);
    assert!(fixture.get_outer_compact());
}
