use anyhow::{Context, Result};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use std::{
    io,
    time::{Duration, Instant},
};
use tui::{backend::CrosstermBackend, Terminal};

pub struct TerminalManager {
    pub terminal: Terminal<CrosstermBackend<io::Stdout>>,
    pub last_tick: Instant,
}

impl Drop for TerminalManager {
    fn drop(&mut self) {
        disable_raw_mode().expect("Can not reset terminal mode");
        execute!(
            self.terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )
        .expect("Can not reset terminal");
        self.terminal.show_cursor().expect("Can not reset cursor");
    }
}

impl TerminalManager {
    pub fn new() -> Result<TerminalManager> {
        enable_raw_mode().context("Can not enable terminal raw mode")?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)
            .context("Can not set up terminal for full screen drawing")?;
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend).context("Can not create terminal backend")?;

        Ok(TerminalManager {
            terminal,
            last_tick: Instant::now(),
        })
    }

    pub fn next(&mut self) -> Result<bool> {
        let tick_rate = Duration::from_millis(250);
        let timeout = tick_rate
            .checked_sub(self.last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Esc => return Ok(false),
                    _ => {}
                }
            }
        }
        if self.last_tick.elapsed() >= tick_rate {
            // app.on_tick();
            self.last_tick = Instant::now();
        }

        Ok(true)
    }

    // TODO (Menno 14.12.2022) Figure out how to pass this closure as a parameter so that terminal doesn't have to be
    //  public.
    // pub fn draw(
    //     &mut self,
    //     draw_func: Box<dyn FnOnce(&mut Frame<CrosstermBackend<Stdout>>)>,
    // ) -> Result<()> {
    //     self.terminal.draw(draw_func)?;
    //     Ok(())
    // }
}
