use log::info;
use spacetime_engine::{
    collisions::Collider,
    math::Vec3,
    navigation::{import_external_navmesh, ExternalNavMesh},
    utils::{Entity, WorldEntity},
    world::World,
};
use spacetimedb::{reducer, ReducerContext};

use crate::{
    constants::WORLD_ID,
    spitter_zombie::{create_spitter_zombie_behavior_tree, SpitterZombieUpdateTick},
    world::WorldTick,
    zombies_spawner::{ZombieSpawnPoint, ZombieSpawnTick},
    zombies_tick::{create_zombie_behavior_tree, ZombieUpdateTick},
};

#[reducer(init)]
pub fn init(ctx: &ReducerContext) {
    World::builder().debug_collisions(true).build().insert(ctx);

    create_zombie_behavior_tree(ctx);
    create_spitter_zombie_behavior_tree(ctx);

    ZombieUpdateTick::schedule(ctx);
    SpitterZombieUpdateTick::schedule(ctx);
    ZombieSpawnTick::schedule(ctx);
    WorldTick::schedule(ctx);

    ZombieSpawnPoint::create(ctx, Vec3::new(0.0, 0.0, -50.0));
    ZombieSpawnPoint::create(ctx, Vec3::new(-16.0, 0.0, 0.0));
    ZombieSpawnPoint::create(ctx, Vec3::new(44.0, 0.0, 2.0));
    ZombieSpawnPoint::create(ctx, Vec3::new(0.0, 0.0, 40.0));

    // Player collider
    Collider::capsule(WORLD_ID, 0.5, 1.8).insert(ctx);
    // Spitter AoE collider
    Collider::sphere(WORLD_ID, 3.0).insert(ctx);
}

#[reducer]
fn editor_upload_navmesh(ctx: &ReducerContext, world_id: u64) {
    let file = include_str!("../../client/navmesh_export.json");
    info!("navmesh export file size: {} bytes", file.len());
    let navmesh =
        serde_json::from_str::<ExternalNavMesh>(file).expect("Failed to parse navmesh JSON");
    info!(
        "Parsed navmesh with {} vertices and {} polygons",
        navmesh.vertices.len(),
        navmesh.polygons.len()
    );

    info!(
        "Uploading navmesh with {} vertices and {} polygons for world {}",
        navmesh.vertices.len(),
        navmesh.polygons.len(),
        world_id
    );

    import_external_navmesh(ctx, world_id, navmesh);
}

#[reducer]
fn generate_external_navmesh(_ctx: &ReducerContext, _en: ExternalNavMesh) {
    // This reducer does nothing, it's just a placeholder to generate
    // the necessary serialization code for ExternalNavMesh.
}
