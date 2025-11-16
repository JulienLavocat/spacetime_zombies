use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use spacetime_engine::{
    behavior::{tick_behavior, Action, BehaviorExecutor, BehaviorTree, Select, Sequence, Status},
    collisions::{Trigger, TriggerId},
    math::Vec3,
    navigation::{NavigationAgent, NavigationState, TargetReachedCondition},
    utils::{get_delta_time, Entity, WorldEntity},
    world::World,
};
use spacetimedb::{
    rand::seq::SliceRandom, reducer, table, ReducerContext, ScheduleAt, Table, TimeDuration,
    Timestamp,
};

use crate::{
    constants::{SPITTER_AOE_COLLIDER_ID, WORLD_ID},
    tables::player::Player,
};

const SPLITTER_ZOMBIE_COOLDOWN_MICROS: i64 = 3_000_000;
const SPLITTER_ZOMBIE_ATTACK_RANGE: f32 = 10.0;

pub type SpitterZombieId = u64;

#[table(name = spitter_zombie, public)]
#[derive(Clone)]
pub struct SpitterZombie {
    #[primary_key]
    #[auto_inc]
    pub id: SpitterZombieId,
    #[index(btree)]
    pub navigation_agent_id: u64,
    pub position: Vec3,
    pub target_player: Option<u64>,
    pub is_attacking: bool,
    pub next_attack_time: Timestamp,
}

impl Entity for SpitterZombie {
    fn insert(self, ctx: &ReducerContext) -> Self {
        ctx.db.spitter_zombie().insert(self)
    }

    fn find(ctx: &ReducerContext, id: u64) -> Option<Self> {
        ctx.db.spitter_zombie().id().find(id)
    }

    fn iter(ctx: &ReducerContext) -> impl Iterator<Item = Self> {
        ctx.db.spitter_zombie().iter()
    }

    fn as_map(ctx: &ReducerContext) -> HashMap<u64, Self> {
        ctx.db
            .spitter_zombie()
            .iter()
            .map(|zombie| (zombie.id, zombie))
            .collect()
    }

    fn as_vec(ctx: &ReducerContext) -> Vec<Self> {
        ctx.db.spitter_zombie().iter().collect()
    }

    fn update(self, ctx: &ReducerContext) -> Self {
        ctx.db.spitter_zombie().id().update(self)
    }

    fn delete(&self, ctx: &ReducerContext) {
        NavigationAgent::find(ctx, self.navigation_agent_id)
            .unwrap()
            .delete(ctx);
        ctx.db.spitter_zombie().id().delete(self.id);
    }

    fn clear(ctx: &ReducerContext) {
        for zombie in ctx.db.spitter_zombie().iter() {
            zombie.delete(ctx);
        }
    }

    fn count(ctx: &ReducerContext) -> u64 {
        ctx.db.spitter_zombie().count()
    }
}

impl SpitterZombie {
    pub fn create(ctx: &ReducerContext, position: Vec3) -> Self {
        let navigation_agent_id = NavigationAgent::builder()
            .desired_speed(3.0)
            .max_speed(5.0)
            .radius(0.3)
            .target_reached_condition(TargetReachedCondition::Distance(Some(
                SPLITTER_ZOMBIE_ATTACK_RANGE,
            )))
            .position(position)
            .build()
            .insert(ctx)
            .id;
        ctx.db.spitter_zombie().insert(SpitterZombie {
            id: 0,
            navigation_agent_id,
            position,
            target_player: None,
            is_attacking: false,
            next_attack_time: Timestamp::from_micros_since_unix_epoch(0),
        })
    }
}

#[table(name = spitter_aoe, public, scheduled(tick_spitter_aoe))]
pub struct SpitterAoE {
    #[primary_key]
    #[auto_inc]
    pub id: u64,
    pub trigger_id: TriggerId,
    pub position: Vec3,
    pub delete_at: Timestamp,
    pub scheduled_at: ScheduleAt,
}

impl Entity for SpitterAoE {
    fn insert(self, ctx: &ReducerContext) -> Self {
        ctx.db.spitter_aoe().insert(self)
    }

    fn find(ctx: &ReducerContext, id: u64) -> Option<Self> {
        ctx.db.spitter_aoe().id().find(id)
    }

    fn iter(ctx: &ReducerContext) -> impl Iterator<Item = Self> {
        ctx.db.spitter_aoe().iter()
    }

    fn as_map(ctx: &ReducerContext) -> HashMap<u64, Self> {
        ctx.db
            .spitter_aoe()
            .iter()
            .map(|aoe| (aoe.id, aoe))
            .collect()
    }

    fn as_vec(ctx: &ReducerContext) -> Vec<Self> {
        ctx.db.spitter_aoe().iter().collect()
    }

    fn update(self, ctx: &ReducerContext) -> Self {
        ctx.db.spitter_aoe().id().update(self)
    }

    fn delete(&self, ctx: &ReducerContext) {
        Trigger::find(ctx, self.trigger_id).unwrap().delete(ctx);
        ctx.db.spitter_aoe().id().delete(self.id);
    }

    fn clear(ctx: &ReducerContext) {
        for aoe in ctx.db.spitter_aoe().iter() {
            aoe.delete(ctx);
        }
    }

