use spacetimedb::{table, SpacetimeType};

// See: https://docs.rs/rapier2d/latest/rapier2d/dynamics/enum.RigidBodyType.html for Rapier2D body types
#[derive(SpacetimeType, Clone, Copy, Debug, PartialEq)]
pub enum BodyType {
    Dynamic,    // Affected by forces and gravity (players)
    Static,     // Immovable (ground, walls) 
    KinematicPositionBased, // Moved by setting position (platforms)
    KinematicVelocityBased, // Moved by setting velocity
}

// See: https://docs.rs/spacetimedb/latest/spacetimedb/attr.table.html for table definitions
#[table(name = physics_body)]
pub struct PhysicsBody {
    #[primary_key]
    pub entity_id: u32,
    pub body_type: BodyType,        // Type of physics body in Rapier2D
    pub on_ground: bool,            // Ground detection for jumping mechanics
    pub collision_groups: u16,      // Rapier2D collision filtering bitmask
}