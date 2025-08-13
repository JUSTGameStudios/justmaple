use spacetimedb::{reducer, ReducerContext, Table};
use crate::tables::{Entity, EntityType, MovementController, PlayerInput};
use crate::types::DbVector2;
use crate::config::*;
use crate::reducers::physics_reducers::create_player_physics_body;
// Import table access traits
use crate::tables::player::player;
use crate::tables::entity::entity;
use crate::tables::movement_controller::movement_controller;
use crate::tables::player_input::player_input;
use crate::tables::config::config;

// See: https://docs.rs/spacetimedb/latest/spacetimedb/attr.reducer.html for reducer definitions

#[reducer]
pub fn enter_game(ctx: &ReducerContext, name: String) {
    log::info!("Creating player with name {}", name);
    
    if let Some(mut player) = ctx.db.player().identity().find(&ctx.sender) {
        let player_id = player.player_id;
        player.name = name;
        ctx.db.player().identity().update(player);
        spawn_player_initial_entity(ctx, player_id);
    } else {
        panic!("Player not found");
    }
}

// NEW: Action-based input system for platformer controls
#[reducer]
pub fn update_player_input(ctx: &ReducerContext, horizontal: f32, jump: bool) {
    if let Some(player) = ctx.db.player().identity().find(&ctx.sender) {
        // Validate and clamp horizontal input to prevent cheating
        let clamped_horizontal = horizontal.clamp(-1.0, 1.0);
        
        // Update or insert player input - table will automatically sync to clients
        if let Some(mut existing_input) = ctx.db.player_input().player_id().find(&player.player_id) {
            existing_input.horizontal_axis = clamped_horizontal;
            existing_input.jump_pressed = jump;
            existing_input.input_sequence += 1; // Anti-cheat sequence increment
            ctx.db.player_input().player_id().update(existing_input);
        } else {
            // First input from this player
            ctx.db.player_input().insert(PlayerInput {
                player_id: player.player_id,
                horizontal_axis: clamped_horizontal,
                jump_pressed: jump,
                input_sequence: 1,
            });
        }
    } else {
        log::warn!("Input received from unknown player: {:?}", ctx.sender);
    }
}

fn spawn_player_initial_entity(ctx: &ReducerContext, player_id: u32) -> Entity {
    use spacetimedb::rand::Rng;
    let config = ctx.db.config().id().find(&0).expect("Config not found");
    let world_size = config.world_size;
    
    // Spawn player above ground level
    let x = ctx.rng().gen_range(100.0..world_size as f32 - 100.0);
    let y = 100.0; // Start above ground
    
    spawn_player_at(ctx, player_id, START_PLAYER_MASS, DbVector2::new(x, y))
}

fn spawn_player_at(
    ctx: &ReducerContext,
    player_id: u32,
    mass: u32,
    position: DbVector2,
) -> Entity {
    // Create entity with new physics-aware structure
    let entity = ctx.db.entity().insert(Entity {
        entity_id: 0, // Auto-incremented
        position,
        velocity: DbVector2::zero(), // Start at rest
        mass,
        entity_type: EntityType::Player, // NEW: Specify entity type
    });

    // Create movement controller for platformer mechanics (replaces Circle)
    ctx.db.movement_controller().insert(MovementController {
        entity_id: entity.entity_id,
        player_id,
        move_speed: PLAYER_MOVE_SPEED,
        jump_force: PLAYER_JUMP_FORCE,
        can_jump: false, // Will be updated by ground detection
    });

    // Create physics body in Rapier2D world
    create_player_physics_body(ctx, entity.entity_id, position, mass);

    // Initialize player input state
    ctx.db.player_input().insert(PlayerInput {
        player_id,
        horizontal_axis: 0.0,
        jump_pressed: false,
        input_sequence: 0,
    });

    log::info!("Spawned player entity {} for player {}", entity.entity_id, player_id);
    entity
}