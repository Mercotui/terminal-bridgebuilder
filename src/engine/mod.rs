#[cfg(test)]
mod unittest;

use crate::level::Level;
use crate::scene::{Object, Scene};
use anyhow::{ensure, Result};

pub enum SimulationState {
    Paused,
    Running,
    _Completed,
    Cleared,
}

pub struct Engine {
    level: Level,
    static_scene: Scene,
    simulation_scene: Scene,
    simulation_state: SimulationState,
}

impl Engine {
    pub(crate) fn new() -> Engine {
        Engine {
            level: Level::default(),
            static_scene: Scene::default(),
            simulation_scene: Scene::default(),
            simulation_state: SimulationState::Cleared,
        }
    }

    pub fn set_level(&mut self, new_level: Level) -> Result<()> {
        self.level = new_level;
        self.static_scene = Scene::new(&self.level)?;

        // TODO (Menno 26.12.2022) This copy should be done at simulation start instead,
        //  once world menu has been created
        self.simulation_scene = self.static_scene.clone();
        Ok(())
    }

    pub fn get_scene(&self) -> &Scene {
        &self.simulation_scene
    }

    pub fn start_simulation(&mut self) {
        self.simulation_state = SimulationState::Running;
    }

    pub fn pause_simulation(&mut self) {
        self.simulation_state = SimulationState::Paused;
    }

    pub fn reset_simulation(&mut self) {
        self.simulation_scene = self.static_scene.clone();
        self.simulation_state = SimulationState::Cleared;
    }

    pub fn simulation_state(&self) -> &SimulationState {
        &self.simulation_state
    }

    pub fn simulate_scene_tick(&mut self) -> Result<()> {
        ensure!(matches!(self.simulation_state, SimulationState::Running));
        self.the_fancy_math();
        Ok(())
    }

    fn the_fancy_math(&mut self) {
        const GRAVITY: f64 = 0.1;
        for object in &mut self.simulation_scene.0 {
            match object {
                Object::Wire(wire) => {
                    wire.line.0.y -= GRAVITY;
                    wire.line.1.y -= GRAVITY;
                }
                Object::Beam(beam) => {
                    if !beam.is_static {
                        beam.line.0.y -= GRAVITY;
                        beam.line.1.y -= GRAVITY;
                    }
                }
                Object::_Vehicle(vehicle) => {
                    vehicle.position.y -= GRAVITY;
                }
            }
        }
    }
}
