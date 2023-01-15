use crate::stop_token::StopToken;
use crate::ui::Gui;
use anyhow::{Context, Result};
use clap::Parser;
use signal_hook::{consts::SIGINT, iterator::Signals};
use std::io;
use std::ops::Deref;
use std::panic;
use std::sync::Arc;
use std::thread;
use tracing::error;
use tracing_subscriber::{EnvFilter, FmtSubscriber};

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

fn configure_logger() -> Result<()> {
    // Create log printer that writes to STDERR
    let subscriber = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .with_writer(io::stderr)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .context("setting default subscriber failed")?;

    // Configure logging of panics
    panic::set_hook(Box::new(|panic_info| {
        let (filename, line) = panic_info
            .location()
            .map(|loc| (loc.file(), loc.line()))
            .unwrap_or(("<unknown>", 0));

        let cause = panic_info
            .payload()
            .downcast_ref::<String>()
            .map(String::deref);

        let cause = cause.unwrap_or_else(|| {
            panic_info
                .payload()
                .downcast_ref::<&str>()
                .copied()
                .unwrap_or("<cause unknown>")
        });

        error!("A panic occurred at {}:{}: {}", filename, line, cause);
    }));

    Ok(())
}

fn main() -> Result<()> {
    let args = Cli::parse();
    configure_logger()?;

    let stop_token = Arc::new(StopToken::new());
    install_signal_handler(stop_token.clone())?;

    let mut ui = Gui::new(stop_token.clone(), Some(&args.level_path))?;
    ui.run()
}
