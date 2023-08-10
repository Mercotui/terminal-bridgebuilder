use crate::ui::components::FocusScope;
use crossterm::event::{KeyCode, KeyEvent};
use tui::backend::Backend;
use tui::layout::Rect;
use tui::style::Color;
use tui::text::Span;
use tui::widgets::canvas;
use tui::widgets::canvas::{Canvas, Context};
use tui::Frame;

/// Trait for drawing icons on a canvas
trait Icon {
    fn draw(context: &mut Context);
}

/// A start/stop icon
struct PlayIcon;
impl Icon for PlayIcon {
    fn draw(context: &mut Context) {
        context.draw(&canvas::Rectangle {
            x: -0.5,
            y: -0.5,
            width: 1.0,
            height: 1.0,
            color: Color::Red,
        });
    }
}

/// A button with an icon, title, and corresponding action
pub struct IconButton {
    title: String,
    icon_draw_func: fn(draw_context: &mut Context),
    action_func: Box<dyn FnMut()>,
}

impl FocusScope for IconButton {
    fn handle_key_event(&mut self, key_event: &KeyEvent) -> anyhow::Result<bool> {
        match key_event.code {
            KeyCode::Enter | KeyCode::Char(' ') => {
                // Enter activates the buttons action
                (self.action_func)();
                Ok(true)
            }
            _ => Ok(false),
        }
    }
}

impl IconButton {
    pub fn new(
        title: String,
        icon_draw_func: fn(draw_context: &mut Context),
        action_func: Box<dyn FnMut()>,
    ) -> Self {
        IconButton {
            title,
            icon_draw_func,
            action_func,
        }
    }

    pub(crate) fn draw<B: Backend>(&self, frame: &mut Frame<B>, area: Rect, is_focused: bool) {
        let canvas = Canvas::default()
            .x_bounds([-1.0, 1.0])
            .y_bounds([-1.0, 1.0])
            .paint(|context| {
                // Draw the icon
                (self.icon_draw_func)(context);

                // Draw a border if this button has focus
                if is_focused {
                    context.draw(&canvas::Rectangle {
                        x: -1.0,
                        y: -1.0,
                        width: 2.0,
                        height: 2.0,
                        color: Color::White,
                    });
                }

                context.print(-0.9, 1.0, Span::from(self.title.clone()));
            });
        frame.render_widget(canvas, area);
    }
}
