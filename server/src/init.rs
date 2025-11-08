use log::info;
use spacetime_engine::{
    math::Vec3,
    navigation::{convert_godot_navmesh_to_landmass, steng_nav_mesh, NavMesh},
    world::World,
};
use spacetimedb::{reducer, ReducerContext, Table};

use crate::{
    world::WorldTick,
    zombies_spawner::{ZombieSpawnPoint, ZombieSpawnTick},
    zombies_tick::ZombieUpdateTick,
};

#[reducer(init)]
pub fn init(ctx: &ReducerContext) {
    World::builder().build().insert(ctx);

    WorldTick::schedule(ctx);
    ZombieUpdateTick::schedule(ctx);
    ZombieSpawnTick::schedule(ctx);

    ZombieSpawnPoint::create(ctx, Vec3::new(0.0, 0.0, -50.0));
    ZombieSpawnPoint::create(ctx, Vec3::new(-16.0, 0.0, 0.0));
    ZombieSpawnPoint::create(ctx, Vec3::new(44.0, 0.0, 2.0));
    ZombieSpawnPoint::create(ctx, Vec3::new(0.0, 0.0, 40.0));
}

#[reducer]
fn editor_upload_navmesh(ctx: &ReducerContext) {
    let file = include_str!("../../client/navmesh_export.json");
    info!("navmesh export file size: {} bytes", file.len());
    let original_navmesh =
        serde_json::from_str::<NavMesh>(file).expect("Failed to parse navmesh JSON");
    info!(
        "Parsed navmesh with {} vertices and {} polygons",
        original_navmesh.vertices.len(),
        original_navmesh.polygons.len()
    );

    let world_id = original_navmesh.world_id;

    info!(
        "Uploading navmesh with {} vertices and {} polygons for world {}",
        original_navmesh.vertices.len(),
        original_navmesh.polygons.len(),
        world_id
    );

    let converted_navmesh = convert_godot_navmesh_to_landmass(original_navmesh);

    converted_navmesh
        .clone()
        .validate()
        .expect("Uploaded navmesh is invalid");

    let mut steng_navmesh = NavMesh::from(converted_navmesh);
    steng_navmesh.world_id = world_id;
    ctx.db.steng_nav_mesh().insert(steng_navmesh);
}
