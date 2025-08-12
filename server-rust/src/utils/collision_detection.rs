use crate::tables::Entity;
use crate::utils::GameMath;

pub struct CollisionDetection;

impl CollisionDetection {
    pub fn is_overlapping(a: &Entity, b: &Entity) -> bool {
        let dx = a.position.x - b.position.x;
        let dy = a.position.y - b.position.y;
        let distance_sq = dx * dx + dy * dy;

        let radius_a = GameMath::mass_to_radius(a.mass);
        let radius_b = GameMath::mass_to_radius(b.mass);

        // If the distance between the two circle centers is less than the
        // maximum radius, then the center of the smaller circle is inside
        // the larger circle. This gives some leeway for the circles to overlap
        // before being eaten.
        let max_radius = if radius_a > radius_b { radius_a } else { radius_b };
        distance_sq <= max_radius * max_radius
    }
}