use crate::engine::{Engine, SimulationState};
use crate::savefile;
use crate::ui::components::FocusScope;
use crate::ui::world_menu::WorldMenu;
use crate::ui::world_view::WorldView;
use anyhow::Result;
use std::cell::RefCell;
use std::rc::Rc;
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout};
use tui::Frame;

pub struct SceneView {
    engine: Rc<RefCell<Engine>>,
    world_view: WorldView,
    world_menu: WorldMenu,
}

impl FocusScope for SceneView {
    fn determine_focus(&mut self) -> Result<Option<&mut dyn FocusScope>> {
        if self.world_view.is_edit_mode_active() {
            Ok(Some(&mut self.world_view))
        } else {
            Ok(Some(&mut self.world_menu))
        }
    }
}

impl SceneView {
    pub fn new(initial_level_path: Option<&std::path::PathBuf>) -> Result<SceneView> {
        let engine = Rc::new(RefCell::new(Engine::new()));
        let engine_clone_1 = Rc::clone(&engine);
        let engine_clone_2 = Rc::clone(&engine);
        let mut new_scene_view = SceneView {
            engine,
            world_view: WorldView::default(),
            world_menu: WorldMenu::new(
                Box::from(move || {
                    let mut engine_ref = engine_clone_1.borrow_mut();
                    match engine_ref.simulation_state() {
                        SimulationState::Paused => engine_ref.start_simulation(),
                        SimulationState::Running => engine_ref.pause_simulation(),
                        SimulationState::Cleared => engine_ref.start_simulation(),
                        _ => {}
                    }
                }),
                Box::from(move || engine_clone_2.borrow_mut().reset_simulation()),
            ),
        };

        if initial_level_path.is_some() {
            new_scene_view.load_level(initial_level_path.unwrap())?;
        }

        Ok(new_scene_view)
    }

    pub fn load_level(&mut self, level_path: &std::path::PathBuf) -> Result<()> {
        self.engine
            .borrow_mut()
            .set_level(savefile::load(level_path)?)?;
        Ok(())
    }

    pub fn physics_tick(&mut self) -> Result<bool> {
        let mut engine_ref = self.engine.borrow_mut();
        match engine_ref.simulation_state() {
            SimulationState::Running => {
                engine_ref.simulate_scene_tick()?;
                Ok(true)
            }
            _ => Ok(false),
        }
    }

    pub fn draw<B: Backend>(&self, frame: &mut Frame<B>) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Percentage(85), Constraint::Percentage(15)].as_ref())
            .split(frame.size());

        self.world_view
            .draw(self.engine.borrow_mut().get_scene(), frame, chunks[0]);
        self.world_menu.draw(frame, chunks[1])
    }
}
