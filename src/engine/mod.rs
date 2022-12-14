#[cfg(test)]
mod unittest;

use crate::level::Level;
use crate::scene::Object;

pub struct Engine {
    objects: Vec<Object>,
}

impl Engine {
    pub(crate) fn new() -> Engine {
        Engine { objects: vec![] }
    }

    pub fn set_level(_new_level: &Level) {}

    pub fn get_scene(&self) -> &Vec<Object> {
        &self.objects
    }

    fn the_fancy_math() {}
}
