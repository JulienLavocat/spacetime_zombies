use log::info;
use spacetime_engine::{
    math::Vec3,
    navigation::{import_external_navmesh, ExternalNavMesh},
    utils::Entity,
    world::World,
};
use spacetimedb::{reducer, ReducerContext};

use crate::{
    world::WorldTick,
    zombies_spawner::{ZombieSpawnPoint, ZombieSpawnTick},
    zombies_tick::{create_zombie_behavior_tree, ZombieUpdateTick},
};

#[reducer(init)]
pub fn init(ctx: &ReducerContext) {
    World::builder()
        .debug(true)
        .debug_navigation(false)
        .build()
        .insert(ctx);

    create_zombie_behavior_tree(ctx);

    WorldTick::schedule(ctx);
    ZombieUpdateTick::schedule(ctx);
    ZombieSpawnTick::schedule(ctx);

    ZombieSpawnPoint::create(ctx, Vec3::new(0.0, 0.0, -50.0));
    ZombieSpawnPoint::create(ctx, Vec3::new(-16.0, 0.0, 0.0));
    ZombieSpawnPoint::create(ctx, Vec3::new(44.0, 0.0, 2.0));
    ZombieSpawnPoint::create(ctx, Vec3::new(0.0, 0.0, 40.0));
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
fn dummy(_ctx: &ReducerContext, _en: ExternalNavMesh) {
    // This reducer does nothing, it's just a placeholder to generate
    // the necessary serialization code for ExternalNavMesh.
}
