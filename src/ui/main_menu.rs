use crate::stop_token::StopToken;
use crate::ui::components::{FocusScope, ListMenu, ListMenuItem, MouseArea, Popup};
use anyhow::Result;
use crossterm::event::{KeyCode, MouseButton, MouseEvent, MouseEventKind};
use std::rc::Rc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tui::backend::Backend;
use tui::layout::{Constraint, Layout, Rect};
use tui::Frame;

pub struct MainMenu {
    is_open: Rc<AtomicBool>,
    list_menu: ListMenu,
    area: Rect,
}

impl FocusScope for MainMenu {
    fn determine_focus(&mut self) -> anyhow::Result<Option<&mut dyn FocusScope>> {
        Ok(Some(&mut self.list_menu))
    }
}

impl MouseArea for MainMenu {
    fn handle_mouse_event(&mut self, mouse_event: &MouseEvent) -> Result<bool> {
        if MouseEventKind::Down(MouseButton::Left) == mouse_event.kind {
            // Note: it's tempting to skip this check, as we already checked if keys should be forwarded to the
            // ListMenu, however it could be that this event was indeed forwarded to the list menu,
            // and it did not handle it. In that case, we shall ignore the event.
            if !self.is_inside(mouse_event, self.area) {
                // If a click happens outside the menu, we close the menu
                self.close();
                return Ok(true);
            }
        }

        Ok(false)
    }

    fn determine_focus(&mut self, mouse_event: &MouseEvent) -> Result<Option<&mut dyn MouseArea>> {
        if self.is_inside(mouse_event, self.area) {
            Ok(Some(&mut self.list_menu))
        } else {
            Ok(None)
        }
    }
}

impl Popup for MainMenu {
    fn open(&mut self) {
        self.is_open.store(true, Ordering::Relaxed);
        self.list_menu.reset_state()
    }

    fn close(&mut self) {
        self.is_open.store(false, Ordering::Relaxed);
    }

    fn is_open(&self) -> bool {
        self.is_open.load(Ordering::Relaxed)
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
        self.list_menu.draw(frame, inner_area);
    }
}

impl MainMenu {
    pub fn new(stop_token: Arc<StopToken>) -> MainMenu {
        let is_open = Rc::new(AtomicBool::new(false));
        MainMenu {
            is_open: is_open.clone(),
            list_menu: ListMenu::new(
                "Main Menu".to_string(),
                vec![
                    ListMenuItem {
                        title: "Back to game (Esc)".to_string(),
                        hotkey: KeyCode::Esc,
                        action_func: Box::from(move || is_open.store(false, Ordering::Relaxed)),
                    },
                    ListMenuItem {
                        title: "Exit to terminal (Q)".to_string(),
                        hotkey: KeyCode::Char('q'),
                        action_func: Box::from(move || stop_token.request_stop()),
                    },
                ],
            ),
            area: Default::default(),
        }
    }
}
