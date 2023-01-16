use crate::ui::components::MouseArea;
use crossterm::event::{KeyModifiers, MouseEvent, MouseEventKind};
use tui::layout::Rect;

struct Testable {}
impl MouseArea for Testable {}

#[test]
fn mouse_area_is_inside() {
    let testable = Testable {};
    let rect = Rect::new(2, 10, 5, 5);
    let mouse_event = MouseEvent {
        kind: MouseEventKind::Moved,
        column: 4,
        row: 13,
        modifiers: KeyModifiers::empty(),
    };

    assert!(testable.is_inside(&mouse_event, rect));
}

// TODO(Menno 01.01.2023) Implement more unittests by mocking
