use bevy::{
    pbr::wireframe::Wireframe,
    prelude::*,
    tasks::{block_on, futures_lite::future, AsyncComputeTaskPool},
};

use crate::{
    spectator::components::SpectatorCamera,
    terrain::{
        generation::ChunkGenerator,
        lod_tree::{LODLeaf, LODTree},
        resources::LODSettings,
        CHUNK_SIZE,
    },
};

use super::{
    components::{DeletedTerrainChunk, PendingTerrainChunk, TerrainChunk},
    resources::{Terrain, TerrainSettings},
};

pub fn update_lod_tree(
    mut terrain: ResMut<Terrain>,
    mut commands: Commands,
    player: Query<&Transform, With<SpectatorCamera>>,
    settings: Res<TerrainSettings>,
    time: Res<Time>,
) {
    terrain.recheck_timer.tick(time.delta());
    if !terrain.recheck_timer.finished() {
        return;
    }

    let Ok(player) = player.get_single() else {
        return;
    };

    fn process(
        tree: &mut LODTree,
        player: Vec2,
        settings: &LODSettings,
        commands: &mut Commands,
        chunk_queue: &mut Vec<(Entity, Rect)>,
    ) {
        match &tree.leaf {
            LODLeaf::Children(_) => {
                if !tree.should_collapse(settings, player) {
                    let mut cchunks = Vec::new();
                    tree.get_child_chunks_recursive(&mut cchunks);
                    for chunk in cchunks {
                        commands.entity(chunk).insert(DeletedTerrainChunk);
                    }
                    tree.leaf = LODLeaf::Pending;
                }
            }
            LODLeaf::Chunk(entity) => {
                if tree.should_collapse(settings, player) && tree.can_collapse() {
                    commands.entity(entity.clone()).insert(DeletedTerrainChunk);
                    assert!(tree.collapse());
                }
            }
            LODLeaf::Pending => {
                if tree.should_collapse(settings, player) && tree.can_collapse() {
                    tree.collapse();
                } else {
                    let entity = commands
                        .spawn((
                            TransformBundle {
                                local: Transform::from_xyz(
                                    tree.boundary.min.x,
                                    0.0,
                                    tree.boundary.min.y,
                                ),
                                ..Default::default()
                            },
                            VisibilityBundle::default(),
                        ))
                        .id();

                    tree.leaf = LODLeaf::Chunk(entity.clone());
                    chunk_queue.push((entity, tree.boundary));
                }
            }
        }

        if let LODLeaf::Children(children) = &mut tree.leaf {
            for child in children.iter_mut() {
                process(child, player, settings, commands, chunk_queue);
            }
        }
    }

    let mut chunk_queue = Vec::new();

    process(
        &mut terrain.lod_tree,
        Vec2::new(player.translation.x, player.translation.z),
        &settings.lod,
        &mut commands,
        &mut chunk_queue,
    );

    let thread_pool = AsyncComputeTaskPool::get();

    for chunk in chunk_queue {
        let target_chunk_size = chunk.1.size();
        let chunk_size = Vec2::new(
            target_chunk_size.x / CHUNK_SIZE as f32,
            target_chunk_size.y / CHUNK_SIZE as f32,
        );
        let task = thread_pool.spawn({
            let chunk = chunk.clone();
            let settings = settings.generation.clone();

            async move {
                let mut generator = ChunkGenerator::new(settings);
                generator.resolution = 1;
                generator.position = chunk.1.min;

                generator.scale = chunk_size;

                generator.generate()
            }
        });

        dbg!(&chunk_size);

        commands
            .entity(chunk.0)
            .insert(PendingTerrainChunk(task, chunk_size));
    }
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
            commands
                .entity(entity)
                .insert(TerrainChunk(mesh.clone(), task.1));

            let child = commands
                .spawn((PbrBundle {
                    mesh,
                    material: settings.material.clone(),
                    transform: Transform::from_scale(Vec3::new(task.1.x, 1.0, task.1.y)),
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
    chunks: Query<Entity, (With<DeletedTerrainChunk>, Without<PendingTerrainChunk>)>,
    mut commands: Commands,
) {
    for entity in &chunks {
        commands.entity(entity).despawn_recursive();
    }
}
