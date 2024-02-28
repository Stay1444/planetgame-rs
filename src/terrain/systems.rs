use bevy::{
    pbr::wireframe::Wireframe,
    prelude::*,
    tasks::{block_on, futures_lite::future, AsyncComputeTaskPool},
};

use crate::{spectator::components::SpectatorCamera, terrain::generation::ChunkGenerator};

use super::{
    components::{DeletedTerrainChunk, PendingTerrainChunk, TerrainChunk},
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
                .spawn((PbrBundle {
                    mesh,
                    material: settings.material.clone(),
                    ..Default::default()
                },))
                .id();

            if settings.wireframe {
                commands.entity(child).insert(Wireframe);
            }

            commands.entity(entity).add_child(child);
        }
    }
}

pub fn enqueue_chunks_around_player(
    player: Query<&Transform, With<SpectatorCamera>>,
    generation_settings: Res<TerrainGenerationSettings>,
    mut terrain: ResMut<Terrain>,
    mut commands: Commands,
) {
    let Ok(player) = player.get_single() else {
        return;
    };

    let chunks = super::generation::chunks_for_radius(
        generation_settings.chunks_radius as i32,
        player.translation.x,
        player.translation.z,
    );

    let mut missing = Vec::new();

    for chunk in chunks {
        if terrain.get_chunk(chunk.0, chunk.1).is_none() {
            missing.push(chunk);
        }
    }

    if missing.is_empty() {
        return;
    }

    let thread_pool = AsyncComputeTaskPool::get();

    for chunk in missing {
        let task = thread_pool.spawn({
            let chunk = chunk.clone();
            let settings = generation_settings.clone();
            async move {
                let mut generator = ChunkGenerator::new(settings);
                generator.resolution = 1;
                generator.position = (chunk.0, chunk.1);
                generator.generate()
            }
        });

        let chunk_pos = super::generation::chunk_to_global_position(chunk.0, chunk.1);
        println!("Enqueued chunk at {}, {}", chunk.0, chunk.1);

        let entity = commands
            .spawn((
                TransformBundle {
                    local: Transform::from_xyz(chunk_pos.0, 0.0, chunk_pos.1),
                    ..Default::default()
                },
                PendingTerrainChunk(task),
                VisibilityBundle::default(),
            ))
            .id();

        terrain.set_chunk(chunk.0, chunk.1, entity);
    }
}

pub fn process_marked_for_deletion(
    mut terrain: ResMut<Terrain>,
    chunks: Query<(Entity, &Transform), (With<DeletedTerrainChunk>, Without<PendingTerrainChunk>)>,
    mut commands: Commands,
) {
    for (entity, transform) in &chunks {
        let position = super::generation::global_to_chunk_position(
            transform.translation.x,
            transform.translation.z,
        );

        terrain.remove_chunk(position.0, position.1);

        commands.entity(entity).despawn_recursive();
    }
}
