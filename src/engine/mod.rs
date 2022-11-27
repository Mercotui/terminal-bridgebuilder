use crate::level::{Level, Object};

pub struct Engine {
    objects: Vec<Object>,
}

impl Engine {
    pub(crate) fn new() -> Engine {
        Engine { objects: vec![] }
    }

    pub fn set_level(new_level: &Level) {}

    pub fn get_scene(&self) -> &Vec<Object> {
        &self.objects
    }

    fn the_fancy_math() {}
}
