use crate::object::Object;

pub struct Engine {
    objects: Vec<Object>,
}

impl Engine {
    fn new() -> Engine {}

    fn get_objects(&self) -> &Vec<Object> {
        &self.objects
    }
}
