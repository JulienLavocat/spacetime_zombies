use spacetime_engine::{math::Vec3, utils::Entity};
use spacetimedb::{reducer, ReducerContext};

use crate::tables::player::{player, Player};

#[reducer]
fn player_ready(ctx: &ReducerContext) {
    Player {
        id: 0,
        name: format!("Player-{}", ctx.sender.to_abbreviated_hex()),
        joined_at: ctx.timestamp,
        identity: ctx.sender,
        position: Vec3::new(0.0, 0.0, 0.0),
    }
    .insert(ctx);
}

#[reducer]
fn player_update_position(ctx: &ReducerContext, new_position: Vec3) {
    let mut player = ctx.db.player().identity().find(ctx.sender).unwrap();
    player.position = new_position;
    player.update(ctx);
}
