//! Runtime keyboard checks for shared actions and overlay focus contracts.

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

fn press(fixture: &InteractionRuntimeFixture, key: Key) {
    fixture
        .window()
        .dispatch_event(WindowEvent::KeyPressed { text: key.into() });
}

fn release(fixture: &InteractionRuntimeFixture, key: Key) {
    fixture
        .window()
        .dispatch_event(WindowEvent::KeyReleased { text: key.into() });
}

fn check_actions(fixture: &InteractionRuntimeFixture) {
    fixture.invoke_focus_first();
    assert!(fixture.get_first_focused());
    assert!(fixture.get_first_focus_visible());

    press(fixture, Key::Return);
    assert!(fixture.get_first_pressed());
    assert_eq!(fixture.get_first_activations(), 0);
    release(fixture, Key::Return);
    assert!(!fixture.get_first_pressed());
    assert_eq!(fixture.get_first_activations(), 1);

    press(fixture, Key::Return);
    release(fixture, Key::Space);
    assert_eq!(
        fixture.get_first_activations(),
        1,
        "a different key must not activate"
    );
    release(fixture, Key::Return);
    assert_eq!(fixture.get_first_activations(), 2);

    press(fixture, Key::Space);
    release(fixture, Key::Space);
    assert_eq!(fixture.get_first_activations(), 3);

    fixture.set_first_enabled(false);
    fixture
        .window()
        .take_snapshot()
        .expect("render disabled control");
    assert!(!fixture.get_first_focused());
    fixture.invoke_focus_first();
    assert!(!fixture.get_first_focused());
    press(fixture, Key::Return);
    release(fixture, Key::Return);
    assert_eq!(fixture.get_first_activations(), 3);

    press(fixture, Key::Tab);
    release(fixture, Key::Tab);
    assert!(fixture.get_second_focused());

    fixture.set_first_enabled(true);
    assert!(
        !fixture.get_first_focused(),
        "reenabling must not restore stale focus"
    );
    fixture.invoke_focus_first();
    press(fixture, Key::Return);
    fixture.set_first_enabled(false);
    release(fixture, Key::Return);
    assert_eq!(fixture.get_first_activations(), 3, "disabled during press");

    fixture.set_first_enabled(true);
    let position = LogicalPosition::new(40.0, 40.0);
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
    assert_eq!(fixture.get_first_activations(), 4);
    assert!(fixture.get_first_focused());
    assert!(!fixture.get_first_focus_visible());
}

fn check_modal(fixture: &InteractionRuntimeFixture) {
    fixture.invoke_focus_second();
    assert!(fixture.get_second_focused());
    fixture.set_modal_open(true);
    fixture.window().take_snapshot().expect("render modal");
    press(fixture, Key::Return);
    release(fixture, Key::Return);
    assert_eq!(fixture.get_modal_confirms(), 1);
    assert!(!fixture.get_modal_open());
    assert_eq!(fixture.get_focus_restores(), 1);
    assert!(fixture.get_first_focused());
    assert!(fixture.get_first_focus_visible());

    fixture.set_modal_open(true);
    press(fixture, Key::Tab);
    release(fixture, Key::Tab);
    press(fixture, Key::Return);
    release(fixture, Key::Return);
    assert_eq!(fixture.get_modal_cancels(), 1);
    assert_eq!(fixture.get_focus_restores(), 2);

    fixture.set_modal_open(true);
    press(fixture, Key::Escape);
    release(fixture, Key::Escape);
    assert_eq!(fixture.get_modal_cancels(), 2);
    assert_eq!(fixture.get_focus_restores(), 3);
}

fn check_menu_and_drawer(fixture: &InteractionRuntimeFixture) {
    fixture.set_menu_open(true);
    fixture.window().take_snapshot().expect("render menu");
    press(fixture, Key::DownArrow);
    release(fixture, Key::DownArrow);
    assert_eq!(fixture.get_menu_active_index(), 1);
    press(fixture, Key::Return);
    release(fixture, Key::Return);
    assert_eq!(fixture.get_menu_selected_id(), "");
    assert!(fixture.get_menu_open());
    press(fixture, Key::DownArrow);
    release(fixture, Key::DownArrow);
    assert_eq!(fixture.get_menu_active_index(), 2);
    press(fixture, Key::Return);
    release(fixture, Key::Return);
    assert_eq!(fixture.get_menu_selected_id(), "last");
    assert!(!fixture.get_menu_open());
    assert_eq!(fixture.get_focus_restores(), 4);
    assert!(fixture.get_first_focused());
    assert!(fixture.get_first_focus_visible());

    fixture.set_menu_open(true);
    press(fixture, Key::Home);
    release(fixture, Key::Home);
    assert_eq!(fixture.get_menu_active_index(), 0);
    press(fixture, Key::End);
    release(fixture, Key::End);
    assert_eq!(fixture.get_menu_active_index(), 2);
    press(fixture, Key::UpArrow);
    release(fixture, Key::UpArrow);
    assert_eq!(fixture.get_menu_active_index(), 1);
    press(fixture, Key::Space);
    release(fixture, Key::Space);
    assert_eq!(fixture.get_menu_selected_id(), "last");
    assert!(fixture.get_menu_open());
    press(fixture, Key::Escape);
    release(fixture, Key::Escape);
    assert_eq!(fixture.get_menu_dismissals(), 1);
    assert!(!fixture.get_menu_open());
    assert_eq!(fixture.get_focus_restores(), 5);
    assert!(fixture.get_first_focused());

    fixture.set_menu_open(true);
    press(fixture, Key::Tab);
    assert!(fixture.get_second_focused());
    release(fixture, Key::Tab);
    assert_eq!(fixture.get_menu_traversals(), 1);
    assert!(!fixture.get_menu_traversed_backward());
    assert_eq!(fixture.get_menu_dismissals(), 2);
    assert!(fixture.get_second_focused());
    assert_eq!(fixture.get_focus_restores(), 5);

    fixture.set_menu_open(true);
    press(fixture, Key::Backtab);
    release(fixture, Key::Backtab);
    assert_eq!(fixture.get_menu_traversals(), 2);
    assert!(fixture.get_menu_traversed_backward());
    assert_eq!(fixture.get_menu_dismissals(), 3);
    assert!(fixture.get_first_focused());
    assert_eq!(fixture.get_focus_restores(), 5);

    fixture.set_drawer_open(true);
    fixture.window().take_snapshot().expect("render drawer");
    press(fixture, Key::Escape);
    release(fixture, Key::Escape);
    assert_eq!(fixture.get_drawer_closes(), 1);
    assert!(!fixture.get_drawer_open());
    assert_eq!(fixture.get_focus_restores(), 6);
    assert!(fixture.get_first_focused());
}

#[test]
fn actions_and_overlays_follow_keyboard_focus_contract() {
    slint::platform::set_platform(Box::new(FixturePlatform {
        window: MinimalSoftwareWindow::new(RepaintBufferType::ReusedBuffer),
    }))
    .expect("fixture platform");

    let fixture = InteractionRuntimeFixture::new().expect("interaction fixture");
    fixture.show().expect("show fixture");
    fixture
        .window()
        .set_size(slint::LogicalSize::new(400.0, 300.0));
    fixture.window().take_snapshot().expect("render fixture");

    check_actions(&fixture);
    check_modal(&fixture);
    check_menu_and_drawer(&fixture);
}
