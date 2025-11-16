use spacetime_engine::{
    collisions::RigidBody,
    math::Vec3,
    utils::{Entity, WorldEntity},
};
use spacetimedb::{reducer, ReducerContext};

use crate::tables::player::{player, Player};

#[reducer]
fn player_ready(ctx: &ReducerContext) {
    Player::create(ctx);
}

#[reducer]
fn player_update_position(ctx: &ReducerContext, new_position: Vec3) {
    let mut player = ctx.db.player().identity().find(ctx.sender).unwrap();
    let mut rb = RigidBody::find(ctx, player.rigid_body_id).unwrap();

    player.position = new_position;
    player.update(ctx);

    rb.position = new_position;
    rb.update(ctx);
}
