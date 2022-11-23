use crate::ui::GUI;
use anyhow::Result;
use clap::Parser;
use log;
use std::io;

mod engine;
mod object;
mod savefile;
mod ui;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
    /// The level of logging verbosity
    #[clap(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
}

fn main() -> Result<(), io::Error> {
    env_logger::init();
    let args = Cli::parse();

    let objects = savefile::load(&args.path, &args.pattern);

    let ui = GUI::new();

    log::info!(
        "Hello, world!\nThe arguments I got were pattern={}, and path={}",
        args.pattern,
        args.path.display()
    );

    Ok(())
}
