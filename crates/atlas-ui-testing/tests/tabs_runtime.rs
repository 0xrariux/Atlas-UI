//! Runtime focus and close settlement for controlled workspace tabs.

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

fn tap(fixture: &TabsRuntimeFixture, key: Key) {
    fixture
        .window()
        .dispatch_event(WindowEvent::KeyPressed { text: key.into() });
    fixture
        .window()
        .dispatch_event(WindowEvent::KeyReleased { text: key.into() });
}

#[test]
fn workspace_tabs_move_activate_and_recover_focus_after_mutation() {
    slint::platform::set_platform(Box::new(FixturePlatform {
        window: MinimalSoftwareWindow::new(RepaintBufferType::ReusedBuffer),
    }))
    .expect("fixture platform");

    let fixture = TabsRuntimeFixture::new().expect("tabs fixture");
    fixture.show().expect("show fixture");
    fixture
        .window()
        .set_size(slint::LogicalSize::new(520.0, 160.0));
    fixture.window().take_snapshot().expect("render tabs");

    fixture.invoke_focus_index(-10);
    assert_eq!(fixture.get_active_index(), 0);
    fixture.invoke_focus_index(99);
    assert_eq!(fixture.get_active_index(), 2);
    fixture.invoke_focus_index(0);
    tap(&fixture, Key::RightArrow);
    assert_eq!(fixture.get_active_index(), 1);
    assert_eq!(fixture.get_focused_index(), 1);
    tap(&fixture, Key::Return);
    assert_eq!(fixture.get_activated_index(), 1);

    tap(&fixture, Key::End);
    assert_eq!(fixture.get_active_index(), 2);
    tap(&fixture, Key::RightArrow);
    assert_eq!(fixture.get_active_index(), 0, "forward navigation wraps");
    tap(&fixture, Key::LeftArrow);
    assert_eq!(fixture.get_active_index(), 2, "reverse navigation wraps");
    tap(&fixture, Key::Home);
    assert_eq!(fixture.get_active_index(), 0);
    tap(&fixture, Key::End);
    assert_eq!(fixture.get_active_index(), 2);

    tap(&fixture, Key::Delete);
    assert_eq!(fixture.get_closed_index(), 2);
    assert_eq!(fixture.get_tab_count(), 2);
    assert_eq!(fixture.get_active_index(), 1);
    assert_eq!(fixture.get_focused_index(), 1);
    tap(&fixture, Key::Space);
    assert_eq!(fixture.get_activated_index(), 1);

    fixture.set_second_enabled(false);
    fixture.invoke_focus_index(0);
    assert_eq!(fixture.get_active_index(), 0);
    assert_eq!(fixture.get_focused_index(), 0);
    tap(&fixture, Key::Return);
    assert_eq!(fixture.get_activated_index(), 0);
    assert!(fixture.get_focus_requests() >= 7);
}
