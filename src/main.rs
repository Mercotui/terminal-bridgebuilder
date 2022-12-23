use crate::engine::Engine;
use crate::stop_token::StopToken;
use crate::ui::Gui;
use anyhow::Result;
use clap::Parser;
use std::rc::Rc;

mod engine;
mod level;
mod savefile;
mod scene;
mod stop_token;
mod ui;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The path to a level to load
    level_path: std::path::PathBuf,
    /// The level of logging verbosity
    #[clap(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
}

fn main() -> Result<()> {
    env_logger::init();
    let args = Cli::parse();

    let _objects = savefile::load(&args.level_path)?;
    let _engine = Engine::new();

    let stop_token = Rc::new(StopToken::new());
    let mut ui = Gui::new(stop_token.clone())?;
    ui.run()
}
