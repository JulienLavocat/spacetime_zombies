use spacetime_engine::utils::Entity;
use spacetimedb::{reducer, ReducerContext, Table};

use crate::tables::{player::player, zombie::Zombie};

mod init;
mod players;
mod tables;
mod types;
mod world;
mod zombies_spawner;
mod zombies_tick;

#[reducer(client_disconnected)]
fn on_disconnect(ctx: &ReducerContext) {
    ctx.db.player().identity().delete(ctx.sender);

    if ctx.db.player().count() == 0 {
        Zombie::clear(ctx);
    }
}
