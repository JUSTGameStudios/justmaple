use spacetimedb::ScheduleAt;
#[spacetimedb::table(name = physics_step_timer, scheduled(crate::physics_step))]
pub struct PhysicsStepTimer {
    #[primary_key]
    #[auto_inc]
    pub scheduled_id: u64,
    pub scheduled_at: ScheduleAt,
}
