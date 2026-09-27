#[link(wasm_import_module = "env")]
unsafe extern "C" {
    #[link_name = "log"]
    fn game_log(ptr: u32, len: u32);
}

pub fn log(message: &str) {
    unsafe {
        game_log(message.as_ptr() as u32, message.len() as u32);
    }
}
