#[cfg(test)]
mod unittest;

use std::sync::atomic::{AtomicBool, Ordering};

pub struct StopToken {
    stop_requested: AtomicBool,
}

impl StopToken {
    pub fn new() -> StopToken {
        StopToken {
            stop_requested: AtomicBool::new(false),
        }
    }

    pub fn request_stop(&self) {
        self.stop_requested.store(true, Ordering::Relaxed);
    }

    pub fn is_stop_requested(&self) -> bool {
        self.stop_requested.load(Ordering::Relaxed)
    }

    pub fn keep_running(&self) -> bool {
        !self.is_stop_requested()
    }
}
