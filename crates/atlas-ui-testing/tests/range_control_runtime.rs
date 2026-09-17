//! Native Slint slider boundary callbacks through the Atlas wrapper.

#![allow(missing_docs)] // Generated fixture bindings are documented in Slint.

slint::include_modules!();

use slint::platform::software_renderer::{MinimalSoftwareWindow, RepaintBufferType};
use slint::platform::{
    Key, Platform, PlatformError, PointerEventButton, WindowAdapter, WindowEvent,
};
use slint::{ComponentHandle, LogicalPosition};
use std::rc::Rc;

struct FixturePlatform {
    window: Rc<MinimalSoftwareWindow>,
}

impl Platform for FixturePlatform {
    fn create_window_adapter(&self) -> Result<Rc<dyn WindowAdapter>, PlatformError> {
        Ok(self.window.clone())
    }
}

fn tap(fixture: &RangeControlRuntimeFixture, key: Key) {
    fixture
        .window()
        .dispatch_event(WindowEvent::KeyPressed { text: key.into() });
    fixture
        .window()
        .dispatch_event(WindowEvent::KeyReleased { text: key.into() });
}

#[test]
fn slider_reports_only_actual_changes_at_bounds() {
    slint::platform::set_platform(Box::new(FixturePlatform {
        window: MinimalSoftwareWindow::new(RepaintBufferType::ReusedBuffer),
    }))
    .expect("fixture platform");

    let fixture = RangeControlRuntimeFixture::new().expect("range fixture");
    fixture.show().expect("show fixture");
    fixture
        .window()
        .set_size(slint::LogicalSize::new(320.0, 120.0));
    fixture.window().take_snapshot().expect("render fixture");

    let position = LogicalPosition::new(140.0, 54.0);
    fixture
        .window()
        .dispatch_event(WindowEvent::PointerPressed {
            position,
            button: PointerEventButton::Left,
        });
    fixture
        .window()
        .dispatch_event(WindowEvent::PointerReleased {
            position,
            button: PointerEventButton::Left,
        });

    tap(&fixture, Key::End);
    assert!((fixture.get_current_value() - 100.0).abs() < f32::EPSILON);
    let changes_at_maximum = fixture.get_change_count();
    assert!(changes_at_maximum > 0);
    tap(&fixture, Key::End);
    tap(&fixture, Key::RightArrow);
    assert!((fixture.get_current_value() - 100.0).abs() < f32::EPSILON);
    assert_eq!(fixture.get_change_count(), changes_at_maximum);

    tap(&fixture, Key::Home);
    assert!(fixture.get_current_value().abs() < f32::EPSILON);
    assert_eq!(fixture.get_change_count(), changes_at_maximum + 1);
    tap(&fixture, Key::Home);
    tap(&fixture, Key::LeftArrow);
    assert!(fixture.get_current_value().abs() < f32::EPSILON);
    assert_eq!(fixture.get_change_count(), changes_at_maximum + 1);
}
