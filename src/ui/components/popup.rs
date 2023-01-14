use std::io::Stdout;
use tui::backend::CrosstermBackend;
use tui::layout::Rect;
use tui::widgets::Clear;
use tui::Frame;

pub trait Popup {
    fn open(&mut self);
    fn close(&mut self);

    fn is_open(&self) -> bool;

    fn calculate_inner_area(&self, outer_area: Rect) -> Rect;

    fn draw_inner(&mut self, frame: &mut Frame<CrosstermBackend<Stdout>>, inner_area: Rect);

    fn draw(&mut self, frame: &mut Frame<CrosstermBackend<Stdout>>) {
        let area = self.calculate_inner_area(frame.size());
        frame.render_widget(Clear, area);
        self.draw_inner(frame, area);
    }
}
