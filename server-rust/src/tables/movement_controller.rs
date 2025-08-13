use spacetimedb::table;

// See: https://docs.rs/spacetimedb/latest/spacetimedb/attr.table.html for table definitions
#[table(name = movement_controller, public)]  
pub struct MovementController {
    #[primary_key]
    pub entity_id: u32,
    #[index(btree)]
    pub player_id: u32,            // Links to Player table for ownership
    pub move_speed: f32,            // Horizontal movement speed (platformer)
    pub jump_force: f32,            // Jump impulse strength
    pub can_jump: bool,             // Jump availability state (ground check dependent)
}