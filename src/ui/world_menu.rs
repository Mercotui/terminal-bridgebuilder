use tui::backend::Backend;
use tui::layout::Rect;
use tui::widgets::{Block, Borders};
use tui::Frame;

pub struct WorldMenu;

impl WorldMenu {
    pub(crate) fn draw<B: Backend>(&self, frame: &mut Frame<B>, area: Rect) {
        let block = Block::default().title("World Menu").borders(Borders::ALL);
        frame.render_widget(block, area);
    }
}
