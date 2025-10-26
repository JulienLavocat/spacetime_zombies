use std::sync::Arc;

use landmass::{
    Agent, AgentId, Archipelago, ArchipelagoOptions, FromAgentRadius, Island, NavigationMesh,
    TargetReachedCondition, Transform, Vec3, XYZ,
};
use spacetimedb::{reducer, Identity, ReducerContext, ScheduleAt, Table, TimeDuration};

use crate::tables::{
    nav_mesh, player, zombie, zombie_tick, NavMesh, Player, StVec3, Zombie, ZombieTick,
};
use log::debug;

mod tables;
mod types;

#[reducer(init)]
pub fn init(ctx: &ReducerContext) {
    ctx.db.nav_mesh().insert(NavMesh {
        id: 0,
        vertices: vec![
            StVec3::new(0.0, 0.0, 0.0),
            StVec3::new(200.0, 0.0, 0.0),
            StVec3::new(200.0, 200.0, 0.0),
            StVec3::new(0.0, 200.0, 0.0),
        ],
        polygons: vec![vec![0, 1, 2, 3]],
        polygon_type_indices: vec![0],
    });

    ctx.db.zombie().insert(Zombie {
        id: 0,
        x: 100.0,
        y: 0.0,
        z: 0.0,
    });

    ctx.db.player().insert(Player {
        id: 0,
        identity: Identity::ZERO,
        name: "Player1".to_string(),
        x: 0.0,
        y: 0.0,
        z: 0.0,
    });

    ctx.db.zombie_tick().insert(ZombieTick {
        id: 0,
        scheduled_at: TimeDuration::from_micros(100_000).into(),
    });
}

#[reducer]
pub fn zombies_update(ctx: &ReducerContext, tick: ZombieTick) {}
