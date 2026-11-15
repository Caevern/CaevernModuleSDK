use crate::game::get_player_position;

#[derive(Clone, Copy)]
pub struct Player {}
impl Player {
    pub fn get_position(&self) -> (f32, f32, f32) {
        get_player_position()
    }
}
