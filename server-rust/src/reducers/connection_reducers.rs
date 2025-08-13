use spacetimedb::{reducer, ReducerContext, Table};
use crate::tables::{Player, LoggedOutPlayer};
// Import table access traits
use crate::tables::player::{player, logged_out_player};
use crate::tables::movement_controller::movement_controller;
use crate::tables::entity::entity;

// See: https://docs.rs/spacetimedb/latest/spacetimedb/attr.reducer.html for reducer definitions

#[reducer(client_connected)]
pub fn connect(ctx: &ReducerContext) {
    // Check if player was previously logged out
    if let Some(player) = ctx.db.logged_out_player().identity().find(&ctx.sender) {
        // Move from logged_out_player to player table
        ctx.db.player().insert(Player {
            identity: player.identity,
            player_id: player.player_id,
            name: player.name,
        });
        ctx.db.logged_out_player().identity().delete(&player.identity);
    } else {
        // Create new player
        ctx.db.player().insert(Player {
            identity: ctx.sender,
            player_id: 0, // Auto-incremented
            name: String::new(),
        });
    }
}

#[reducer(client_disconnected)]
pub fn disconnect(ctx: &ReducerContext) {
    let player = ctx.db.player().identity().find(&ctx.sender)
        .expect("Player not found");

    // Remove any player entities from the arena
    for controller in ctx.db.movement_controller().player_id().filter(&player.player_id) {
        if let Some(entity) = ctx.db.entity().entity_id().find(&controller.entity_id) {
            ctx.db.entity().entity_id().delete(&entity.entity_id);
            ctx.db.movement_controller().entity_id().delete(&entity.entity_id);
        }
    }

    // Move player to logged_out_player table
    ctx.db.logged_out_player().insert(LoggedOutPlayer {
        identity: player.identity,
        player_id: player.player_id,
        name: player.name,
    });
    ctx.db.player().identity().delete(&player.identity);
}