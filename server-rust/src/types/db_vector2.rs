use spacetimedb::SpacetimeType;
use std::ops::{Add, Sub, Mul, Div};
// See: https://docs.rs/nalgebra/latest/nalgebra/ for nalgebra Vector2
// Using Rapier's nalgebra re-export to avoid version conflicts
use rapier2d::na::Vector2;

// See: https://docs.rs/spacetimedb/latest/spacetimedb/derive.SpacetimeType.html for custom types
#[derive(SpacetimeType, Clone, Copy, Debug, PartialEq)]
pub struct DbVector2 {
    pub x: f32,
    pub y: f32,
}

impl DbVector2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn sqr_magnitude(&self) -> f32 {
        self.x * self.x + self.y * self.y
    }

    pub fn magnitude(&self) -> f32 {
        self.sqr_magnitude().sqrt()
    }

    pub fn normalized(&self) -> Self {
        let mag = self.magnitude();
        if mag > 0.0 {
            *self / mag
        } else {
            Self::new(0.0, 0.0)
        }
    }

    pub fn zero() -> Self {
        Self::new(0.0, 0.0)
    }

    // Conversion to/from nalgebra Vector2 for Rapier2D integration
    // See: https://docs.rs/nalgebra/latest/nalgebra/base/struct.Vector2.html
    pub fn to_nalgebra(&self) -> Vector2<f32> {
        Vector2::new(self.x, self.y)
    }

    pub fn from_nalgebra(vec: Vector2<f32>) -> Self {
        Self::new(vec.x, vec.y)
    }
}

impl Add for DbVector2 {
    type Output = Self;
    
    fn add(self, other: Self) -> Self::Output {
        Self::new(self.x + other.x, self.y + other.y)
    }
}

impl Sub for DbVector2 {
    type Output = Self;
    
    fn sub(self, other: Self) -> Self::Output {
        Self::new(self.x - other.x, self.y - other.y)
    }
}

impl Mul<f32> for DbVector2 {
    type Output = Self;
    
    fn mul(self, scalar: f32) -> Self::Output {
        Self::new(self.x * scalar, self.y * scalar)
    }
}

impl Div<f32> for DbVector2 {
    type Output = Self;
    
    fn div(self, scalar: f32) -> Self::Output {
        Self::new(self.x / scalar, self.y / scalar)
    }
}