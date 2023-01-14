#[cfg(test)]
mod unittest;

mod key_handler;
mod mouse_area;

pub use key_handler::KeyHandler;
pub use mouse_area::MouseArea;
pub struct NoReactions;
