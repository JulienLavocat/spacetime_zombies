use spacetime_engine::{
    collisions::Trigger,
    utils::{Entity, WorldEntity},
};
use spacetimedb::{reducer, ReducerContext, Table};

use crate::{
    constants::WORLD_ID,
    spitter_zombie::{SpitterAoE, SpitterZombie},
    tables::{
        player::{player, Player},
        zombie::Zombie,
    },
};

#[reducer(client_disconnected)]
fn on_disconnect(ctx: &ReducerContext) {
    let player = Player::find_by_identity(ctx, ctx.sender);
    if let Some(player) = player {
        player.delete(ctx);
    }

    if ctx.db.player().count() == 0 {
        Zombie::clear(ctx);
        SpitterZombie::clear(ctx);
        SpitterAoE::clear(ctx);
        Trigger::clear(ctx, WORLD_ID);
    }
}
