use spacetimedb::{reducer, table, ReducerContext, ScheduleAt, Table, TimeDuration};

use crate::tables::{player::player, zombie::zombie};

#[table(name = zombie_update_tick, scheduled(tick_zombie))]
pub struct ZombieUpdateTick {
    #[primary_key]
    #[auto_inc]
    pub id: u64,
    pub scheduled_at: ScheduleAt,
}

impl ZombieUpdateTick {
    pub fn schedule(ctx: &ReducerContext) -> Self {
        ctx.db.zombie_update_tick().insert(ZombieUpdateTick {
            id: 0,
            scheduled_at: ScheduleAt::Interval(TimeDuration::from_micros(250_000)),
        })
    }
}

#[reducer]
pub fn tick_zombie(ctx: &ReducerContext, _t: ZombieUpdateTick) {
    // TODO: Proper target selection logic instead of just picking the first player
    let player = match ctx.db.player().iter().next() {
        Some(p) => p,
        None => return,
    };

    for zombie in ctx.db.zombie().iter() {
        zombie.update_target(ctx, Some(player.position));
    }
}
