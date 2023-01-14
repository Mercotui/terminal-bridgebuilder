use anyhow::Result;
use crossterm::event::MouseEvent;
use tui::layout::Rect;

pub trait MouseArea<TReactions> {
    fn forward_mouse_event(
        &mut self,
        _mouse_event: &MouseEvent,
        _reactions: &TReactions,
    ) -> Result<bool> {
        // Default implementation doesn't forward events
        Ok(false)
    }

    fn handle_mouse_event(
        &mut self,
        _mouse_event: &MouseEvent,
        _reactions: &TReactions,
    ) -> Result<bool> {
        // Default implementation doesn't handle any events
        Ok(false)
    }

    fn submit_mouse_event(
        &mut self,
        mouse_event: &MouseEvent,
        reactions: TReactions,
    ) -> Result<bool> {
        // Try to forward the mouse event to a child
        if self.forward_mouse_event(mouse_event, &reactions)? {
            Ok(true)
        } else {
            // If the mouse event was not handled by a child, than try to handle it ourselves
            self.handle_mouse_event(mouse_event, &reactions)
        }
    }

    fn is_inside(&self, mouse_event: &MouseEvent, rect: Rect) -> bool {
        rect.left() <= mouse_event.column
            && mouse_event.column <= rect.right()
            && rect.bottom() >= mouse_event.row
            && mouse_event.row >= rect.top()
    }
}
