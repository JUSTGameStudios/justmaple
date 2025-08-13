use spacetimedb::{reducer, ReducerContext, Table};
use crate::tables::Config;
use crate::config::*;
use crate::reducers::physics_reducers::init_physics;
use crate::tables::config::config;

#[reducer(init)]
pub fn init(ctx: &ReducerContext) -> Result<(), Box<dyn std::error::Error>> {
    log::info!("Initializing game world...");
    
    ctx.db.config().insert(Config {
        id: 0,
        world_size: DEFAULT_WORLD_SIZE,
    });
    init_physics(ctx)?;

    Ok(())
}

