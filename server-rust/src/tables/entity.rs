use spacetimedb::{table, SpacetimeType};
use crate::types::DbVector2;

// See: https://docs.rs/spacetimedb/latest/spacetimedb/derive.SpacetimeType.html for custom types
#[derive(SpacetimeType, Clone, Copy, Debug, PartialEq)]
pub enum EntityType {
    Player,
    // Future: NPCs, Items, etc.
}

// See: https://docs.rs/spacetimedb/latest/spacetimedb/attr.table.html for table definitions
#[table(name = entity, public)]
pub struct Entity {
    #[primary_key]
    #[auto_inc]
    pub entity_id: u32,
    pub position: DbVector2,
    pub velocity: DbVector2,      // NEW: Physics velocity for Rapier2D sync
    pub mass: u32,
    pub entity_type: EntityType,  // NEW: Type classification for MMORPG extensibility
}
