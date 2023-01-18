use crate::scene::{BeamMaterial, Line, Object, Scene, WireMaterial};
use crate::ui::components::FocusScope;
use crossterm::event::{KeyCode, KeyEvent};
use tui::backend::Backend;
use tui::layout::Rect;
use tui::style::Color;
use tui::widgets::canvas;
use tui::widgets::canvas::{Canvas, Context};
use tui::Frame;

#[derive(Default)]
pub struct WorldView {
    is_edit_mode_active: bool,
}

impl FocusScope for WorldView {
    fn handle_key_event(&mut self, key_event: &KeyEvent) -> anyhow::Result<bool> {
        match key_event.code {
            KeyCode::Esc => {
                self.is_edit_mode_active = false;
                Ok(true)
            }
            _ => Ok(false),
        }
    }
}

impl WorldView {
    pub(crate) fn is_edit_mode_active(&self) -> bool {
        self.is_edit_mode_active
    }

    pub(crate) fn draw<B: Backend>(&self, scene: &Scene, frame: &mut Frame<B>, area: Rect) {
        let canvas = Canvas::default()
            // TODO(Menno 28.12.2022) Calculate bounding box of scene automatically, which should possibly only care about roads
            .x_bounds([0.0, 3.0])
            .y_bounds([0.0, 3.0])
            .paint(|ctx| {
                for object in &scene.0 {
                    match object {
                        Object::Wire(wire) => {
                            let color = match wire.material {
                                WireMaterial::Steel => Color::Gray,
                            };
                            Self::draw_line(ctx, &wire.line, color)
                        }
                        Object::Beam(beam) => {
                            let color = match beam.material {
                                BeamMaterial::Wood => Color::Red,
                                BeamMaterial::Steel => Color::DarkGray,
                                BeamMaterial::Road => Color::Black,
                            };
                            Self::draw_line(ctx, &beam.line, color);
                        }
                        Object::Vehicle(vehicle) => ctx.draw(&canvas::Rectangle {
                            x: vehicle.position.x,
                            y: vehicle.position.y,
                            width: 0.3,
                            height: 0.2,
                            color: Color::LightYellow,
                        }),
                    }
                }
            });
        frame.render_widget(canvas, area);
    }

    fn draw_line(context: &mut Context, line: &Line, color: Color) {
        context.draw(&canvas::Line {
            x1: line.0.x,
            y1: line.0.y,
            x2: line.1.x,
            y2: line.1.y,
            color,
        });
    }
}
