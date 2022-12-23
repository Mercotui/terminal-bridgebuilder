use anyhow::Result;
use std::io::Stdout;
use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::widgets::{Block, Borders};
use tui::Frame;

pub struct SceneView {}

impl SceneView {
    pub fn new() -> SceneView {
        SceneView {}
    }

    pub fn loadLevel() {}

    pub fn physicsTick(&self) -> Result<()> {
        Ok(())
    }

    pub fn draw(&self, frame: &mut Frame<CrosstermBackend<Stdout>>) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Percentage(90), Constraint::Percentage(10)].as_ref())
            .split(frame.size());

        let block = Block::default().title("World View").borders(Borders::ALL);
        frame.render_widget(block, chunks[0]);
    }
}
