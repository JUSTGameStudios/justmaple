// See: https://docs.rs/rapier2d/latest/rapier2d/ for Rapier2D Physics Engine
use rapier2d::prelude::*;
// Import Vector2 explicitly from nalgebra re-export
use rapier2d::na::Vector2;
use std::collections::HashMap;

// Physics world management for server-authoritative simulation
pub struct PhysicsWorld {
    pub rigid_body_set: RigidBodySet,
    pub collider_set: ColliderSet,
    pub physics_pipeline: PhysicsPipeline,
    pub island_manager: IslandManager,
    pub broad_phase: DefaultBroadPhase,
    pub narrow_phase: NarrowPhase,
    pub impulse_joint_set: ImpulseJointSet,
    pub multibody_joint_set: MultibodyJointSet,
    pub ccd_solver: CCDSolver,
    pub query_pipeline: QueryPipeline,
    
    // Mapping between SpacetimeDB entity IDs and Rapier2D handles
    pub entity_to_body: HashMap<u32, RigidBodyHandle>,
    pub body_to_entity: HashMap<RigidBodyHandle, u32>,
    
    pub gravity: Vector2<f32>,
    pub integration_parameters: IntegrationParameters,
}

impl PhysicsWorld {
    // See: https://rapier.rs/docs/user_guides/rust/getting_started for physics world creation
    pub fn new() -> Self {
        let mut integration_parameters = IntegrationParameters::default();
        integration_parameters.dt = crate::config::PHYSICS_TIMESTEP; // 50Hz simulation
        
        Self {
            rigid_body_set: RigidBodySet::new(),
            collider_set: ColliderSet::new(),
            physics_pipeline: PhysicsPipeline::new(),
            island_manager: IslandManager::new(),
            broad_phase: DefaultBroadPhase::new(),
            narrow_phase: NarrowPhase::new(),
            impulse_joint_set: ImpulseJointSet::new(),
            multibody_joint_set: MultibodyJointSet::new(),
            ccd_solver: CCDSolver::new(),
            query_pipeline: QueryPipeline::new(),
            
            entity_to_body: HashMap::new(),
            body_to_entity: HashMap::new(),
            
            gravity: Vector2::new(0.0, crate::config::GRAVITY), // Standard gravity
            integration_parameters,
        }
    }

    // Create a dynamic rigid body for a player entity
    // See: https://docs.rs/rapier2d/latest/rapier2d/dynamics/struct.RigidBodyBuilder.html
    pub fn create_player_body(&mut self, entity_id: u32, position: Vector2<f32>, mass: f32) -> RigidBodyHandle {
        let rigid_body = RigidBodyBuilder::dynamic()
            .translation(position)
            .build();
        
        let body_handle = self.rigid_body_set.insert(rigid_body);
        
        // Create a capsule collider for the player (typical for platformer characters)
        // See: https://docs.rs/rapier2d/latest/rapier2d/geometry/struct.ColliderBuilder.html
        let collider = ColliderBuilder::capsule_y(0.5, 0.3) // height=1.0, radius=0.3
            .density(mass / 1.0) // Adjust density to achieve desired mass
            .friction(0.5)
            .restitution(0.0) // No bouncing for platformer feel
            .build();
            
        self.collider_set.insert_with_parent(collider, body_handle, &mut self.rigid_body_set);
        
        // Store mapping
        self.entity_to_body.insert(entity_id, body_handle);
        self.body_to_entity.insert(body_handle, entity_id);
        
        body_handle
    }

    // Create a static ground/platform collider
    // See: https://docs.rs/rapier2d/latest/rapier2d/dynamics/struct.RigidBodyBuilder.html
    pub fn create_static_ground(&mut self, position: Vector2<f32>, size: Vector2<f32>) {
        let rigid_body = RigidBodyBuilder::fixed()
            .translation(position)
            .build();
            
        let body_handle = self.rigid_body_set.insert(rigid_body);
        
        let collider = ColliderBuilder::cuboid(size.x / 2.0, size.y / 2.0)
            .friction(0.7)
            .build();
            
        self.collider_set.insert_with_parent(collider, body_handle, &mut self.rigid_body_set);
    }

    // Apply horizontal movement force to a player
    // See: https://docs.rs/rapier2d/latest/rapier2d/dynamics/struct.RigidBody.html#method.apply_impulse
    pub fn apply_movement_force(&mut self, entity_id: u32, horizontal_input: f32, move_speed: f32) {
        if let Some(&body_handle) = self.entity_to_body.get(&entity_id) {
            if let Some(body) = self.rigid_body_set.get_mut(body_handle) {
                // Apply horizontal force based on input
                let force = Vector2::new(horizontal_input * move_speed * body.mass(), 0.0);
                body.apply_impulse(force, true);
            }
        }
    }

    // Apply jump impulse if on ground
    // See: https://docs.rs/rapier2d/latest/rapier2d/dynamics/struct.RigidBody.html#method.apply_impulse
    pub fn apply_jump_force(&mut self, entity_id: u32, jump_force: f32) -> bool {
        if let Some(&body_handle) = self.entity_to_body.get(&entity_id) {
            // Check ground state before getting mutable reference
            let is_grounded = self.is_on_ground(body_handle);
            
            if is_grounded {
                if let Some(body) = self.rigid_body_set.get_mut(body_handle) {
                    let impulse = Vector2::new(0.0, jump_force * body.mass());
                    body.apply_impulse(impulse, true);
                    return true;
                }
            }
        }
        false
    }

    // Simplified ground detection using velocity
    // TODO: Implement proper raycasting for more accurate ground detection
    fn is_on_ground(&self, body_handle: RigidBodyHandle) -> bool {
        if let Some(body) = self.rigid_body_set.get(body_handle) {
            // Simple check: if vertical velocity is near zero or negative, assume on ground
            body.linvel().y <= 0.1 && body.linvel().y >= -0.1
        } else {
            false
        }
    }

    // Step the physics simulation
    // See: https://docs.rs/rapier2d/latest/rapier2d/pipeline/struct.PhysicsPipeline.html#method.step
    pub fn step(&mut self) {
        self.physics_pipeline.step(
            &self.gravity,
            &self.integration_parameters,
            &mut self.island_manager,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &mut self.rigid_body_set,
            &mut self.collider_set,
            &mut self.impulse_joint_set,
            &mut self.multibody_joint_set,
            &mut self.ccd_solver,
            Some(&mut self.query_pipeline),
            &(),
            &(),
        );
    }

    // Get updated position and velocity for database sync
    pub fn get_body_state(&self, entity_id: u32) -> Option<(Vector2<f32>, Vector2<f32>)> {
        if let Some(&body_handle) = self.entity_to_body.get(&entity_id) {
            if let Some(body) = self.rigid_body_set.get(body_handle) {
                return Some((*body.translation(), *body.linvel()));
            }
        }
        None
    }
}