//! Runtime roving focus and controlled selection for Atlas radio choices.

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

fn tap(fixture: &RadioGroupRuntimeFixture, key: Key) {
    fixture
        .window()
        .dispatch_event(WindowEvent::KeyPressed { text: key.into() });
    fixture
        .window()
        .dispatch_event(WindowEvent::KeyReleased { text: key.into() });
}

#[test]
fn radio_group_routes_focus_around_disabled_choices_and_selects_on_enter() {
    slint::platform::set_platform(Box::new(FixturePlatform {
        window: MinimalSoftwareWindow::new(RepaintBufferType::ReusedBuffer),
    }))
    .expect("fixture platform");

    let fixture = RadioGroupRuntimeFixture::new().expect("radio fixture");
    fixture.show().expect("show fixture");
    fixture
        .window()
        .set_size(slint::LogicalSize::new(480.0, 320.0));
    fixture.window().take_snapshot().expect("render fixture");

    fixture.invoke_focus_first();
    fixture
        .window()
        .take_snapshot()
        .expect("settle first focus");
    assert_eq!(fixture.get_active_index(), 0);
    assert_eq!(fixture.get_focused_index(), 0);
    fixture.invoke_focus_disabled();
    assert_eq!(fixture.get_active_index(), 0);
    assert_eq!(fixture.get_focused_index(), 0);

    tap(&fixture, Key::RightArrow);
    fixture.window().take_snapshot().expect("settle next focus");
    assert_eq!(fixture.get_navigations(), 1);
    assert_eq!(fixture.get_active_index(), 2);
    assert_eq!(fixture.get_focused_index(), 2);
    assert_eq!(fixture.get_selected_id(), "one");

    tap(&fixture, Key::Return);
    assert_eq!(fixture.get_selected_id(), "three");
    assert_eq!(fixture.get_selections(), 1);

    tap(&fixture, Key::LeftArrow);
    fixture
        .window()
        .take_snapshot()
        .expect("settle previous focus");
    assert_eq!(fixture.get_navigations(), 2);
    assert_eq!(fixture.get_active_index(), 0);
    assert_eq!(fixture.get_focused_index(), 0);
    tap(&fixture, Key::Space);
    assert_eq!(fixture.get_selected_id(), "one");
    assert_eq!(fixture.get_selections(), 2);
}
