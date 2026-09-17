//! Runtime checks for Atlas P0 viewport and long-menu behavior.

#![allow(missing_docs)] // Generated fixture bindings are documented in Slint.

slint::include_modules!();

use slint::platform::software_renderer::{MinimalSoftwareWindow, RepaintBufferType};
use slint::platform::{Key, Platform, PlatformError, WindowAdapter, WindowEvent};
use slint::{ComponentHandle, ModelRc, VecModel};
use std::rc::Rc;

struct FixturePlatform {
    window: Rc<MinimalSoftwareWindow>,
}

impl Platform for FixturePlatform {
    fn create_window_adapter(&self) -> Result<Rc<dyn WindowAdapter>, PlatformError> {
        Ok(self.window.clone())
    }
}

fn render<T: ComponentHandle>(fixture: &T) {
    fixture
        .window()
        .set_size(slint::LogicalSize::new(360.0, 360.0));
    slint::platform::update_timers_and_animations();
    fixture.window().take_snapshot().expect("render fixture");
}

fn near(actual: f32, expected: f32) {
    assert!(
        (actual - expected).abs() <= 0.01,
        "expected {expected}, got {actual}"
    );
}

fn tap<T: ComponentHandle>(fixture: &T, key: Key) {
    fixture
        .window()
        .dispatch_event(WindowEvent::KeyPressed { text: key.into() });
    fixture
        .window()
        .dispatch_event(WindowEvent::KeyReleased { text: key.into() });
}

#[test]
fn controlled_viewport_clamps_both_axes_and_reveals_rectangles() {
    slint::platform::set_platform(Box::new(FixturePlatform {
        window: MinimalSoftwareWindow::new(RepaintBufferType::ReusedBuffer),
    }))
    .expect("fixture platform");
    let fixture = ScrollViewportRuntimeFixture::new().expect("viewport fixture");
    fixture.show().expect("show fixture");
    render(&fixture);

    assert!(fixture.get_maximum_x() > 200.0);
    near(fixture.get_maximum_y(), 180.0);
    fixture.invoke_scroll(999.0, 999.0);
    render(&fixture);
    near(fixture.get_offset_x(), fixture.get_maximum_x());
    near(fixture.get_offset_y(), fixture.get_maximum_y());

    fixture.invoke_scroll(0.0, 0.0);
    fixture.invoke_reveal(250.0, 200.0, 20.0, 20.0);
    render(&fixture);
    assert!(fixture.get_offset_x() > 0.0);
    assert!(fixture.get_offset_y() > 0.0);

    fixture.invoke_scroll(0.0, 0.0);
    fixture.set_right_to_left(true);
    fixture.invoke_focus_viewport();
    tap(&fixture, Key::LeftArrow);
    render(&fixture);
    assert!(fixture.get_offset_x() > 0.0);

    fixture.set_host_content_width(100.0);
    fixture.set_host_content_height(100.0);
    render(&fixture);
    near(fixture.get_maximum_x(), 0.0);
    near(fixture.get_maximum_y(), 0.0);
    near(fixture.get_offset_x(), 0.0);
    near(fixture.get_offset_y(), 0.0);
}

#[test]
fn long_menu_reveals_keyboard_active_item_inside_height_limit() {
    slint::platform::set_platform(Box::new(FixturePlatform {
        window: MinimalSoftwareWindow::new(RepaintBufferType::ReusedBuffer),
    }))
    .expect("fixture platform");
    let fixture = LongMenuRuntimeFixture::new().expect("menu fixture");
    let items: Vec<MenuItem> = (0..20)
        .map(|index| MenuItem {
            id: index.to_string().into(),
            label: format!("Action {index}").into(),
            shortcut: "".into(),
            enabled: true,
            danger: false,
        })
        .collect();
    fixture.set_items(ModelRc::from(Rc::new(VecModel::from(items))));
    fixture.show().expect("show fixture");
    render(&fixture);
    assert!(fixture.get_menu_height() <= 160.0);
    fixture.invoke_navigate(2);
    render(&fixture);
    assert_eq!(fixture.get_active_index(), 19);
    assert!(fixture.get_menu_offset() > 0.0);

    let one_item = vec![MenuItem {
        id: "first".into(),
        label: "First".into(),
        shortcut: "".into(),
        enabled: true,
        danger: false,
    }];
    fixture.set_items(ModelRc::from(Rc::new(VecModel::from(one_item))));
    render(&fixture);
    assert_eq!(fixture.get_active_index(), 0);
    near(fixture.get_menu_offset(), 0.0);
}

#[test]
fn tree_view_emits_stable_ids_for_keyboard_selection_and_disclosure() {
    slint::platform::set_platform(Box::new(FixturePlatform {
        window: MinimalSoftwareWindow::new(RepaintBufferType::ReusedBuffer),
    }))
    .expect("fixture platform");
    let fixture = TreeViewRuntimeFixture::new().expect("tree fixture");
    let rows = vec![
        TreeViewRow {
            id: "root".into(),
            label: "Root".into(),
            depth: 0,
            has_children: true,
            expanded: false,
            selected: false,
            disabled: false,
        },
        TreeViewRow {
            id: "sibling".into(),
            label: "Sibling".into(),
            depth: 0,
            has_children: false,
            expanded: false,
            selected: false,
            disabled: false,
        },
    ];
    fixture.set_rows(ModelRc::from(Rc::new(VecModel::from(rows))));
    fixture.show().expect("show fixture");
    render(&fixture);
    fixture.invoke_focus_first();
    tap(&fixture, Key::RightArrow);
    render(&fixture);
    assert_eq!(fixture.get_expanded_id(), "root");
    assert!(fixture.get_expansion_state());
    tap(&fixture, Key::DownArrow);
    tap(&fixture, Key::Return);
    render(&fixture);
    assert_eq!(fixture.get_focused_index(), 1);
    assert_eq!(fixture.get_selected_id(), "sibling");
}
