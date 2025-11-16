use spacetime_engine::{math::Vec3, navigation::Character, utils::Entity};
use spacetimedb::{reducer, table, ReducerContext, ScheduleAt, Table};

use crate::{
    constants::WORLD_ID,
    spitter_zombie::SpitterZombie,
    tables::{player::Player, zombie::Zombie},
};

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
    let characters = Player::iter(ctx).map(|p| Character {
        position: p.position,
        velocity: Vec3::ZERO,
        radius: 0.5,
    });

    let agents = spacetime_engine::world::tick_world(ctx, WORLD_ID, tick.scheduled_at, characters);

    for mut zombie in Zombie::iter(ctx) {
        if let Some(agent) = agents.get(&zombie.navigation_agent_id) {
            zombie.position = agent.position();
            zombie.update(ctx);
        }
    }

    for mut spitter_zombie in SpitterZombie::iter(ctx) {
        if let Some(agent) = agents.get(&spitter_zombie.navigation_agent_id) {
            spitter_zombie.position = agent.position();
            spitter_zombie.update(ctx);
        }
    }
}
