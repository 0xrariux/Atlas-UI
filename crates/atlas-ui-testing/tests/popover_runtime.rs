//! Runtime dismissal and focus restoration for the slotted popover.

#![allow(missing_docs)] // Generated fixture bindings are documented in Slint.

slint::include_modules!();

use slint::ComponentHandle;
use slint::LogicalPosition;
use slint::platform::software_renderer::{MinimalSoftwareWindow, RepaintBufferType};
use slint::platform::{
    Key, Platform, PlatformError, PointerEventButton, WindowAdapter, WindowEvent,
};
use std::rc::Rc;

struct FixturePlatform {
    window: Rc<MinimalSoftwareWindow>,
}

impl Platform for FixturePlatform {
    fn create_window_adapter(&self) -> Result<Rc<dyn WindowAdapter>, PlatformError> {
        Ok(self.window.clone())
    }
}

fn escape(fixture: &PopoverRuntimeFixture) {
    fixture.window().dispatch_event(WindowEvent::KeyPressed {
        text: Key::Escape.into(),
    });
    fixture.window().dispatch_event(WindowEvent::KeyReleased {
        text: Key::Escape.into(),
    });
}

fn click(fixture: &PopoverRuntimeFixture, x: f32, y: f32) {
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
fn popover_dismisses_on_escape_outside_click_and_anchor_loss() {
    slint::platform::set_platform(Box::new(FixturePlatform {
        window: MinimalSoftwareWindow::new(RepaintBufferType::ReusedBuffer),
    }))
    .expect("fixture platform");

    let fixture = PopoverRuntimeFixture::new().expect("popover fixture");
    fixture.show().expect("show fixture");
    fixture
        .window()
        .set_size(slint::LogicalSize::new(480.0, 320.0));
    fixture.window().take_snapshot().expect("render fixture");
    fixture.invoke_focus_invoker();

    fixture.set_popover_open(true);
    fixture.window().take_snapshot().expect("render popover");
    fixture.window().dispatch_event(WindowEvent::KeyPressed {
        text: Key::Tab.into(),
    });
    assert_eq!(fixture.get_traversals(), 1);
    escape(&fixture);
    assert!(!fixture.get_popover_open());
    assert_eq!(fixture.get_dismissals(), 1);
    assert_eq!(fixture.get_restorations(), 1);
    assert!(fixture.get_invoker_focused());

    fixture.set_popover_open(true);
    fixture.window().take_snapshot().expect("render popover");
    click(&fixture, 180.0, 100.0);
    assert!(fixture.get_popover_open());
    fixture.set_outside_dismiss_enabled(false);
    click(&fixture, 400.0, 250.0);
    assert!(fixture.get_popover_open());
    fixture.set_outside_dismiss_enabled(true);
    click(&fixture, 400.0, 250.0);
    assert!(!fixture.get_popover_open());
    assert_eq!(fixture.get_dismissals(), 2);
    assert_eq!(fixture.get_restorations(), 2);
    assert!(fixture.get_invoker_focused());

    fixture.set_popover_open(true);
    fixture.window().take_snapshot().expect("render popover");
    fixture.set_anchor_eligible(false);
    fixture
        .window()
        .take_snapshot()
        .expect("settle anchor loss");
    assert!(!fixture.get_popover_open());
    assert_eq!(fixture.get_dismissals(), 3);
    assert_eq!(fixture.get_restorations(), 3);
    assert!(fixture.get_invoker_focused());

    fixture.set_popover_open(true);
    fixture
        .window()
        .take_snapshot()
        .expect("reject ineligible open");
    assert!(!fixture.get_popover_open());
    assert_eq!(fixture.get_dismissals(), 4);
    assert_eq!(fixture.get_restorations(), 4);
}
