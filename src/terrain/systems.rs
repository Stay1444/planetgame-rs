use bevy::{
    pbr::wireframe::Wireframe,
    prelude::*,
    tasks::{block_on, futures_lite::future},
};

use crate::spectator::components::SpectatorCamera;

use super::{
    components::{PendingTerrainChunk, TerrainChunk},
    resources::{Terrain, TerrainGenerationSettings},
};

pub fn poll_pending_chunks(
    mut commands: Commands,
    mut tasks: Query<(Entity, &mut PendingTerrainChunk)>,
    mut meshes: ResMut<Assets<Mesh>>,
    settings: Res<TerrainGenerationSettings>,
) {
    for (entity, mut task) in tasks.iter_mut() {
        if let Some(mesh) = block_on(future::poll_once(&mut task.0)) {
            let mesh = meshes.add(mesh);
            commands.entity(entity).remove::<PendingTerrainChunk>();
            commands.entity(entity).insert(TerrainChunk(mesh.clone()));

            let child = commands
                .spawn((
                    PbrBundle {
                        mesh,
                        material: settings.material.clone(),
                        ..Default::default()
                    },
                    Wireframe,
                ))
                .id();

            commands.entity(entity).add_child(child);
        }
    }
}

pub fn enqueue_chunks_around_player(
    player: Query<&Transform, With<SpectatorCamera>>,
    terrain: Res<Terrain>,
) {
    let Ok(player) = player.get_single() else {
        return;
    };

    let chunk = terrain.get_chunk_global(player.translation.x, player.translation.z);

    dbg!(&chunk);
}
