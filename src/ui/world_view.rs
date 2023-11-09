use crate::scene::{BeamMaterial, Coordinates, Line, Object, Scene, VehicleType, WireMaterial};
use crate::ui::components::FocusScope;
use crossterm::event::{KeyCode, KeyEvent};
use euclid;
use iterwindows::IterArrayWindows;
use tracing::error;
use tui::backend::Backend;
use tui::layout::Rect;
use tui::style::Color;
use tui::widgets::canvas;
use tui::widgets::canvas::{Canvas, Context};
use tui::Frame;

struct WorldSpace;
struct VehicleSpace;

type VehiclePoint = euclid::Point2D<f64, VehicleSpace>;
type VehiclePosition = euclid::Transform2D<f64, VehicleSpace, WorldSpace>;

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
                            Self::draw_line(ctx, &wire.line, color);
                        }
                        Object::Beam(beam) => {
                            let color = match beam.material {
                                BeamMaterial::Wood => Color::Red,
                                BeamMaterial::Steel => Color::DarkGray,
                                BeamMaterial::Road => Color::Black,
                            };
                            Self::draw_line(ctx, &beam.line, color);
                        }
                        Object::Vehicle(vehicle) => match vehicle.vehicle_type {
                            VehicleType::Bus => {
                                //TODO(Menno 06.11.2023) Add bus rendering
                                error!("Bus rendering not implemented");
                            }
                            VehicleType::Car => {
                                Self::draw_car(ctx, vehicle.position, vehicle.rotation)
                            }
                        },
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

    fn draw_car(context: &mut Context, position: Coordinates, rotation: f64) {
        let rotation: euclid::Angle<f64> = euclid::Angle::degrees(rotation);
        let translation: euclid::Vector2D<f64, WorldSpace> =
            euclid::Vector2D::new(position.x, position.y);
        let world_transformation: VehiclePosition =
            euclid::Transform2D::rotation(rotation).then_translate(translation);

        let car_body: Vec<VehiclePoint> = vec![
            euclid::point2(-0.1, 0.1),
            euclid::point2(0.1, 0.1),
            euclid::point2(0.1, 0.2),
            euclid::point2(0.04, 0.2),
            euclid::point2(0.0, 0.3),
            euclid::point2(-0.08, 0.3),
            euclid::point2(-0.1, 0.2),
            euclid::point2(-0.1, 0.1),
        ];

        let tire: Vec<VehiclePoint> = vec![
            euclid::point2(-0.02, 0.1),
            euclid::point2(0.0, 0.0),
            euclid::point2(0.02, 0.1),
        ];

        let tires: Vec<euclid::Translation2D<f64, VehicleSpace, VehicleSpace>> = vec![
            euclid::Translation2D::new(-0.08, 0.0),
            euclid::Translation2D::new(0.08, 0.0),
        ];

        // Draw tires first
        for tire_location in tires {
            for [tire_point_1, tire_point_2] in tire.iter().array_windows() {
                let tire_world_point_1 = world_transformation
                    .transform_point(tire_location.transform_point(*tire_point_1));
                let tire_world_point_2 = world_transformation
                    .transform_point(tire_location.transform_point(*tire_point_2));

                context.draw(&canvas::Line {
                    x1: tire_world_point_1.x,
                    y1: tire_world_point_1.y,
                    x2: tire_world_point_2.x,
                    y2: tire_world_point_2.y,
                    color: Color::DarkGray,
                });
            }
        }

        // Then overdraw the car body
        for [point_1, point_2] in car_body.iter().array_windows() {
            let world_point_1 = world_transformation.transform_point(*point_1);
            let world_point_2 = world_transformation.transform_point(*point_2);
            context.draw(&canvas::Line {
                x1: world_point_1.x,
                y1: world_point_1.y,
                x2: world_point_2.x,
                y2: world_point_2.y,
                color: Color::LightYellow,
            });
        }
    }
}
