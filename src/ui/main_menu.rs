use crate::stop_token::StopToken;
use crate::ui::focus_scope::FocusScope;
use crate::ui::mouse_area::MouseArea;
use crate::ui::popup::Popup;
use anyhow::{Context, Result};
use crossterm::event::{KeyCode, KeyEvent, MouseButton, MouseEvent, MouseEventKind};
use std::sync::Arc;
use tui::backend::Backend;
use tui::layout::{Constraint, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, List, ListItem, ListState};
use tui::Frame;

struct MenuItem(pub String, pub fn(menu: &mut MainMenu));

pub struct MainMenu {
    stop_token: Arc<StopToken>,
    is_open: bool,
    state: ListState,
    items: Vec<MenuItem>,
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
            KeyCode::Up => {
                let new_idx = match self.state.selected() {
                    Some(current_idx) => {
                        if current_idx == 0 {
                            self.items.len() - 1
                        } else {
                            current_idx - 1
                        }
                    }
                    None => 0,
                };
                self.state.select(Some(new_idx));
                Ok(true)
            }
            KeyCode::Down => {
                let new_idx = match self.state.selected() {
                    Some(current_idx) => {
                        if current_idx >= self.items.len() - 1 {
                            0
                        } else {
                            current_idx + 1
                        }
                    }
                    None => 0,
                };
                self.state.select(Some(new_idx));
                Ok(true)
            }
            KeyCode::Enter => {
                if let Some(current_idx) = self.state.selected() {
                    self.items
                        .get(current_idx)
                        .context("Can not find menu item at specified index")?
                        .1(self);
                }
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
                    // If a click happens outside the menu, we close the menu
                    self.close();
                }

                // swallow all clicks
                Ok(true)
            }
            _ => Ok(false),
        }
    }
}

impl Popup for MainMenu {
    fn open(&mut self) {
        self.is_open = true;
        self.state.select(Some(0));
    }

    fn close(&mut self) {
        self.is_open = false;
    }

    fn is_open(&self) -> bool {
        self.is_open
    }

    fn calculate_inner_area(&self, outer_area: Rect) -> Rect {
        let menu_layout = Layout::default()
            .horizontal_margin(50)
            .vertical_margin(10)
            .constraints([Constraint::Percentage(10)].as_ref())
            .split(outer_area);

        menu_layout[0]
    }

    fn draw_inner<B: Backend>(&mut self, frame: &mut Frame<B>, inner_area: Rect) {
        self.area = inner_area;

        let list_items: Vec<ListItem> = self
            .items
            .iter()
            .map(|i| {
                ListItem::new(i.0.clone()).style(Style::default().fg(Color::Black).bg(Color::White))
            })
            .collect();

        let list = List::new(list_items)
            .block(Block::default().title("Main Menu").borders(Borders::ALL))
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
            .highlight_symbol(">>");

        frame.render_stateful_widget(list, self.area, &mut self.state);
    }
}

impl MainMenu {
    pub fn new(stop_token: Arc<StopToken>) -> MainMenu {
        MainMenu {
            stop_token,
            is_open: false,
            state: ListState::default(),
            items: vec![
                MenuItem("Back to game (Esc)".to_string(), |menu| menu.close()),
                MenuItem("Load Level (L)".to_string(), |_menu| {}),
                MenuItem("Exit to terminal (Q)".to_string(), |menu| {
                    menu.stop_token.request_stop()
                }),
            ],
            area: Rect::default(),
        }
    }
}
