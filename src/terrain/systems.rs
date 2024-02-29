use bevy::{
    pbr::wireframe::Wireframe,
    prelude::*,
    tasks::{block_on, futures_lite::future},
};

use crate::{
    spectator::components::SpectatorCamera,
    terrain::{lod_tree::LODTree, resources::LODSettings},
};

use super::{
    components::{DeletedTerrainChunk, PendingTerrainChunk, TerrainChunk},
    resources::{Terrain, TerrainSettings},
};

pub fn update_lod_tree(
    mut terrain: ResMut<Terrain>,
    player: Query<&Transform, With<SpectatorCamera>>,
    settings: Res<TerrainSettings>,
) {
    let Ok(player) = player.get_single() else {
        return;
    };

    terrain.lod_tree.clear();

    fn process(tree: &mut LODTree, player: Vec2, mut layer: usize, settings: &LODSettings) {
        let distance = f32::max(
            settings.max + -(layer as f32) * settings.layer_penalty,
            settings.min,
        );

        if tree.boundary().center().distance_squared(player) / 100.0 < distance {
            tree.subdivide();
        }

        layer += 1;

        if let Some(children) = tree.children_mut() {
            for child in children.iter_mut() {
                process(child, player, layer, settings);
            }
        }
    }

    process(
        &mut terrain.lod_tree,
        Vec2::new(player.translation.x, player.translation.z),
        0,
        &settings.lod,
    );
}

pub fn poll_pending_chunks(
    mut commands: Commands,
    mut tasks: Query<(Entity, &mut PendingTerrainChunk)>,
    mut meshes: ResMut<Assets<Mesh>>,
    settings: Res<TerrainSettings>,
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
