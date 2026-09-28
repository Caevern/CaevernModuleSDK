use crate::objects::object::Object;

#[link(wasm_import_module = "env")]
unsafe extern "C" {
    #[link_name = "log"]
    fn game_log(ptr: u32, len: u32);

    #[link_name = "create_mesh"]
    fn game_create_mesh(
        vertices_ptr: u32,
        vertices_len: u32,
        indices_ptr: u32,
        indices_len: u32,
    ) -> u32;
}

pub fn log(message: &str) {
    unsafe {
        game_log(message.as_ptr() as u32, message.len() as u32);
    }
}

pub fn create_mesh_object(vertices: &[f32], indices: &[u32]) -> Object {
    unsafe {
        let object_id = game_create_mesh(
            vertices.as_ptr() as u32,
            vertices.len() as u32,
            indices.as_ptr() as u32,
            indices.len() as u32,
        );
        Object::new(object_id as usize)
    }
}
