use spacetimedb::ScheduleAt;

// See: https://docs.rs/spacetimedb/latest/spacetimedb/attr.table.html for table definitions
// See: https://docs.rs/spacetimedb/latest/spacetimedb/struct.ScheduleAt.html for scheduled tables

#[spacetimedb::table(name = spawn_food_timer, scheduled(crate::spawn_food))]
pub struct SpawnFoodTimer {
    #[primary_key]
    #[auto_inc]
    pub scheduled_id: u64,
    pub scheduled_at: ScheduleAt,
}

#[spacetimedb::table(name = move_all_players_timer, scheduled(crate::move_all_players))]
pub struct MoveAllPlayersTimer {
    #[primary_key]
    #[auto_inc]
    pub scheduled_id: u64,
    pub scheduled_at: ScheduleAt,
}
