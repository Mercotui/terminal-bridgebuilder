use crate::ui::components::{
    ListMenu, ListMenuEntry, ListMenuItem, ListMenuItems, ListMenuReactions, Popup,
};
use crate::ui::event_handling::{KeyHandler, MouseArea};
use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent, MouseEvent};
use std::collections::HashMap;
use std::io::Stdout;
use tui::backend::CrosstermBackend;
use tui::Frame;

pub struct MainMenu {
    list_menu: ListMenu,
}

pub struct MainMenuReactions<'a> {
    pub(crate) exit_application: &'a dyn Fn(),
}

impl<'a> KeyHandler<Box<MainMenuReactions<'a>>> for MainMenu {
    fn forward_key_event(
        &mut self,
        key_event: &KeyEvent,
        reactions: &Box<MainMenuReactions>,
    ) -> Result<bool> {
        let child_reactions = self.get_list_menu_reactions(reactions);
        self.list_menu.submit_key_event(key_event, child_reactions)
    }
}

impl<'a> MouseArea<Box<MainMenuReactions<'a>>> for MainMenu {
    fn forward_mouse_event(
        &mut self,
        mouse_event: &MouseEvent,
        reactions: &Box<MainMenuReactions>,
    ) -> Result<bool> {
        let child_reactions = self.get_list_menu_reactions(reactions);
        self.list_menu
            .submit_mouse_event(mouse_event, child_reactions)
    }
}

impl MainMenu {
    pub fn new() -> MainMenu {
        MainMenu {
            list_menu: ListMenu::new(
                "Main Menu".to_string(),
                ListMenuItems(vec![
                    ListMenuEntry::Item(ListMenuItem {
                        title: "Back to game [Esc]".to_string(),
                        hotkey: KeyCode::Esc,
                    }),
                    ListMenuEntry::Item(ListMenuItem {
                        title: "Load Level".to_string(),
                        hotkey: KeyCode::Char('l'),
                    }),
                    ListMenuEntry::Item(ListMenuItem {
                        title: "Exit to terminal".to_string(),
                        hotkey: KeyCode::Char('q'),
                    }),
                ]),
            ),
        }
    }

    pub fn open(&mut self) {
        self.list_menu.open()
    }

    pub fn is_open(&self) -> bool {
        self.list_menu.is_open()
    }

    pub fn draw(&mut self, frame: &mut Frame<CrosstermBackend<Stdout>>) {
        self.list_menu.draw(frame)
    }

    fn get_list_menu_reactions<'a>(
        &mut self,
        reactions: &MainMenuReactions<'a>,
    ) -> Box<ListMenuReactions> {
        Box::from(ListMenuReactions(HashMap::from([
            (KeyCode::Char('q'), reactions.exit_application),
            (KeyCode::Char('l'), &mut || {
                print!("Level loading not yet implemented")
            }),
            (KeyCode::Esc, &mut || self.list_menu.close()),
        ])))
    }
}
