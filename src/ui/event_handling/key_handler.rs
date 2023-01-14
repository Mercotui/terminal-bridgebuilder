use anyhow::Result;
use crossterm::event::KeyEvent;

pub trait KeyHandler<TReactions> {
    fn forward_key_event(
        &mut self,
        _key_event: &KeyEvent,
        _reactions: &TReactions,
    ) -> Result<bool> {
        // Default implementation doesn't forward events
        Ok(false)
    }

    fn handle_key_event(&mut self, _key_event: &KeyEvent, _reactions: &TReactions) -> Result<bool> {
        // Default implementation doesn't handle any events
        Ok(false)
    }

    fn submit_key_event(&mut self, key_event: &KeyEvent, reactions: TReactions) -> Result<bool> {
        // Try to forward the key event to a focused child
        if self.forward_key_event(key_event, &reactions)? {
            Ok(true)
        } else {
            // If the key event was not handled by a child, than try to handle it ourselves
            self.handle_key_event(key_event, &reactions)
        }
    }
}
