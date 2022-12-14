use crate::engine::Engine;
use crate::ui::Gui;
use anyhow::Result;
use clap::Parser;

mod engine;
mod level;
mod savefile;
mod scene;
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

    let objects = savefile::load(&args.level_path)?;
    let engine = Engine::new();

    let mut ui = Gui::new()?;
    ui.run()
}
