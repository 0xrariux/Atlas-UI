//! Slint 1.18 visual layout order and keyboard traversal are measured separately.

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

fn tab(fixture: &LayoutOrderRuntimeFixture) {
    fixture.window().dispatch_event(WindowEvent::KeyPressed {
        text: Key::Tab.into(),
    });
    fixture.window().dispatch_event(WindowEvent::KeyReleased {
        text: Key::Tab.into(),
    });
}

#[test]
fn layout_order_repositions_items_without_changing_declared_tab_sequence() {
    slint::platform::set_platform(Box::new(FixturePlatform {
        window: MinimalSoftwareWindow::new(RepaintBufferType::ReusedBuffer),
    }))
    .expect("fixture platform");

    let fixture = LayoutOrderRuntimeFixture::new().expect("layout order fixture");
    fixture.show().expect("show fixture");
    fixture
        .window()
        .set_size(slint::LogicalSize::new(360.0, 100.0));
    fixture.window().take_snapshot().expect("render fixture");
    assert!(fixture.get_second_x() < fixture.get_third_x());
    assert!(fixture.get_third_x() < fixture.get_first_x());

    fixture.invoke_focus_first();
    assert_eq!(fixture.get_last_focused(), 1);
    tab(&fixture);
    assert_eq!(fixture.get_last_focused(), 2);
    tab(&fixture);
    assert_eq!(fixture.get_last_focused(), 3);
}
