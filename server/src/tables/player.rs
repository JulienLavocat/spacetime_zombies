use std::collections::HashMap;

use spacetime_engine::{math::Vec3, utils::Entity};
use spacetimedb::{table, Identity, ReducerContext, Table, Timestamp};

#[table(name = player, public)]
pub struct Player {
    #[primary_key]
    #[auto_inc]
    pub id: u64,
    #[unique]
    pub identity: Identity,
    pub joined_at: Timestamp,
    pub name: String,
    pub position: Vec3,
}

impl Entity for Player {
    fn insert(self, ctx: &ReducerContext) -> Self {
        ctx.db.player().insert(self)
    }

    fn find(ctx: &ReducerContext, id: u64) -> Option<Self> {
        ctx.db.player().id().find(id)
    }

    fn as_map(ctx: &ReducerContext) -> HashMap<u64, Self> {
        ctx.db
            .player()
            .iter()
            .map(|player| (player.id, player))
            .collect()
    }

    fn as_vec(ctx: &ReducerContext) -> Vec<Self> {
        ctx.db.player().iter().collect()
    }

    fn update(self, ctx: &ReducerContext) -> Self {
        ctx.db.player().id().update(self)
    }

    fn delete(&self, ctx: &ReducerContext) {
        ctx.db.player().id().delete(self.id);
    }

    fn clear(ctx: &ReducerContext) {
        for player in ctx.db.player().iter() {
            player.delete(ctx);
        }
    }
}
