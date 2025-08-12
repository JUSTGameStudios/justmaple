use spacetimedb::table;

// See: https://docs.rs/spacetimedb/latest/spacetimedb/attr.table.html for table definitions
#[table(name = food, public)]
pub struct Food {
    #[primary_key]
    pub entity_id: u32,
}