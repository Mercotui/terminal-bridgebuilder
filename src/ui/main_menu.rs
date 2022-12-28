use crate::stop_token::StopToken;
use crate::ui::focus_scope::FocusScope;
use crate::ui::mouse_area::MouseArea;
use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent, MouseButton, MouseEvent, MouseEventKind};
use std::io::Stdout;
use std::sync::Arc;
use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Layout, Rect};
use tui::widgets::{Block, Borders};
use tui::Frame;

pub struct MainMenu {
    stop_token: Arc<StopToken>,
    is_open: bool,
    area: Rect,
}

impl FocusScope for MainMenu {
    fn handle_key_event(&mut self, key_event: &KeyEvent) -> Result<bool> {
        match key_event.code {
            KeyCode::Esc => {
                self.close();
                Ok(true)
            }
            KeyCode::Char('q') => {
                self.stop_token.request_stop();
                Ok(true)
            }
            _ => Ok(false),
        }
    }
}

impl MouseArea for MainMenu {
    fn handle_mouse_event(&mut self, mouse_event: &MouseEvent) -> Result<bool> {
        match mouse_event.kind {
            MouseEventKind::Down(MouseButton::Left) => {
                if self.is_inside(mouse_event, self.area) {
                    // TODO (Menno 25.12.2022) Remove this once proper menu items have been added
                    self.stop_token.request_stop();
                } else {
                    // if a click happens outside the menu, we close the menu
                    self.close();
                }

                // swallow all clicks
                Ok(true)
            }
            _ => Ok(false),
        }
    }
}

impl MainMenu {
    pub fn new(stop_token: Arc<StopToken>) -> MainMenu {
        MainMenu {
            stop_token,
            is_open: false,
            area: Rect::default(),
        }
    }

    // TODO(Menno 23.12.2022) Put this logic in a trait
    pub fn open(&mut self) {
        self.is_open = true;
    }

    pub fn is_open(&self) -> bool {
        self.is_open
    }

    fn close(&mut self) {
        self.is_open = false;
    }

    pub fn draw(&mut self, frame: &mut Frame<CrosstermBackend<Stdout>>) {
        let menu_layout = Layout::default()
            .horizontal_margin(50)
            .vertical_margin(10)
            .constraints([Constraint::Percentage(100)].as_ref())
            .split(frame.size());

        let block = Block::default().title("Main Menu").borders(Borders::ALL);
        self.area = menu_layout[0];

        frame.render_widget(block, self.area);
    }
}
