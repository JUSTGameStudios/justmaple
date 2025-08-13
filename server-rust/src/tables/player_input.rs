use spacetimedb::table;

// See: https://docs.rs/spacetimedb/latest/spacetimedb/attr.table.html for table definitions
#[table(name = player_input, public)]
pub struct PlayerInput {
    #[primary_key]
    pub player_id: u32,
    pub horizontal_axis: f32,       // -1.0 to 1.0 (A/D keys), clamped server-side
    pub jump_pressed: bool,         // Space key state
    pub input_sequence: u32,        // Anti-cheat sequence numbering for input validation
}