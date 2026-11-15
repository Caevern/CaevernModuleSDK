use crate::game::{get_position, set_position};

#[derive(Clone, Copy)]
pub struct Object {
    object_id: usize,
}
impl Object {
    pub fn new(object_id: usize) -> Self {
        Self { object_id }
    }

    pub fn get_id(&self) -> usize {
        self.object_id
    }
    pub fn get_position(&self) -> (f32, f32, f32) {
        get_position(self.object_id)
    }
    pub fn set_position(&mut self, x: f32, y: f32, z: f32) {
        set_position(self.object_id, x, y, z);
    }
}
