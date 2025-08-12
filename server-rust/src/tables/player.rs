use spacetimedb::{table, Identity};

// See: https://docs.rs/spacetimedb/latest/spacetimedb/attr.table.html for table definitions
#[table(name = logged_out_player)]
pub struct LoggedOutPlayer {
    #[primary_key]
    pub identity: Identity,
    #[unique]
    #[auto_inc] 
    pub player_id: u32,
    pub name: String,
}

#[table(name = player, public)]
pub struct Player {
    #[primary_key]
    pub identity: Identity,
    #[unique]
    #[auto_inc]
    pub player_id: u32,
    pub name: String,
}