//! Runtime keyboard selection and dismissal for the preview combobox.

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

fn tap(fixture: &ComboboxRuntimeFixture, key: Key) {
    fixture
        .window()
        .dispatch_event(WindowEvent::KeyPressed { text: key.into() });
    fixture
        .window()
        .dispatch_event(WindowEvent::KeyReleased { text: key.into() });
}

fn click(fixture: &ComboboxRuntimeFixture, x: f32, y: f32) {
    let position = LogicalPosition::new(x, y);
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
}

#[test]
fn combobox_selects_by_keyboard_and_dismisses_when_disabled() {
    slint::platform::set_platform(Box::new(FixturePlatform {
        window: MinimalSoftwareWindow::new(RepaintBufferType::ReusedBuffer),
    }))
    .expect("fixture platform");

    let fixture = ComboboxRuntimeFixture::new().expect("combobox fixture");
    fixture.show().expect("show fixture");
    fixture
        .window()
        .set_size(slint::LogicalSize::new(480.0, 320.0));
    fixture.window().take_snapshot().expect("render fixture");
    fixture.invoke_focus_combobox();

    tap(&fixture, Key::Return);
    assert!(fixture.get_open());
    tap(&fixture, Key::DownArrow);
    tap(&fixture, Key::Return);
    assert!(!fixture.get_open());
    assert_eq!(fixture.get_selected_id(), "two");
    assert_eq!(fixture.get_selections(), 1);

    tap(&fixture, Key::Return);
    assert!(fixture.get_open());
    tap(&fixture, Key::Escape);
    assert!(!fixture.get_open());

    tap(&fixture, Key::Return);
    assert!(fixture.get_open());
    tap(&fixture, Key::Tab);
    assert!(!fixture.get_open());
    assert_eq!(fixture.get_traversals(), 1);

    fixture.invoke_focus_combobox();
    tap(&fixture, Key::Return);
    assert!(fixture.get_open());
    fixture.set_enabled(false);
    fixture
        .window()
        .take_snapshot()
        .expect("settle disabled anchor");
    assert!(!fixture.get_open());

    fixture.set_enabled(true);
    fixture.set_menu_x(100.0);
    fixture.set_menu_y(130.0);
    fixture.invoke_focus_combobox();
    tap(&fixture, Key::Return);
    assert!(fixture.get_open());
    fixture
        .window()
        .take_snapshot()
        .expect("render repositioned menu");
    click(&fixture, 150.0, 195.0);
    assert_eq!(fixture.get_selected_id(), "two");
    assert_eq!(fixture.get_selections(), 2);
    assert!(!fixture.get_open());

    fixture.invoke_focus_combobox();
    fixture.set_active_index(1);
    tap(&fixture, Key::Return);
    assert_eq!(fixture.get_active_index(), 1);
    fixture.set_single_option(true);
    fixture
        .window()
        .take_snapshot()
        .expect("settle shortened menu");
    assert_eq!(fixture.get_active_index(), 0);
    assert!(fixture.get_open());
    fixture.set_empty_options(true);
    fixture.window().take_snapshot().expect("settle empty menu");
    assert!(!fixture.get_open());

    fixture.set_empty_options(false);
    fixture.set_anchor_eligible(false);
    fixture.invoke_focus_combobox();
    tap(&fixture, Key::Return);
    assert!(!fixture.get_open());
}
