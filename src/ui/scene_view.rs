use crate::engine::Engine;
use crate::savefile;
use crate::ui::world_menu::WorldMenu;
use crate::ui::world_view::WorldView;
use anyhow::Result;
use std::io::Stdout;
use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::Frame;

pub struct SceneView {
    engine: Engine,
    world_view: WorldView,
    world_menu: WorldMenu,
}

impl SceneView {
    pub fn new(initial_level_path: Option<&std::path::PathBuf>) -> Result<SceneView> {
        let mut new_scene_view = SceneView {
            engine: Engine::new(),
            world_view: WorldView {},
            world_menu: WorldMenu,
        };

        if initial_level_path.is_some() {
            new_scene_view.load_level(initial_level_path.unwrap())?;
        }

        Ok(new_scene_view)
    }

    pub fn load_level(&mut self, level_path: &std::path::PathBuf) -> Result<()> {
        self.engine.set_level(savefile::load(level_path)?)?;
        Ok(())
    }

    pub fn physics_tick(&mut self) -> Result<bool> {
        self.engine.simulate_scene_tick();

        // For now we will always redraw
        Ok(true)
    }

    pub fn draw(&self, frame: &mut Frame<CrosstermBackend<Stdout>>) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Percentage(85), Constraint::Percentage(15)].as_ref())
            .split(frame.size());

        self.world_view
            .draw(self.engine.get_scene(), frame, chunks[0]);
        self.world_menu.draw(frame, chunks[1])
    }
}
