use spacetimedb::{reducer, ReducerContext, Table, ScheduleAt};
use std::time::Duration;
use crate::tables::{Config, Entity, Food, SpawnFoodTimer, MoveAllPlayersTimer};
use crate::types::DbVector2;
use crate::config::*;
use crate::utils::{GameMath, CollisionDetection};
// Import table access traits
use crate::tables::config::config;
use crate::tables::entity::entity;
use crate::tables::food::food;
use crate::tables::circle::circle;
use crate::tables::player::player;
use crate::tables::timers::{spawn_food_timer, move_all_players_timer};

// See: https://docs.rs/spacetimedb/latest/spacetimedb/attr.reducer.html for reducer definitions

#[reducer(init)]
pub fn init(ctx: &ReducerContext) -> Result<(), Box<dyn std::error::Error>> {
    log::info!("Initializing...");
    
    ctx.db.config().insert(Config {
        id: 0,
        world_size: DEFAULT_WORLD_SIZE,
    });

    // Schedule periodic food spawning
    ctx.db.spawn_food_timer().try_insert(SpawnFoodTimer {
        scheduled_id: 0,
        scheduled_at: ScheduleAt::Interval(Duration::from_millis(FOOD_SPAWN_INTERVAL_MS as u64).into()),
    })?;

    // Schedule periodic player movement
    ctx.db.move_all_players_timer().try_insert(MoveAllPlayersTimer {
        scheduled_id: 0,
        scheduled_at: ScheduleAt::Interval(Duration::from_millis(PLAYER_MOVEMENT_INTERVAL_MS as u64).into()),
    })?;

    Ok(())
}

#[reducer]
pub fn spawn_food(ctx: &ReducerContext, _timer: SpawnFoodTimer) {
    // Are there no players yet?
    if ctx.db.player().count() == 0 {
        return;
    }

    use spacetimedb::rand::Rng;
    let config = ctx.db.config().id().find(&0).expect("Config not found");
    let world_size = config.world_size;
    let food_count = ctx.db.food().count();

    let mut current_food_count = food_count;
    while current_food_count < TARGET_FOOD_COUNT as u64 {
        let food_mass = ctx.rng().gen_range(FOOD_MASS_MIN..FOOD_MASS_MAX + 1); // +1 because range is exclusive
        let food_radius = GameMath::mass_to_radius(food_mass);
        let x = ctx.rng().gen_range(food_radius..world_size as f32 - food_radius);
        let y = ctx.rng().gen_range(food_radius..world_size as f32 - food_radius);
        
        let entity = ctx.db.entity().insert(Entity {
            entity_id: 0, // Auto-incremented
            position: DbVector2::new(x, y),
            mass: food_mass,
        });

        ctx.db.food().insert(Food {
            entity_id: entity.entity_id,
        });

        current_food_count += 1;
        log::info!("Spawned food! {}", entity.entity_id);
    }
}

#[reducer]
pub fn move_all_players(ctx: &ReducerContext, _timer: MoveAllPlayersTimer) {
    let config = ctx.db.config().id().find(&0).expect("Config not found");
    let world_size = config.world_size as f32;

    // Handle player input
    for circle in ctx.db.circle().iter() {
        if let Some(mut circle_entity) = ctx.db.entity().entity_id().find(&circle.entity_id) {
            let circle_radius = GameMath::mass_to_radius(circle_entity.mass);
            let direction = circle.direction * circle.speed;
            let move_speed = GameMath::mass_to_max_move_speed(circle_entity.mass);
            let new_pos = circle_entity.position + direction * move_speed;
            
            // Clamp position to world bounds
            circle_entity.position.x = new_pos.x.clamp(circle_radius, world_size - circle_radius);
            circle_entity.position.y = new_pos.y.clamp(circle_radius, world_size - circle_radius);

            // Check collisions
            for entity in ctx.db.entity().iter() {
                if entity.entity_id == circle_entity.entity_id {
                    continue;
                }

                if CollisionDetection::is_overlapping(&circle_entity, &entity) {
                    // Check to see if we're overlapping with food
                    if ctx.db.food().entity_id().find(&entity.entity_id).is_some() {
                        ctx.db.entity().entity_id().delete(&entity.entity_id);
                        ctx.db.food().entity_id().delete(&entity.entity_id);
                        circle_entity.mass += entity.mass;
                    }

                    // Check to see if we're overlapping with another circle owned by another player
                    if let Some(other_circle) = ctx.db.circle().entity_id().find(&entity.entity_id) {
                        if other_circle.player_id != circle.player_id {
                            let mass_ratio = entity.mass as f32 / circle_entity.mass as f32;
                            if mass_ratio < MINIMUM_SAFE_MASS_RATIO {
                                ctx.db.entity().entity_id().delete(&entity.entity_id);
                                ctx.db.circle().entity_id().delete(&entity.entity_id);
                                circle_entity.mass += entity.mass;
                            }
                        }
                    }
                }
            }
            
            ctx.db.entity().entity_id().update(circle_entity);
        }
    }
}