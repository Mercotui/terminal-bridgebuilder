mod scene_view;
mod terminal_manager;

use crate::ui::scene_view::SceneView;
use anyhow::{Context, Result};
use terminal_manager::TerminalManager;

pub struct Gui {
    terminal_manager: TerminalManager,
    scene_view: SceneView,
    keep_running: bool,
}

impl Gui {
    pub fn new() -> Result<Gui> {
        Ok(Gui {
            terminal_manager: TerminalManager::new().context("Can't setup terminal")?,
            scene_view: SceneView::new(),
            keep_running: true,
        })
    }

    pub fn run(&mut self) -> Result<()> {
        while self.keep_running {
            self.keep_running = self.terminal_manager.next()?;
            self.scene_view.update()?;
            self.terminal_manager
                .terminal
                .draw(|frame| self.scene_view.draw(frame))?;
        }
        Ok(())
    }
}
