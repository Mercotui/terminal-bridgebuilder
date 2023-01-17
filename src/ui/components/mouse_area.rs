use anyhow::Result;
use crossterm::event::MouseEvent;
use tui::layout::Rect;

pub trait MouseArea {
    fn handle_mouse_event(&mut self, _mouse_event: &MouseEvent) -> Result<bool> {
        // Default implementation doesn't handle any events
        Ok(false)
    }

    fn submit_mouse_event(&mut self, mouse_event: &MouseEvent) -> Result<bool> {
        let mut handled = false;

        // Forward the mouse event to a focused child
        if let Some(focused) = self.determine_focus(mouse_event)? {
            handled = focused.submit_mouse_event(mouse_event)?;
        }

        // If the mouse event was not handled by a child, than try to handle it ourselves
        if !handled {
            handled = self.handle_mouse_event(mouse_event)?;
        }
        Ok(handled)
    }

    fn determine_focus(&mut self, _mouse_event: &MouseEvent) -> Result<Option<&mut dyn MouseArea>> {
        Ok(None)
    }

    fn is_inside(&self, mouse_event: &MouseEvent, rect: Rect) -> bool {
        rect.left() <= mouse_event.column
            && mouse_event.column <= rect.right()
            && rect.bottom() >= mouse_event.row
            && mouse_event.row >= rect.top()
    }
}
