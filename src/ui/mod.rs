mod components;
mod event_handling;
mod main_menu;
mod scene_view;
mod terminal_manager;
mod world_menu;
mod world_view;

use crate::stop_token::StopToken;
use crate::ui::event_handling::{KeyHandler, MouseArea, NoReactions};
use crate::ui::main_menu::{MainMenu, MainMenuReactions};
use crate::ui::scene_view::SceneView;
use crate::ui::terminal_manager::TerminalManagerEvent;
use anyhow::{Context, Result};
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers, MouseEvent};
use std::sync::Arc;
use terminal_manager::TerminalManager;

pub struct Gui {
    stop_token: Arc<StopToken>,
    terminal_manager: TerminalManager,
    main_menu: MainMenu,
    scene_view: SceneView,
}

impl KeyHandler<NoReactions> for Gui {
    fn forward_key_event(&mut self, key_event: &KeyEvent, _: &NoReactions) -> Result<bool> {
        if self.main_menu.is_open() {
            self.main_menu
                .submit_key_event(key_event, self.get_main_menu_reactions())
        } else {
            Ok(false)
        }
    }

    fn handle_key_event(&mut self, key_event: &KeyEvent, _: &NoReactions) -> Result<bool> {
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
}

impl MouseArea<NoReactions> for Gui {
    fn forward_mouse_event(&mut self, mouse_event: &MouseEvent, _: &NoReactions) -> Result<bool> {
        if self.main_menu.is_open() {
            self.main_menu
                .submit_mouse_event(mouse_event, self.get_main_menu_reactions())
        } else {
            Ok(false)
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
            main_menu: MainMenu::new(),
            scene_view: SceneView::new(initial_level_path)?,
        })
    }

    pub fn run(&mut self) -> Result<()> {
        while self.stop_token.keep_running() {
            let draw_needed;
            match self.terminal_manager.next()? {
                TerminalManagerEvent::TickEvent => {
                    draw_needed = self.scene_view.physics_tick()?;
                }
                TerminalManagerEvent::TerminalEvent(event) => {
                    draw_needed = self.handle_terminal_event(event)?
                }
            }
            if draw_needed {
                self.terminal_manager.terminal.draw(|frame| {
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
                self.submit_key_event(&key_event, NoReactions)?;
                Ok(true)
            }
            Event::Mouse(mouse_event) => {
                self.submit_mouse_event(&mouse_event, NoReactions)?;
                Ok(true)
            }
            Event::Resize(_, _) => {
                // resized terminal requires a redraw
                // TODO(Menno 23.12.2022) Figure out if redraw is really needed, maybe TUI crate handles it for us
                Ok(true)
            }
            // We don't handle this event, no need to redraw
            _ => Ok(false),
        }
    }

    fn get_main_menu_reactions(&self) -> Box<MainMenuReactions> {
        Box::from(MainMenuReactions {
            exit_application: &|| self.stop_token.request_stop(),
        })
    }
}
