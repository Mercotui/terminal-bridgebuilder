use anyhow::Result;
use std::io;
use tui::{backend::CrosstermBackend, Terminal};

pub struct GUI {}

impl GUI {
    pub fn new() -> Result<()> {
        let stdout = io::stdout();
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        Ok(())
    }
}
