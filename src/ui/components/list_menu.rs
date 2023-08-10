use crate::ui::components::{FocusScope, MouseArea};
use anyhow::{Context, Result};
use crossterm::event::{KeyCode, KeyEvent, MouseButton, MouseEvent, MouseEventKind};
use tui::backend::Backend;
use tui::layout::Rect;
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, List, ListItem, ListState};
use tui::Frame;

pub struct ListMenuItem {
    pub title: String,
    pub hotkey: KeyCode,
    pub action_func: Box<dyn FnMut()>,
}

const BORDER_THICKNESS: u16 = 1;

pub struct ListMenu {
    title: String,
    state: ListState,
    items: Vec<ListMenuItem>,
    area: Rect,
}

impl FocusScope for ListMenu {
    fn handle_key_event(&mut self, key_event: &KeyEvent) -> Result<bool> {
        match key_event.code {
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
            KeyCode::Enter | KeyCode::Char(' ') => {
                if let Some(current_idx) = self.state.selected() {
                    let item = self
                        .items
                        .get_mut(current_idx)
                        .context("Can not find menu item at specified index")?;
                    (item.action_func)();
                }
                Ok(true)
            }
            _ => {
                // See if this key is a hotkey for one of the items
                let item_optional = self
                    .items
                    .iter_mut()
                    .find(|item| item.hotkey == key_event.code);
                if let Some(item) = item_optional {
                    // If we found an item that matches this hotkey, activate its action function
                    (item.action_func)();
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
        }
    }
}

impl MouseArea for ListMenu {
    fn handle_mouse_event(&mut self, mouse_event: &MouseEvent) -> Result<bool> {
        let first_item_row = self.area.y + BORDER_THICKNESS;
        match mouse_event.kind {
            MouseEventKind::Down(MouseButton::Left) => {
                if mouse_event.row >= first_item_row {
                    let index = (mouse_event.row - first_item_row) as usize;
                    if let Some(item) = self.items.get_mut(index) {
                        (item.action_func)();
                    }
                }
                Ok(true)
            }
            MouseEventKind::Moved => {
                if mouse_event.row >= first_item_row {
                    let index = (mouse_event.row - first_item_row) as usize;
                    self.state
                        .select(Some(index.clamp(0, self.items.len() - 1)));
                }
                Ok(true)
            }
            _ => Ok(false),
        }
    }
}

impl ListMenu {
    pub fn new(title: String, items: Vec<ListMenuItem>) -> ListMenu {
        ListMenu {
            title,
            state: ListState::default(),
            items,
            area: Rect::default(),
        }
    }

    pub fn reset_state(&mut self) {
        self.state.select(Some(0));
    }

    pub(crate) fn draw<B: Backend>(&mut self, frame: &mut Frame<B>, area: Rect) {
        self.area = area;

        let list_items: Vec<ListItem> = self
            .items
            .iter()
            .map(|item| {
                ListItem::new(item.title.clone())
                    .style(Style::default().fg(Color::Black).bg(Color::White))
            })
            .collect();

        let list = List::new(list_items)
            .block(
                Block::default()
                    .title(self.title.clone())
                    .borders(Borders::ALL),
            )
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
            .highlight_symbol(">>");

        frame.render_stateful_widget(list, self.area, &mut self.state);
    }
}
