// JustMaple SpacetimeDB Rust Server
// See: https://docs.rs/spacetimedb/latest/spacetimedb/ for SpacetimeDB Rust SDK

pub mod tables;
pub mod reducers;
pub mod utils;
pub mod config;
pub mod types;
pub mod physics; // NEW: Physics simulation module

// Re-export all modules for external access
pub use tables::*;
pub use reducers::*;
pub use utils::*;
pub use config::*;
pub use types::*;
pub use physics::*;
