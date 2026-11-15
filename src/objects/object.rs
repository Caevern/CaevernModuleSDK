use crate::{game::set_position, objects::texture::Texture};

pub struct Object {
    object_id: usize,
    texture: Option<Texture>,
}

impl Object {
    pub fn new(object_id: usize) -> Self {
        Self {
            object_id,
            texture: None,
        }
    }

    pub fn get_id(&self) -> usize {
        self.object_id
    }

    pub fn set_texture(&mut self, texture: Texture) {
        self.texture = Some(texture);
    }
    pub fn set_position(&mut self, x: f32, y: f32, z: f32) {
        set_position(self.object_id, x, y, z);
    }
}
