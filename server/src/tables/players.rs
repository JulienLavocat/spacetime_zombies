use spacetimedb::{table, Identity};

#[table(name = player)]
pub struct Player {
    #[primary_key]
    pub id: u64,
    #[unique]
    pub identity: Identity,
    pub name: String,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
