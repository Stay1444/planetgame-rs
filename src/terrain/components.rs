use bevy::{prelude::*, tasks::Task};

#[derive(Component)]
pub struct TerrainChunk(pub Handle<Mesh>, pub Vec2);

#[derive(Component)]
pub struct PendingTerrainChunk(pub Task<Mesh>, pub Vec2);

#[derive(Component)]
pub struct DeletedTerrainChunk;
