use spacetimedb::StdbRng;

// Extensions for SpacetimeDB's built-in random number generator
pub trait RandomExtensions {
    fn range_f32(&mut self, min: f32, max: f32) -> f32;
    fn range_u32(&mut self, min: u32, max: u32) -> u32;
    fn range_u64(&mut self, min: u64, max: u64) -> u64;
}

impl RandomExtensions for StdbRng {
    fn range_f32(&mut self, min: f32, max: f32) -> f32 {
        use spacetimedb::rand::Rng;
        self.gen::<f32>() * (max - min) + min
    }

    fn range_u32(&mut self, min: u32, max: u32) -> u32 {
        use spacetimedb::rand::Rng;
        self.gen_range(min..max)
    }
    
    fn range_u64(&mut self, min: u64, max: u64) -> u64 {
        use spacetimedb::rand::Rng;
        self.gen_range(min..max)
    }
}