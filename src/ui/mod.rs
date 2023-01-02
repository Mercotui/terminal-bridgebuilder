mod focus_scope;
mod icon_button;
mod main_menu;
mod mouse_area;
mod popup;
mod scene_view;
mod terminal_manager;
mod world_menu;
mod world_view;

use crate::stop_token::StopToken;
use crate::ui::focus_scope::FocusScope;
use crate::ui::main_menu::MainMenu;
use crate::ui::mouse_area::MouseArea;
use crate::ui::popup::Popup;
use crate::ui::scene_view::SceneView;
use crate::ui::terminal_manager::TerminalManagerEvent;
use anyhow::{Context, Result};
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use std::sync::Arc;
use terminal_manager::TerminalManager;

pub struct Gui {
    stop_token: Arc<StopToken>,
    terminal_manager: TerminalManager,
    main_menu: MainMenu,
    scene_view: SceneView,
}

impl FocusScope for Gui {
    fn handle_key_event(&mut self, key_event: &KeyEvent) -> Result<bool> {
        match key_event.code {
            KeyCode::Esc => {
                // Escape can toggle the main menu
                self.main_menu.open();
                Ok(true)
            }
            KeyCode::Char('c') if key_event.modifiers == KeyModifiers::CONTROL => {
                self.stop_token.request_stop();
                Ok(true)
            }
            _ => {
                // We are at the top level, discard unused key events, don't request draw
                Ok(false)
            }
        }
    }

    fn determine_focus(&mut self) -> Result<Option<&mut dyn FocusScope>> {
        if self.main_menu.is_open() {
            Ok(Some(&mut self.main_menu))
        } else {
            Ok(Some(&mut self.scene_view))
        }
    }
}

impl MouseArea for Gui {
    fn determine_focus(&mut self) -> Result<Option<&mut dyn MouseArea>> {
        if self.main_menu.is_open() {
            Ok(Some(&mut self.main_menu))
        } else {
            Ok(None)
        }
    }
}

impl Gui {
    pub fn new(
        stop_token: Arc<StopToken>,
        initial_level_path: Option<&std::path::PathBuf>,
    ) -> Result<Gui> {
        Ok(Gui {
            stop_token: stop_token.clone(),
            terminal_manager: TerminalManager::new().context("Can't setup terminal")?,
            main_menu: MainMenu::new(stop_token),
            scene_view: SceneView::new(initial_level_path)?,
        })
    }

    pub fn run(&mut self) -> Result<()> {
        let mut draw_needed = true;
        while self.stop_token.keep_running() {
            match self.terminal_manager.next()? {
                TerminalManagerEvent::TickEvent => {
                    draw_needed |= self.scene_view.physics_tick()?;
                }
                TerminalManagerEvent::TerminalEvent(event) => {
                    draw_needed |= self.handle_terminal_event(event)?
                }
            }
            if draw_needed {
                draw_needed = false;
                self.terminal_manager.draw(|frame| {
                    self.scene_view.draw(frame);
                    if self.main_menu.is_open() {
                        self.main_menu.draw(frame);
                    }
                })?;
            }
        }
        Ok(())
    }

    fn handle_terminal_event(&mut self, event: Event) -> Result<bool> {
        match event {
            Event::Key(key_event) => {
                // If the key event was handled, return true to redraw the UI
                self.submit_key_event(&key_event)
            }
            Event::Mouse(mouse_event) => {
                // If the mouse event was handled, return true to redraw the UI
                self.submit_mouse_event(&mouse_event)
            }
            Event::Resize(_, _) => {
                // resized terminal requires a redraw
                return Ok(true);
            }
            // We don't handle other events, no need to redraw
            _ => return Ok(false),
        }
    }
}
