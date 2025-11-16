use spacetime_engine::math::Vec3;
use spacetimedb::{reducer, table, ReducerContext, ScheduleAt, Table, TimeDuration};

use crate::{
    constants::SPITTER_SPAWN_CHANCE,
    spitter_zombie::SpitterZombie,
    tables::{
        player::player,
        zombie::{zombie, Zombie},
    },
};

#[table(name = zombie_spawn_tick, scheduled(spawn_zombies))]
pub struct ZombieSpawnTick {
    #[primary_key]
    #[auto_inc]
    pub id: u64,
    pub scheduled_at: ScheduleAt,
}

impl ZombieSpawnTick {
    pub fn schedule(ctx: &ReducerContext) -> Self {
        ctx.db.zombie_spawn_tick().insert(ZombieSpawnTick {
            id: 0,
            scheduled_at: ScheduleAt::Interval(TimeDuration::from_micros(500_000)),
        })
    }
}

#[table(name = zombie_spawn_point)]
pub struct ZombieSpawnPoint {
    #[primary_key]
    #[auto_inc]
    pub id: u64,
    pub position: Vec3,
}

impl ZombieSpawnPoint {
    pub fn create(ctx: &ReducerContext, position: Vec3) -> Self {
        ctx.db
            .zombie_spawn_point()
            .insert(ZombieSpawnPoint { id: 0, position })
    }
}

#[reducer]
pub fn spawn_zombies(ctx: &ReducerContext, _t: ZombieSpawnTick) {
    let zombie_count = ctx.db.zombie().count();
    let player_count = ctx.db.player().count();
    if zombie_count >= 100 || player_count == 0 {
        return;
    }

    // FIXME: Remove the collect here
    let spawn_points: Vec<ZombieSpawnPoint> = ctx.db.zombie_spawn_point().iter().collect();
    if spawn_points.is_empty() {
        return;
    }

    let spawn_point = &spawn_points[ctx.random::<usize>() % spawn_points.len()];

    if ctx.random::<f32>() <= SPITTER_SPAWN_CHANCE {
        SpitterZombie::create(ctx, spawn_point.position);
    } else {
        Zombie::create(ctx, spawn_point.position);
    }
}