    fn count(ctx: &ReducerContext) -> u64 {
        ctx.db.spitter_aoe().count()
    }
}

#[table(name = spitter_zombie_update_tick, scheduled(tick_spitter_zombie))]
pub struct SpitterZombieUpdateTick {
    #[primary_key]
    #[auto_inc]
    pub id: u64,
    pub scheduled_at: ScheduleAt,
}

impl SpitterZombieUpdateTick {
    pub fn schedule(ctx: &ReducerContext) -> Self {
        ctx.db
            .spitter_zombie_update_tick()
            .insert(SpitterZombieUpdateTick {
                id: 0,
                scheduled_at: ScheduleAt::Interval(TimeDuration::from_micros(250_000)),
            })
    }
}

#[reducer]
fn tick_spitter_zombie(ctx: &ReducerContext, tick: SpitterZombieUpdateTick) {
    let delta_time = get_delta_time(tick.scheduled_at);
    let world = World::find(ctx, WORLD_ID).expect("World not found");
    let mut zombies = SpitterZombie::as_vec(ctx);
    tick_behavior(ctx, &world, 1, delta_time, &mut zombies);
}

#[derive(Serialize, Deserialize)]
enum SpitterZombieAction {
    IsMoving,
    TargetRandomPlayer,
    Chase,
    IsTargetReached,
    Attack,
}

impl BehaviorExecutor<SpitterZombieAction> for SpitterZombie {
    fn run_action(
        &mut self,
        ctx: &ReducerContext,
        _world: &World,
        _dt: f32,
        action: &SpitterZombieAction,
    ) -> Status {
        let mut agent = NavigationAgent::find(ctx, self.navigation_agent_id)
            .expect("NavigationAgent not found");
        match action {
            SpitterZombieAction::IsMoving => match agent.state == NavigationState::Moving {
                true => Status::Success,
                false => Status::Failure,
            },
            SpitterZombieAction::IsTargetReached => {
                match agent.state == NavigationState::ReachedTarget {
                    true => Status::Success,
                    false => Status::Failure,
                }
            }
            SpitterZombieAction::TargetRandomPlayer => {
                let players = Player::as_vec(ctx);
                if players.is_empty() {
                    return Status::Failure;
                }

                let target = players.choose(&mut ctx.rng()).unwrap();

                agent.paused = false;
                agent.current_target = Some(target.position);
                agent.update(ctx);

                self.target_player = Some(target.id);
                self.is_attacking = false;
                self.clone().update(ctx);
                Status::Success
            }
            SpitterZombieAction::Chase => {
                if let Some(player) = Player::find(ctx, self.target_player.unwrap()) {
                    agent.current_target = Some(player.position);
                    agent.update(ctx);
                    return Status::Success;
                }

                // Target player not found, fail the action to re-target
                Status::Failure
            }
            SpitterZombieAction::Attack => {
                let player = Player::find(ctx, self.target_player.unwrap());

                if player.is_none() {
                    return Status::Failure;
                }

                if ctx.timestamp <= self.next_attack_time {
                    self.is_attacking = false;
                    self.clone().update(ctx);
                    return Status::Success;
                }

                let player = player.unwrap();

                if player.position.distance(&agent.position) > SPLITTER_ZOMBIE_ATTACK_RANGE {
                    self.is_attacking = false;
                    self.clone().update(ctx);
                    return Status::Failure;
                }

                self.is_attacking = true;
                self.next_attack_time =
                    ctx.timestamp + TimeDuration::from_micros(SPLITTER_ZOMBIE_COOLDOWN_MICROS);
                self.clone().update(ctx);

                let trigger_id = Trigger::builder()
                    .position(player.position)
                    .collider_id(SPITTER_AOE_COLLIDER_ID)
                    .build()
                    .insert(ctx)
                    .id;
                SpitterAoE {
                    id: 0,
                    trigger_id,
                    position: player.position,
                    delete_at: ctx.timestamp
                        + TimeDuration::from_micros(SPLITTER_ZOMBIE_COOLDOWN_MICROS),
                    scheduled_at: ScheduleAt::Interval(TimeDuration::from_micros(500_000)),
                }
                .insert(ctx);

                Status::Success
            }
        }
    }
}

pub fn create_spitter_zombie_behavior_tree(ctx: &ReducerContext) {
    let bt = Select(vec![
        // If we are moving, chase the target (i.e update target position)
        Sequence(vec![
            Action(SpitterZombieAction::IsMoving),
            Action(SpitterZombieAction::Chase),
        ]),
        // If we reached the target, attack
        Sequence(vec![
            Action(SpitterZombieAction::IsTargetReached),
            Action(SpitterZombieAction::Attack),
        ]),
        // Otherwise, target a random player
        Action(SpitterZombieAction::TargetRandomPlayer),
    ]);
    BehaviorTree::create(ctx, bt);
}

#[reducer]
fn tick_spitter_aoe(ctx: &ReducerContext, tick: SpitterAoE) {
    let trigger = Trigger::find(ctx, tick.trigger_id).unwrap();

    if ctx.timestamp >= tick.delete_at {
        tick.delete(ctx);
        return;
    }

    log::debug!("SpitterAoE#{} trigger={:?}", tick.id, trigger);
}
