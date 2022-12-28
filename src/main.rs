use crate::stop_token::StopToken;
use crate::ui::Gui;
use anyhow::Result;
use clap::Parser;
use signal_hook::{consts::SIGINT, iterator::Signals};
use std::sync::Arc;
use std::thread;

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

fn install_signal_handler(stop_token: Arc<StopToken>) -> Result<()> {
    let mut signals = Signals::new(&[SIGINT])?;

    thread::spawn(move || {
        for _sig in signals.forever() {
            stop_token.request_stop();
        }
    });
    Ok(())
}

fn main() -> Result<()> {
    env_logger::init();
    let args = Cli::parse();

    let stop_token = Arc::new(StopToken::new());
    install_signal_handler(stop_token.clone())?;

    let mut ui = Gui::new(stop_token.clone(), Some(&args.level_path))?;
    ui.run()
}
