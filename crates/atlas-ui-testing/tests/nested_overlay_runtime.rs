//! Runtime focus restoration through nested Atlas overlays.

#![allow(missing_docs)] // Generated fixture bindings are documented in Slint.

slint::include_modules!();

use slint::ComponentHandle;
use slint::platform::software_renderer::{MinimalSoftwareWindow, RepaintBufferType};
use slint::platform::{Key, Platform, PlatformError, WindowAdapter, WindowEvent};
use std::rc::Rc;

struct FixturePlatform {
    window: Rc<MinimalSoftwareWindow>,
}

impl Platform for FixturePlatform {
    fn create_window_adapter(&self) -> Result<Rc<dyn WindowAdapter>, PlatformError> {
        Ok(self.window.clone())
    }
}

fn tap(fixture: &NestedOverlayRuntimeFixture, key: Key) {
    fixture
        .window()
        .dispatch_event(WindowEvent::KeyPressed { text: key.into() });
    fixture
        .window()
        .dispatch_event(WindowEvent::KeyReleased { text: key.into() });
}

#[test]
fn escape_unwinds_menu_then_modal_and_restores_each_invoker() {
    slint::platform::set_platform(Box::new(FixturePlatform {
        window: MinimalSoftwareWindow::new(RepaintBufferType::ReusedBuffer),
    }))
    .expect("fixture platform");

    let fixture = NestedOverlayRuntimeFixture::new().expect("nested overlay fixture");
    fixture.show().expect("show fixture");
    fixture
        .window()
        .set_size(slint::LogicalSize::new(480.0, 320.0));
    fixture.window().take_snapshot().expect("render fixture");

    fixture.invoke_focus_outside();
    assert!(fixture.get_outside_focused());
    fixture.set_modal_open(true);
    fixture.window().take_snapshot().expect("render modal");
    fixture.invoke_focus_inside();
    assert!(fixture.get_inside_focused());
    assert!(!fixture.get_outside_focused());

    fixture.set_menu_open(true);
    fixture
        .window()
        .take_snapshot()
        .expect("render nested menu");
    assert!(!fixture.get_inside_focused());
    tap(&fixture, Key::Escape);
    assert_eq!(fixture.get_menu_dismissals(), 1);
    assert!(!fixture.get_menu_open());
    assert!(fixture.get_modal_open());
    assert!(fixture.get_inside_focused());

    tap(&fixture, Key::Return);
    assert_eq!(fixture.get_inside_activations(), 1);
    tap(&fixture, Key::Escape);
    assert_eq!(fixture.get_modal_dismissals(), 1);
    assert!(!fixture.get_modal_open());
    assert!(fixture.get_outside_focused());

    fixture.set_modal_open(true);
    fixture.set_menu_open(true);
    fixture
        .window()
        .take_snapshot()
        .expect("render atomic stack");
    tap(&fixture, Key::Escape);
    assert_eq!(fixture.get_menu_dismissals(), 2);
    assert!(!fixture.get_menu_open());
    assert!(fixture.get_modal_open());
    assert!(fixture.get_inside_focused());
    tap(&fixture, Key::Escape);
    assert_eq!(fixture.get_modal_dismissals(), 2);
    assert!(!fixture.get_modal_open());
    assert!(fixture.get_outside_focused());

    fixture.set_menu_open(true);
    fixture.set_modal_open(true);
    fixture
        .window()
        .take_snapshot()
        .expect("render reverse atomic stack");
    tap(&fixture, Key::Escape);
    assert_eq!(fixture.get_menu_dismissals(), 3);
    assert!(!fixture.get_menu_open());
    assert!(fixture.get_modal_open());
    assert!(fixture.get_inside_focused());
    tap(&fixture, Key::Escape);
    assert_eq!(fixture.get_modal_dismissals(), 3);
    assert!(!fixture.get_modal_open());
    assert!(fixture.get_outside_focused());

    fixture.set_modal_open(true);
    fixture
        .window()
        .take_snapshot()
        .expect("render anchor host");
    fixture.invoke_focus_inside();
    fixture.set_menu_open(true);
    fixture
        .window()
        .take_snapshot()
        .expect("render anchored menu");
    fixture.set_anchor_eligible(false);
    fixture
        .window()
        .take_snapshot()
        .expect("settle removed anchor");
    assert_eq!(fixture.get_menu_dismissals(), 4);
    assert!(!fixture.get_menu_open());
    assert!(fixture.get_inside_focused());
    fixture.set_anchor_eligible(true);
    tap(&fixture, Key::Escape);
    assert_eq!(fixture.get_modal_dismissals(), 4);
    assert!(fixture.get_outside_focused());
}
