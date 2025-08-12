use crate::config::*;

pub struct GameMath;

impl GameMath {
    pub fn mass_to_radius(mass: u32) -> f32 {
        (mass as f32).sqrt()
    }

    pub fn mass_to_max_move_speed(mass: u32) -> f32 {
        2.0 * START_PLAYER_SPEED as f32 / (1.0 + ((mass as f32) / (START_PLAYER_MASS as f32)).sqrt())
    }
}