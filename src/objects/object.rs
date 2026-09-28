pub struct Object {
    object_id: usize,
}

impl Object {
    pub fn new(object_id: usize) -> Self {
        Self { object_id }
    }
}
