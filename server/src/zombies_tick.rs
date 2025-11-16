use crate::constants::WORLD_ID;
use serde::{Deserialize, Serialize};
use spacetime_engine::{
    behavior::{tick_behavior, Action, BehaviorExecutor, BehaviorTree, Select, Sequence, Status},
    navigation::{NavigationAgent, NavigationState},
    utils::{Entity, WorldEntity},
    world::World,
};
use spacetimedb::{
    rand::seq::SliceRandom, reducer, table, ReducerContext, ScheduleAt, Table, TimeDuration,
};

use crate::tables::{player::Player, zombie::Zombie};

#[derive(Serialize, Deserialize)]
pub enum ZombieAction {
    IsMoving,
    TargetRandomPlayer,
    Chase,
    IsTargetReached,
    Attack,
}

impl BehaviorExecutor<ZombieAction> for Zombie {
    fn run_action(
        &mut self,
        ctx: &ReducerContext,
        _world: &World,
        _dt: f32,
        action: &ZombieAction,
    ) -> Status {
        let mut agent = NavigationAgent::find(ctx, self.navigation_agent_id)
            .expect("NavigationAgent not found");
        match action {
            ZombieAction::IsMoving => match agent.state == NavigationState::Moving {
                true => Status::Success,
                false => Status::Failure,
            },
            ZombieAction::IsTargetReached => match agent.state == NavigationState::ReachedTarget {
                true => Status::Success,
                false => Status::Failure,
            },
            ZombieAction::TargetRandomPlayer => {
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
            ZombieAction::Chase => {
                if let Some(player) = Player::find(ctx, self.target_player.unwrap()) {
                    agent.current_target = Some(player.position);
                    agent.update(ctx);
                    return Status::Success;
                }

                // Target player not found, fail the action to re-target
                Status::Failure
            }
            ZombieAction::Attack => {
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

                if player.position.distance(&agent.position) > 1.0 {
                    self.is_attacking = false;
                    self.clone().update(ctx);
                    return Status::Failure;
                }

                self.is_attacking = true;
                self.next_attack_time = ctx.timestamp + TimeDuration::from_micros(1_000_000);
                self.clone().update(ctx);

                Status::Success
            }
        }
    }
}

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

pub fn create_zombie_behavior_tree(ctx: &ReducerContext) {
    let bt = Select(vec![
        // If we are moving, chase the target (i.e update target position)
        Sequence(vec![
            Action(ZombieAction::IsMoving),
            Action(ZombieAction::Chase),
        ]),
        // If we reached the target, attack
        Sequence(vec![
            Action(ZombieAction::IsTargetReached),
            Action(ZombieAction::Attack),
        ]),
        // Otherwise, target a random player
        Action(ZombieAction::TargetRandomPlayer),
    ]);
    BehaviorTree::create(ctx, bt);
}

#[reducer]
pub fn tick_zombie(ctx: &ReducerContext, tick: ZombieUpdateTick) {
    let delta_time = match tick.scheduled_at {
        ScheduleAt::Interval(dt) => dt.to_micros() as f32 / 1_000_000.0,
        _ => 0.0,
    };
    let world = World::find(ctx, WORLD_ID).expect("World not found");
    let mut zombies = Zombie::as_vec(ctx);
    tick_behavior(ctx, &world, 1, delta_time, &mut zombies);
}
