use spacetimedb::{reducer, ReducerContext, Timestamp, Table};
use crate::tables::{Entity, Circle};
use crate::types::DbVector2;
use crate::config::*;
use crate::utils::GameMath;
// Import table access traits
use crate::tables::player::player;
use crate::tables::circle::circle;
use crate::tables::entity::entity;
use crate::tables::config::config;

// See: https://docs.rs/spacetimedb/latest/spacetimedb/attr.reducer.html for reducer definitions

#[reducer]
pub fn enter_game(ctx: &ReducerContext, name: String) {
    log::info!("Creating player with name {}", name);
    
    if let Some(mut player) = ctx.db.player().identity().find(&ctx.sender) {
        let player_id = player.player_id;
        player.name = name;
        ctx.db.player().identity().update(player);
        spawn_player_initial_circle(ctx, player_id);
    } else {
        panic!("Player not found");
    }
}

#[reducer]
pub fn update_player_input(ctx: &ReducerContext, direction: DbVector2) {
    if let Some(player) = ctx.db.player().identity().find(&ctx.sender) {
        for circle in ctx.db.circle().player_id().filter(&player.player_id) {
            let mut updated_circle = circle;
            updated_circle.direction = direction.normalized();
            updated_circle.speed = direction.magnitude().min(1.0);
            ctx.db.circle().entity_id().update(updated_circle);
        }
    } else {
        panic!("Player not found");
    }
}

fn spawn_player_initial_circle(ctx: &ReducerContext, player_id: u32) -> Entity {
    use spacetimedb::rand::Rng;
    let config = ctx.db.config().id().find(&0).expect("Config not found");
    let world_size = config.world_size;
    let player_start_radius = GameMath::mass_to_radius(START_PLAYER_MASS);
    
    let x = ctx.rng().gen_range(player_start_radius..world_size as f32 - player_start_radius);
    let y = ctx.rng().gen_range(player_start_radius..world_size as f32 - player_start_radius);
    
    spawn_circle_at(
        ctx,
        player_id,
        START_PLAYER_MASS,
        DbVector2::new(x, y),
        ctx.timestamp,
    )
}

fn spawn_circle_at(
    ctx: &ReducerContext,
    player_id: u32,
    mass: u32,
    position: DbVector2,
    timestamp: Timestamp,
) -> Entity {
    let entity = ctx.db.entity().insert(Entity {
        entity_id: 0, // Auto-incremented
        position,
        mass,
    });

    ctx.db.circle().insert(Circle {
        entity_id: entity.entity_id,
        player_id,
        direction: DbVector2::new(0.0, 1.0),
        speed: 0.0,
        last_split_time: timestamp,
    });

    entity
}