#[link(wasm_import_module = "env")]
unsafe extern "C" {
    #[link_name = "connect"]
    fn game_connect(world: u32);
}

pub fn connect(world: usize) {
    unsafe {
        game_connect(world as u32);
    }
}
