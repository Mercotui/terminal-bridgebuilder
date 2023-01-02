use anyhow::{Context, Result};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use std::{
    io,
    time::{Duration, Instant},
};
use tui::{backend::CrosstermBackend, Terminal};

pub enum TerminalManagerEvent {
    TerminalEvent(event::Event),
    TickEvent,
}

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

    pub fn next(&mut self) -> Result<TerminalManagerEvent> {
        loop {
            let tick_rate = Duration::from_millis(250);

            // check if it's time for a physics tick
            if self.last_tick.elapsed() >= tick_rate {
                self.last_tick = Instant::now();
                return Ok(TerminalManagerEvent::TickEvent);
            }

            // wait for event or timeout
            let timeout = tick_rate
                .checked_sub(self.last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));
            if crossterm::event::poll(timeout)? {
                return Ok(TerminalManagerEvent::TerminalEvent(event::read()?));
            }
        }
    }

    // TODO (Menno 14.12.2022) Figure out how to pass this closure as a parameter so that terminal doesn't have to be
    //  public.
    // pub fn draw<B: Backend>(
    //     &mut self,
    //     draw_func: Box<dyn FnOnce(&mut Frame<B>)>,
    // ) -> Result<()> {
    //     self.terminal.draw(draw_func)?;
    //     Ok(())
    // }
}
