use anyhow::Result;
use crossterm::event::KeyEvent;

pub trait FocusScope {
    fn handle_key_event(&mut self, _key_event: &KeyEvent) -> Result<bool> {
        // Default implementation doesn't handle any events
        Ok(false)
    }

    fn submit_key_event(&mut self, key_event: &KeyEvent) -> Result<bool> {
        let mut handled = false;

        // Forward the key event to a focused child
        if let Some(focused) = self.determine_focus()? {
            handled = focused.submit_key_event(key_event)?;
        }

        // If the key event was not handled by a child, than try to handle it ourselves
        if !handled {
            handled = self.handle_key_event(key_event)?;
        }
        Ok(handled)
    }

    fn determine_focus(&mut self) -> Result<Option<&mut dyn FocusScope>> {
        // Default implementation has nothing to focus on
        Ok(None)
    }
}
