use crate::objects::{object::Object, player::Player, texture::Texture};

#[link(wasm_import_module = "env")]
unsafe extern "C" {
    #[link_name = "log"]
    fn game_log(ptr: u32, len: u32);

    #[link_name = "get_time"]
    fn game_get_time() -> u32;

    #[link_name = "create_mesh_indexed"]
    fn game_create_mesh_indexed(
        vertices_ptr: u32,
        vertices_len: u32,
        uvs_ptr: u32,
        uvs_len: u32,
        indices_ptr: u32,
        indices_len: u32,
    ) -> u32;

    #[link_name = "create_mesh"]
    fn game_create_mesh(vertices_ptr: u32, vertices_len: u32, uvs_ptr: u32, uvs_len: u32) -> u32;

    #[link_name = "create_texture_rgba8"]
    fn game_create_texture_rgba8(pixels_ptr: u32, pixels_len: u32, width: u32, height: u32) -> u32;

    #[link_name = "set_texture"]
    fn game_set_texture(object_id: u32, texture_id: u32);

    #[link_name = "get_position"]
    fn game_get_position(object_id: u32, out_position: *mut f32);
    #[link_name = "get_player_position"]
    fn game_get_player_position(out_position: *mut f32);

    #[link_name = "set_position"]
    fn game_set_position(object_id: u32, x: f32, y: f32, z: f32);

    #[link_name = "get_element_by_name"]
    fn game_get_element_by_name(name_ptr: u32, name_len: u32) -> u32;
}

pub fn get_time() -> u32 {
    unsafe { game_get_time() }
}

pub fn log(message: &str) {
    unsafe {
        game_log(message.as_ptr() as u32, message.len() as u32);
    }
}

pub fn create_mesh_object_indexed(
    vertices: &Vec<f32>,
    uvs: &Vec<f32>,
    indices: &Vec<u32>,
) -> Object {
    unsafe {
        let object_id = game_create_mesh_indexed(
            vertices.as_ptr() as u32,
            vertices.len() as u32,
            uvs.as_ptr() as u32,
            uvs.len() as u32,
            indices.as_ptr() as u32,
            indices.len() as u32,
        );
        Object::new(object_id as usize)
    }
}

pub fn create_mesh_object(vertices: &Vec<f32>, uvs: &Vec<f32>) -> Object {
    unsafe {
        let object_id = game_create_mesh(
            vertices.as_ptr() as u32,
            vertices.len() as u32,
            uvs.as_ptr() as u32,
            uvs.len() as u32,
        );
        Object::new(object_id as usize)
    }
}

pub fn create_texture_rgba8(pixels: &[u8], width: u32, height: u32) -> Texture {
    assert!(
        (width as usize)
            .checked_mul(height as usize)
            .and_then(|n| n.checked_mul(4))
            == Some(pixels.len()),
        "Expected width * height * 4 RGBA bytes"
    );

    let texture_id = unsafe {
        game_create_texture_rgba8(pixels.as_ptr() as u32, pixels.len() as u32, width, height)
    };

    Texture::new(texture_id as usize)
}

pub fn set_texture(object_id: usize, texture_id: usize) {
    unsafe {
        game_set_texture(object_id as u32, texture_id as u32);
    }
}

pub fn get_element_by_name(name: &str) -> Option<Object> {
    unsafe {
        let object_id = game_get_element_by_name(name.as_ptr() as u32, name.len() as u32);
        if object_id == 0 {
            None
        } else {
            Some(Object::new(object_id as usize))
        }
    }
}
pub fn get_local_player() -> Player {
    Player {}
}

pub fn get_position(object_id: usize) -> (f32, f32, f32) {
    let mut position = [0.0_f32; 3];

    unsafe {
        game_get_position(object_id as u32, position.as_mut_ptr());
    }

    (position[0], position[1], position[2])
}
pub fn get_player_position() -> (f32, f32, f32) {
    let mut position = [0.0_f32; 3];

    unsafe {
        game_get_player_position(position.as_mut_ptr());
    }

    (position[0], position[1], position[2])
}

pub fn set_position(object_id: usize, x: f32, y: f32, z: f32) {
    unsafe {
        game_set_position(object_id as u32, x, y, z);
    }
}
