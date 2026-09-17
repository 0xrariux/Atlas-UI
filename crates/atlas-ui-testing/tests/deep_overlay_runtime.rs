//! Runtime focus order through a modal, popover, and menu.

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

fn escape(fixture: &DeepOverlayRuntimeFixture) {
    fixture.window().dispatch_event(WindowEvent::KeyPressed {
        text: Key::Escape.into(),
    });
    fixture.window().dispatch_event(WindowEvent::KeyReleased {
        text: Key::Escape.into(),
    });
}

#[test]
fn atomic_three_layer_stack_unwinds_to_each_invoker() {
    slint::platform::set_platform(Box::new(FixturePlatform {
        window: MinimalSoftwareWindow::new(RepaintBufferType::ReusedBuffer),
    }))
    .expect("fixture platform");

    let fixture = DeepOverlayRuntimeFixture::new().expect("deep overlay fixture");
    fixture.show().expect("show fixture");
    fixture
        .window()
        .set_size(slint::LogicalSize::new(480.0, 320.0));
    fixture.window().take_snapshot().expect("render fixture");
    fixture.invoke_focus_outside();

    fixture.set_modal_open(true);
    fixture.set_popover_open(true);
    fixture.set_menu_open(true);
    fixture.window().take_snapshot().expect("render stack");
    assert!(!fixture.get_outside_focused());
    assert!(!fixture.get_modal_focused());
    assert!(!fixture.get_popover_focused());

    escape(&fixture);
    assert_eq!(fixture.get_menu_dismissals(), 1);
    assert!(!fixture.get_menu_open());
    assert!(fixture.get_popover_open());
    assert!(fixture.get_modal_open());
    assert!(fixture.get_popover_focused());

    escape(&fixture);
    assert_eq!(fixture.get_popover_dismissals(), 1);
    assert!(!fixture.get_popover_open());
    assert!(fixture.get_modal_open());
    assert!(fixture.get_modal_focused());

    escape(&fixture);
    assert_eq!(fixture.get_modal_dismissals(), 1);
    assert!(!fixture.get_modal_open());
    assert!(fixture.get_outside_focused());
}
