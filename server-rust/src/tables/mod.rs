pub mod player;
pub mod entity;
pub mod config;
pub mod timers;
// NEW: Physics and movement tables
pub mod physics_body;
pub mod player_input;
pub mod movement_controller;

pub use player::*;
pub use entity::*;
pub use config::*;
pub use timers::*;
// NEW: Physics and movement exports
pub use physics_body::*;
pub use player_input::*;
pub use movement_controller::*;