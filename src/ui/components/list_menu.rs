use crate::ui::components::popup::Popup;
use crate::ui::event_handling::{KeyHandler, MouseArea};
use anyhow::{Context, Result};
use crossterm::event::{KeyCode, KeyEvent, MouseButton, MouseEvent, MouseEventKind};
use std::collections::HashMap;
use std::io::Stdout;
use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, List, ListItem, ListState};
use tui::Frame;

pub struct ListMenuSeparator {
    pub title: String,
}

pub struct ListMenuItem {
    pub title: String,
    pub hotkey: KeyCode,
}

pub enum ListMenuEntry {
    Item(ListMenuItem),
    Separator(ListMenuSeparator),
}

pub struct ListMenuItems(pub Vec<ListMenuEntry>);

pub struct ListMenu {
    pub title: String,
    is_open: bool,
    state: ListState,
    area: Rect,
    items: ListMenuItems,
}

pub struct ListMenuReactions<'a>(pub HashMap<KeyCode, &'a dyn Fn()>);

impl<'a> KeyHandler<Box<ListMenuReactions<'a>>> for ListMenu {
    fn handle_key_event(
        &mut self,
        key_event: &KeyEvent,
        reactions: &Box<ListMenuReactions>,
    ) -> Result<bool> {
        match key_event.code {
            KeyCode::Up => {
                let new_idx = match self.state.selected() {
                    Some(current_idx) => {
                        if current_idx == 0 {
                            self.items.0.len() - 1
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
                        if current_idx >= self.items.0.len() - 1 {
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
                match self.state.selected() {
                    Some(current_idx) => {
                        match self
                            .items
                            .0
                            .get(current_idx)
                            .context("Can not find menu item at specified index")?
                        {
                            ListMenuEntry::Item(item) => {
                                reactions
                                    .0
                                    .get(&item.hotkey)
                                    .context("Can not find reaction for this keycode")?(
                                );
                            }
                            _ => {}
                        }
                    }
                    None => {}
                }
                Ok(true)
            }
            _ => Ok(false),
        }
    }
}

impl<'a> MouseArea<Box<ListMenuReactions<'a>>> for ListMenu {
    fn handle_mouse_event(
        &mut self,
        mouse_event: &MouseEvent,
        reactions: &Box<ListMenuReactions>,
    ) -> Result<bool> {
        match mouse_event.kind {
            MouseEventKind::Down(MouseButton::Left) => {
                if self.is_inside(mouse_event, self.area) {
                    (reactions
                        .0
                        .get(&KeyCode::Char('q'))
                        .context("Can not find reaction for this keycode")?)();
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

impl Popup for ListMenu {
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

    fn draw_inner(&mut self, frame: &mut Frame<CrosstermBackend<Stdout>>, inner_area: Rect) {
        self.area = inner_area;

        // Create full list of entries
        let list_items: Vec<ListItem> = self
            .items
            .0
            .iter()
            .map(|entry| match entry {
                ListMenuEntry::Item(item) => ListItem::new(item.title.clone())
                    .style(Style::default().fg(Color::Black).bg(Color::White)),
                ListMenuEntry::Separator(separator) => ListItem::new(separator.title.clone())
                    .style(
                        Style::default()
                            .fg(Color::Black)
                            .bg(Color::White)
                            .add_modifier(Modifier::UNDERLINED),
                    ),
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

impl ListMenu {
    pub(crate) fn new(title: String, items: ListMenuItems) -> Self {
        Self {
            title,
            is_open: false,
            state: Default::default(),
            area: Default::default(),
            items,
        }
    }
}
