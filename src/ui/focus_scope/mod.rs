mod unittest;

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
        match self.determine_focus()? {
            Some(focused) => {
                handled = focused.submit_key_event(key_event)?;
            }
            None => {}
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
