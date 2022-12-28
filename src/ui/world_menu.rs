use std::io::Stdout;
use tui::backend::CrosstermBackend;
use tui::layout::Rect;
use tui::widgets::{Block, Borders};
use tui::Frame;

pub struct WorldMenu;

impl WorldMenu {
    pub(crate) fn draw(&self, frame: &mut Frame<CrosstermBackend<Stdout>>, area: Rect) {
        let block = Block::default().title("World Menu").borders(Borders::ALL);
        frame.render_widget(block, area);
    }
}
