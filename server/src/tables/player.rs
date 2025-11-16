use std::collections::HashMap;

use spacetime_engine::{
    collisions::RigidBody,
    math::Vec3,
    utils::{Entity, WorldEntity},
};
use spacetimedb::{table, Identity, ReducerContext, Table, Timestamp};

use crate::constants::{PLAYER_COLLIDER_ID, WORLD_ID};

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
    pub rigid_body_id: u64,
}

impl Entity for Player {
    fn insert(self, ctx: &ReducerContext) -> Self {
        ctx.db.player().insert(self)
    }

    fn find(ctx: &ReducerContext, id: u64) -> Option<Self> {
        ctx.db.player().id().find(id)
    }

    fn iter(ctx: &ReducerContext) -> impl Iterator<Item = Self> {
        ctx.db.player().iter()
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
        if let Some(rb) = RigidBody::find(ctx, self.rigid_body_id) {
            rb.delete(ctx);
        }

        ctx.db.player().id().delete(self.id);
    }

    fn clear(ctx: &ReducerContext) {
        for player in ctx.db.player().iter() {
            player.delete(ctx);
        }
    }

    fn count(ctx: &ReducerContext) -> u64 {
        ctx.db.player().count()
    }
}

impl Player {
    pub fn create(ctx: &ReducerContext) -> Self {
        let rb = RigidBody::builder()
            .world_id(WORLD_ID)
            .collider_id(PLAYER_COLLIDER_ID)
            .build()
            .insert(ctx);
        Player {
            id: 0,
            name: format!("Player-{}", ctx.sender.to_abbreviated_hex()),
            joined_at: ctx.timestamp,
            identity: ctx.sender,
            position: Vec3::new(0.0, 0.0, 0.0),
            rigid_body_id: rb.id,
        }
        .insert(ctx)
    }

    pub fn find_by_identity(ctx: &ReducerContext, identity: Identity) -> Option<Self> {
        ctx.db.player().identity().find(identity)
    }
}
