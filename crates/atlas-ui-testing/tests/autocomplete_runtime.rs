//! Live editing and keyboard selection while suggestions remain visible.

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

fn tap(fixture: &AutocompleteRuntimeFixture, text: impl Into<slint::SharedString>) {
    let text = text.into();
    fixture
        .window()
        .dispatch_event(WindowEvent::KeyPressed { text: text.clone() });
    fixture
        .window()
        .dispatch_event(WindowEvent::KeyReleased { text });
}

fn click(fixture: &AutocompleteRuntimeFixture, x: f32, y: f32) {
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
fn autocomplete_keeps_editing_focus_and_selects_suggestions() {
    slint::platform::set_platform(Box::new(FixturePlatform {
        window: MinimalSoftwareWindow::new(RepaintBufferType::ReusedBuffer),
    }))
    .expect("fixture platform");

    let fixture = AutocompleteRuntimeFixture::new().expect("autocomplete fixture");
    fixture.show().expect("show fixture");
    fixture
        .window()
        .set_size(slint::LogicalSize::new(480.0, 320.0));
    fixture.window().take_snapshot().expect("render fixture");
    fixture.invoke_focus_input();
    assert!(fixture.get_input_focused());

    tap(&fixture, "a");
    assert_eq!(fixture.get_query(), "a");
    assert!(fixture.get_suggestions_open());
    assert!(fixture.get_input_focused());
    tap(&fixture, "b");
    assert_eq!(fixture.get_query(), "ab");
    assert_eq!(fixture.get_edits(), 2);
    assert!(fixture.get_input_focused());

    tap(&fixture, Key::DownArrow);
    assert_eq!(fixture.get_active_index(), 1);
    assert!(fixture.get_input_focused());
    tap(&fixture, Key::Return);
    assert_eq!(fixture.get_selected_id(), "beta");
    assert_eq!(fixture.get_query(), "Beta");
    assert!(!fixture.get_suggestions_open());
    assert!(fixture.get_input_focused());

    fixture.set_suggestions_open(true);
    tap(&fixture, Key::Escape);
    assert!(!fixture.get_suggestions_open());
    assert!(fixture.get_input_focused());

    fixture.set_suggestions_open(true);
    tap(&fixture, Key::Tab);
    assert!(!fixture.get_suggestions_open());
    assert_eq!(fixture.get_traversals(), 1);

    fixture.set_suggestions_open(true);
    fixture
        .window()
        .take_snapshot()
        .expect("render open suggestions");
    click(&fixture, 60.0, 115.0);
    assert_eq!(fixture.get_selected_id(), "alpha");
    assert!(!fixture.get_suggestions_open());

    fixture.set_active_index(1);
    fixture.set_suggestions_open(true);
    fixture.set_single_suggestion(true);
    fixture
        .window()
        .take_snapshot()
        .expect("settle filtered suggestions");
    assert_eq!(fixture.get_active_index(), 0);
    assert!(fixture.get_suggestions_open());
    fixture.set_anchor_eligible(false);
    fixture
        .window()
        .take_snapshot()
        .expect("settle removed anchor");
    assert!(!fixture.get_suggestions_open());
}
