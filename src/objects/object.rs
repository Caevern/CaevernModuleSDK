use crate::objects::texture::Texture;

pub struct Object {
    object_id: usize,
    texture: Option<Texture>
}

impl Object {
    pub fn new(object_id: usize) -> Self {
        Self {
            object_id,
            texture: None
        }
    }

    pub fn get_id(&self) -> usize {
        self.object_id
    }

    pub fn set_texture(&mut self, texture: Texture) {
        self.texture = Some(texture);
    }
}
