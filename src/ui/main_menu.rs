use crate::stop_token::StopToken;
use crate::ui::focus_scope::FocusScope;
use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent};
use std::io::Stdout;
use std::rc::Rc;
use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::widgets::{Block, Borders};
use tui::Frame;

pub struct MainMenu {
    stop_token: Rc<StopToken>,
    is_open: bool,
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

impl MainMenu {
    pub fn new(stop_token: Rc<StopToken>) -> MainMenu {
        MainMenu {
            stop_token,
            is_open: false,
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

    pub fn draw(&self, frame: &mut Frame<CrosstermBackend<Stdout>>) {
        let menu_layout = Layout::default()
            .horizontal_margin(50)
            .vertical_margin(10)
            .constraints([Constraint::Percentage(100)].as_ref())
            .split(frame.size());

        let block = Block::default().title("Main Menu").borders(Borders::ALL);
        frame.render_widget(block, menu_layout[0]);
    }
}
