use crate::scene::{BeamMaterial, Object, Scene, WireMaterial};
use tui::backend::Backend;
use tui::layout::Rect;
use tui::style::Color;
use tui::widgets::canvas::{Canvas, Line};
use tui::Frame;

pub struct WorldView {}

impl WorldView {
    pub(crate) fn draw<B: Backend>(&self, scene: &Scene, frame: &mut Frame<B>, area: Rect) {
        let canvas = Canvas::default()
            // TODO(Menno 28.12.2022) Calculate bounding box of scene automatically, which should possibly only care about roads
            .x_bounds([0.0, 3.0])
            .y_bounds([0.0, 3.0])
            .paint(|ctx| {
                for object in &scene.0 {
                    match object {
                        Object::WireObject(wire) => {
                            let color;
                            match wire.material {
                                WireMaterial::Steel => color = Color::Gray,
                            }
                            ctx.draw(&Line {
                                x1: wire.line.0.x,
                                y1: wire.line.0.y,
                                x2: wire.line.1.x,
                                y2: wire.line.1.y,
                                color,
                            });
                        }
                        Object::BeamObject(beam) => {
                            let color;
                            match beam.material {
                                BeamMaterial::Wood => color = Color::Red,
                                BeamMaterial::Steel => color = Color::DarkGray,
                                BeamMaterial::Road => color = Color::Black,
                            }
                            ctx.draw(&Line {
                                x1: beam.line.0.x,
                                y1: beam.line.0.y,
                                x2: beam.line.1.x,
                                y2: beam.line.1.y,
                                color,
                            });
                        }
                        Object::_VehicleObject(_) => {}
                    }
                }
            });
        frame.render_widget(canvas, area);
    }
}
