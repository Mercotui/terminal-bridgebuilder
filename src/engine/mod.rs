#[cfg(test)]
mod unittest;

use crate::level::Level;
use crate::scene::Scene;
use anyhow::Result;

pub struct Engine {
    level: Level,
    static_scene: Scene,
    simulation_scene: Scene,
}

impl Engine {
    pub(crate) fn new() -> Engine {
        Engine {
            level: Level::default(),
            static_scene: Scene::default(),
            simulation_scene: Scene::default(),
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

    pub fn _reset_simulation(&mut self) {
        self.simulation_scene = self.static_scene.clone();
    }

    pub fn simulate_scene_tick(&mut self) {
        self.the_fancy_math();
    }

    fn the_fancy_math(&mut self) {}
}
