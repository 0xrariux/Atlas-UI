//! Runtime visibility after an anchored tooltip loses eligibility.

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

#[test]
fn tooltip_closes_when_its_anchor_becomes_ineligible() {
    slint::platform::set_platform(Box::new(FixturePlatform {
        window: MinimalSoftwareWindow::new(RepaintBufferType::ReusedBuffer),
    }))
    .expect("fixture platform");

    let fixture = TooltipRuntimeFixture::new().expect("tooltip fixture");
    fixture.show().expect("show fixture");
    fixture
        .window()
        .set_size(slint::LogicalSize::new(480.0, 320.0));
    fixture.window().take_snapshot().expect("render tooltip");
    assert!(fixture.get_open());

    fixture.set_anchor_eligible(false);
    fixture
        .window()
        .take_snapshot()
        .expect("render anchor loss");
    assert!(!fixture.get_open());

    fixture.set_anchor_eligible(true);
    fixture.set_force_open(false);
    fixture
        .window()
        .take_snapshot()
        .expect("render closed tooltip");
    assert!(!fixture.get_open());
}
