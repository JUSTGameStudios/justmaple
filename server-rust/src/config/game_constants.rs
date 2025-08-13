// Game configuration constants

// Physics constants
pub const PHYSICS_TIMESTEP: f32 = 0.02; // 50Hz physics simulation (1/50 = 0.02)
pub const GRAVITY: f32 = -9.81;


// Player platformer constants
pub const START_PLAYER_MASS: u32 = 15;
pub const PLAYER_MOVE_SPEED: f32 = 5.0;  // Horizontal movement speed
pub const PLAYER_JUMP_FORCE: f32 = 8.0;  // Jump impulse strength

// Game mechanics constants  
pub const MINIMUM_SAFE_MASS_RATIO: f32 = 0.85;

// World configuration
pub const DEFAULT_WORLD_SIZE: u64 = 1000;

// Timer intervals (in milliseconds)
pub const PHYSICS_STEP_INTERVAL_MS: i32 = 20;  // 50Hz physics = 20ms intervals
