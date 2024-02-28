use bevy::{prelude::*, tasks::Task};

#[derive(Component)]
pub struct TerrainChunk(pub Handle<Mesh>);

#[derive(Component)]
pub struct PendingTerrainChunk(pub Task<Mesh>);

#[derive(Component)]
pub struct DeletedTerrainChunk;
