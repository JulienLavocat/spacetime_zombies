use spacetime_engine::utils::Entity;
use spacetimedb::{reducer, table, ReducerContext, ScheduleAt, Table};

use crate::tables::zombie::zombie;

// We only have one world
pub const WORLD_ID: u64 = 1;

#[table(name = world_tick, scheduled(tick_world))]
pub struct WorldTick {
    #[primary_key]
    #[auto_inc]
    pub id: u64,
    pub scheduled_at: ScheduleAt,
}

impl WorldTick {
    pub fn schedule(ctx: &ReducerContext) -> Self {
        ctx.db.world_tick().insert(WorldTick {
            id: 0,
            scheduled_at: ScheduleAt::Interval(spacetimedb::TimeDuration::from_micros(100_000)),
        })
    }
}

#[reducer]
pub fn tick_world(ctx: &ReducerContext, tick: WorldTick) {
    spacetime_engine::world::tick_world(
        ctx,
        WORLD_ID,
        tick.scheduled_at,
        |ctx, _world_id, _dt, agents| {
            for mut zombie in ctx.db.zombie().iter() {
                if let Some(agent) = agents.get(&zombie.navigation_agent_id) {
                    zombie.position = agent.position;
                    zombie.update(ctx);
                }
            }
        },
    );
}
