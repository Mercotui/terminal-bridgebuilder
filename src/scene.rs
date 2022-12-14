use crate::level::Coordinates;

/// A generic object in the scene
#[derive(Debug, PartialEq)]
pub struct Object {
    pub name: String,
    pub is_static: bool,
    pub location: Coordinates,
    pub depth: u64,
}

/// A level transformed into renderable objects
pub struct Scene(Vec<Object>);
