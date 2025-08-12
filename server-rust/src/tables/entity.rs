use spacetimedb::table;
use crate::types::DbVector2;

// See: https://docs.rs/spacetimedb/latest/spacetimedb/attr.table.html for table definitions
#[table(name = entity, public)]
pub struct Entity {
    #[primary_key]
    #[auto_inc]
    pub entity_id: u32,
    pub position: DbVector2,
    pub mass: u32,
}