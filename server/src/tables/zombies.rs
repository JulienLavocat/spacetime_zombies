use spacetimedb::{table, ScheduleAt};

use crate::zombies_update;

#[table(name = zombie)]
#[derive(Clone)]
pub struct Zombie {
    #[primary_key]
    #[auto_inc]
    pub id: u64,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[table(name = zombie_tick, scheduled(zombies_update))]
pub struct ZombieTick {
    #[primary_key]
    #[auto_inc]
    pub id: u64,
    pub scheduled_at: ScheduleAt,
}
