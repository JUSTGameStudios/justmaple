use spacetimedb::{reducer, ReducerContext, Table, ScheduleAt};
use std::time::Duration;
use std::sync::Mutex;
use crate::tables::{EntityType, PhysicsBody, BodyType, PhysicsStepTimer};
use crate::types::DbVector2;
use crate::config::*;
use crate::physics::PhysicsWorld;
// Import table access traits
use crate::tables::entity::entity;
use crate::tables::physics_body::physics_body;
use crate::tables::movement_controller::movement_controller;
use crate::tables::player_input::player_input;
use crate::tables::timers::physics_step_timer;

// Global physics world - in production, consider using SpacetimeDB's context for storage
// See: https://docs.rs/spacetimedb/latest/spacetimedb/ for state management patterns
static PHYSICS_WORLD: Mutex<Option<PhysicsWorld>> = Mutex::new(None);

// See: https://docs.rs/spacetimedb/latest/spacetimedb/attr.reducer.html for reducer definitions

// Initialize physics world and schedule physics simulation
// This is now called from the main init reducer in game_reducers.rs
pub fn init_physics(ctx: &ReducerContext) -> Result<(), Box<dyn std::error::Error>> {
    log::info!("Initializing physics world...");
    
    // Initialize the global physics world
    {
        let mut world_lock = PHYSICS_WORLD.lock().unwrap();
        *world_lock = Some(PhysicsWorld::new());
        
        // Create ground plane for the world
        if let Some(ref mut world) = *world_lock {
            let ground_position = rapier2d::na::Vector2::new(500.0, -10.0); // Center bottom of 1000x1000 world
            let ground_size = rapier2d::na::Vector2::new(1000.0, 20.0);     // Wide ground platform
            world.create_static_ground(ground_position, ground_size);
        }
    }

    // Schedule 50Hz physics simulation
    ctx.db.physics_step_timer().try_insert(PhysicsStepTimer {
        scheduled_id: 0,
        scheduled_at: ScheduleAt::Interval(Duration::from_millis(PHYSICS_STEP_INTERVAL_MS as u64).into()),
    })?;

    Ok(())
}

// Main physics simulation step - runs at 50Hz
// See: https://docs.rs/spacetimedb/latest/spacetimedb/attr.reducer.html for scheduled reducers
#[reducer]
pub fn physics_step(ctx: &ReducerContext, _timer: PhysicsStepTimer) {
    let mut world_lock = PHYSICS_WORLD.lock().unwrap();
    
    if let Some(ref mut physics_world) = *world_lock {
        // 1. Apply player input forces to physics bodies
        apply_player_inputs(ctx, physics_world);
        
        // 2. Step the physics simulation (20ms = 50Hz)
        physics_world.step();
        
        // 3. Sync physics results back to SpacetimeDB entities
        sync_physics_to_database(ctx, physics_world);
        
        // 4. Update ground detection for jump mechanics
        update_ground_detection(ctx, physics_world);
    }
}

// Apply player input to physics forces
fn apply_player_inputs(ctx: &ReducerContext, physics_world: &mut PhysicsWorld) {
    for input in ctx.db.player_input().iter() {
        // Find all movement controllers for this player
        for controller in ctx.db.movement_controller().player_id().filter(&input.player_id) {
            // Apply horizontal movement force
            if input.horizontal_axis.abs() > 0.01 { // Deadzone
                physics_world.apply_movement_force(
                    controller.entity_id,
                    input.horizontal_axis,
                    controller.move_speed,
                );
            }
            
            // Apply jump force if requested and allowed
            if input.jump_pressed && controller.can_jump {
                let jumped = physics_world.apply_jump_force(controller.entity_id, controller.jump_force);
                
                if jumped {
                    // Update controller to prevent double jumping
                    let mut updated_controller = controller;
                    updated_controller.can_jump = false;
                    ctx.db.movement_controller().entity_id().update(updated_controller);
                }
            }
        }
    }
}

// Sync Rapier2D physics results back to SpacetimeDB entities
fn sync_physics_to_database(ctx: &ReducerContext, physics_world: &PhysicsWorld) {
    for mut entity in ctx.db.entity().iter() {
        if entity.entity_type == EntityType::Player {
            if let Some((position, velocity)) = physics_world.get_body_state(entity.entity_id) {
                // Update entity position and velocity from physics simulation
                entity.position = DbVector2::from_nalgebra(position);
                entity.velocity = DbVector2::from_nalgebra(velocity);
                
                // Update in database - clients will receive via subscription
                ctx.db.entity().entity_id().update(entity);
            }
        }
    }
}

// Update ground detection for jump mechanics
fn update_ground_detection(ctx: &ReducerContext, physics_world: &PhysicsWorld) {
    for mut controller in ctx.db.movement_controller().iter() {
        if let Some(&body_handle) = physics_world.entity_to_body.get(&controller.entity_id) {
            if let Some(body) = physics_world.rigid_body_set.get(body_handle) {
                // Simple ground detection: if vertical velocity is small, assume grounded
                let is_grounded = body.linvel().y <= 0.1 && body.linvel().y >= -0.1;
                
                if is_grounded && !controller.can_jump {
                    controller.can_jump = true;
                    ctx.db.movement_controller().entity_id().update(controller);
                }
            }
        }
    }
}

// Create a physics body for a newly spawned player
pub fn create_player_physics_body(ctx: &ReducerContext, entity_id: u32, position: DbVector2, mass: u32) {
    let mut world_lock = PHYSICS_WORLD.lock().unwrap();
    
    if let Some(ref mut physics_world) = *world_lock {
        let nalgebra_pos = position.to_nalgebra();
        let mass_f32 = mass as f32;
        
        // Create physics body in Rapier2D
        physics_world.create_player_body(entity_id, nalgebra_pos, mass_f32);
        
        // Create physics body record in database
        ctx.db.physics_body().insert(PhysicsBody {
            entity_id,
            body_type: BodyType::Dynamic,
            on_ground: false,
            collision_groups: 0b0001, // Default collision group
        });
    }
}