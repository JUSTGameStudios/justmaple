use crate::config::*;

pub struct GameMath;

impl GameMath {
    pub fn mass_to_radius(mass: u32) -> f32 {
        (mass as f32).sqrt()
    }

    // DEPRECATED: Movement speed is now handled by platformer physics
    // This is kept for compatibility but should be removed in future updates
    pub fn mass_to_max_move_speed(_mass: u32) -> f32 {
        PLAYER_MOVE_SPEED
    }
}