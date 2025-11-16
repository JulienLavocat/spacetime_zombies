use std::collections::HashMap;

use spacetime_engine::{
    math::Vec3,
    navigation::{DestinationReachedCondition, NavigationAgent, NavigationAgentId},
    utils::{Entity, WorldEntity},
};
use spacetimedb::{table, ReducerContext, Table, Timestamp};

pub type ZombieId = u64;

#[table(name = zombie, public)]
#[derive(Clone)]
pub struct Zombie {
    #[primary_key]
    #[auto_inc]
    pub id: ZombieId,
    #[index(btree)]
    pub navigation_agent_id: NavigationAgentId,
    pub position: Vec3,
    pub target_player: Option<u64>,
    pub is_attacking: bool,
    pub next_attack_time: Timestamp,
}

impl Entity for Zombie {
    fn insert(self, ctx: &ReducerContext) -> Self {
        ctx.db.zombie().insert(self)
    }

    fn find(ctx: &ReducerContext, id: u64) -> Option<Self> {
        ctx.db.zombie().id().find(id)
    }

    fn iter(ctx: &ReducerContext) -> impl Iterator<Item = Self> {
        ctx.db.zombie().iter()
    }

    fn as_map(ctx: &ReducerContext) -> HashMap<u64, Self> {
        ctx.db
            .zombie()
            .iter()
            .map(|zombie| (zombie.id, zombie))
            .collect()
    }

    fn as_vec(ctx: &ReducerContext) -> Vec<Self> {
        ctx.db.zombie().iter().collect()
    }

    fn update(self, ctx: &ReducerContext) -> Self {
        ctx.db.zombie().id().update(self)
    }

    fn delete(&self, ctx: &ReducerContext) {
        NavigationAgent::find(ctx, self.navigation_agent_id)
            .unwrap()
            .delete(ctx);
        ctx.db.zombie().id().delete(self.id);
    }

    fn clear(ctx: &ReducerContext) {
        for zombie in ctx.db.zombie().iter() {
            zombie.delete(ctx);
        }
    }

    fn count(ctx: &ReducerContext) -> u64 {
        ctx.db.zombie().count()
    }
}

impl Zombie {
    pub fn create(ctx: &ReducerContext, position: Vec3) -> Self {
        let navigation_agent_id = NavigationAgent::builder()
            .desired_speed(3.0)
            .max_speed(5.0)
            .radius(0.3)
            .target_reached_condition(DestinationReachedCondition::Distance(Some(1.5)))
            .position(position)
            .build()
            .insert(ctx)
            .id();

        Zombie {
            id: 0,
            navigation_agent_id,
            target_player: None,
            position,
            is_attacking: false,
            next_attack_time: ctx.timestamp, // We could make it an option but its fine like this
        }
        .insert(ctx)
    }
}
