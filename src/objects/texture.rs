pub struct Texture {
    texture_id: usize,
}

impl Texture {
    pub fn new(texture_id: usize) -> Self {
        Self { texture_id }
    }

    pub fn get_id(&self) -> usize {
        self.texture_id
    }
}
