use crate::ui::components::{FocusScope, IconButton};
use anyhow::Context;
use crossterm::event::{KeyCode, KeyEvent};
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::Color;
use tui::widgets::{Block, Borders};
use tui::Frame;

pub struct WorldMenu {
    focused_button_idx: Option<usize>,
    simulation_controls: Vec<IconButton>,
}

impl FocusScope for WorldMenu {
    fn handle_key_event(&mut self, key_event: &KeyEvent) -> anyhow::Result<bool> {
        match key_event.code {
            KeyCode::Left => {
                if let Some(idx) = self.focused_button_idx {
                    self.focused_button_idx = {
                        if idx == 0 {
                            // TODO(Menno 01.01.2023) Use full set of menu items
                            Some(self.simulation_controls.len() - 1)
                        } else {
                            Some(idx - 1)
                        }
                    }
                }
                Ok(true)
            }
            KeyCode::Right => {
                if let Some(idx) = self.focused_button_idx {
                    self.focused_button_idx = {
                        if idx == self.simulation_controls.len() - 1 {
                            // TODO(Menno 01.01.2023) Use full set of menu items
                            Some(0)
                        } else {
                            Some(idx + 1)
                        }
                    }
                }
                Ok(true)
            }
            _ => Ok(false),
        }
    }

    fn determine_focus(&mut self) -> anyhow::Result<Option<&mut dyn FocusScope>> {
        match self.focused_button_idx {
            None => Ok(None),
            Some(idx) => Ok(Some(
                self.simulation_controls
                    .get_mut(idx)
                    .context("Could not access button at focused index")?,
            )),
        }
    }
}

impl WorldMenu {
    pub fn new(
        simulation_start_pause_func: Box<dyn FnMut()>,
        simulation_reset_func: Box<dyn FnMut()>,
    ) -> Self {
        WorldMenu {
            focused_button_idx: Some(0),
            simulation_controls: vec![
                IconButton::new(
                    "Start/Pause (1)".to_string(),
                    |context| {
                        context.draw(&tui::widgets::canvas::Rectangle {
                            x: -0.5,
                            y: -0.5,
                            width: 1.0,
                            height: 1.0,
                            color: Color::Red,
                        })
                    },
                    simulation_start_pause_func,
                ),
                IconButton::new(
                    "Reset (2)".to_string(),
                    |context| {
                        context.draw(&tui::widgets::canvas::Rectangle {
                            x: -0.5,
                            y: -0.5,
                            width: 1.0,
                            height: 1.0,
                            color: Color::Blue,
                        });
                    },
                    simulation_reset_func,
                ),
            ],
        }
    }

    pub(crate) fn draw<B: Backend>(&self, frame: &mut Frame<B>, area: Rect) {
        let block = Block::default().title("World Menu").borders(Borders::ALL);
        frame.render_widget(block, area);

        // TODO(Menno 02.01.2023) These constraints should probably be based on fixed sizes, or a square ratio
        let mut constraints = vec![Constraint::Percentage(10); self.simulation_controls.len()];
        constraints.push(Constraint::Percentage(0));

        let button_layout = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints(constraints)
            .split(area);

        for (i, button) in self.simulation_controls.iter().enumerate() {
            button.draw(frame, button_layout[i], self.focused_button_idx == Some(i));
        }
    }
}
