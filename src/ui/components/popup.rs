use tui::backend::Backend;
use tui::layout::Rect;
use tui::widgets::Clear;
use tui::Frame;

pub trait Popup {
    fn open(&mut self);
    fn close(&mut self);

    fn is_open(&self) -> bool;

    fn calculate_inner_area(&self, outer_area: Rect) -> Rect;

    fn draw_inner<B: Backend>(&mut self, frame: &mut Frame<B>, inner_area: Rect);

    fn draw<B: Backend>(&mut self, frame: &mut Frame<B>) {
        let area = self.calculate_inner_area(frame.size());
        frame.render_widget(Clear, area);
        self.draw_inner(frame, area);
    }
}
