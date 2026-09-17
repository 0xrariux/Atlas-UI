//! Outside dismissal through Atlas popover and menu composition.

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

fn click(fixture: &MenuLayerRuntimeFixture, x: f32, y: f32) {
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

fn escape(fixture: &MenuLayerRuntimeFixture) {
    fixture.window().dispatch_event(WindowEvent::KeyPressed {
        text: Key::Escape.into(),
    });
    fixture.window().dispatch_event(WindowEvent::KeyReleased {
        text: Key::Escape.into(),
    });
}

#[test]
fn composed_menu_closes_on_outside_click_and_escape_with_one_focus_return() {
    slint::platform::set_platform(Box::new(FixturePlatform {
        window: MinimalSoftwareWindow::new(RepaintBufferType::ReusedBuffer),
    }))
    .expect("fixture platform");

    let fixture = MenuLayerRuntimeFixture::new().expect("menu layer fixture");
    fixture.show().expect("show fixture");
    fixture
        .window()
        .set_size(slint::LogicalSize::new(480.0, 320.0));
    fixture.window().take_snapshot().expect("render fixture");
    fixture.invoke_focus_invoker();

    fixture.set_open(true);
    fixture.window().take_snapshot().expect("render menu layer");
    click(&fixture, 400.0, 250.0);
    assert!(!fixture.get_open());
    assert_eq!(fixture.get_dismissals(), 1);
    assert_eq!(fixture.get_restorations(), 1);
    assert!(fixture.get_invoker_focused());

    fixture.set_open(true);
    fixture.window().take_snapshot().expect("render menu layer");
    escape(&fixture);
    assert!(!fixture.get_open());
    assert_eq!(fixture.get_dismissals(), 2);
    assert_eq!(fixture.get_restorations(), 2);
    assert!(fixture.get_invoker_focused());
}
