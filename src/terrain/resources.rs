use bevy::{prelude::*, utils::HashMap};

use super::CHUNK_SIZE;

#[derive(Resource)]
pub struct TerrainGenerationSettings {
    pub material: Handle<StandardMaterial>,
}

impl FromWorld for TerrainGenerationSettings {
    fn from_world(world: &mut World) -> Self {
        let mut images = world
            .get_resource_mut::<Assets<Image>>()
            .expect("Image Assets");

        let texture = images.add(crate::uv_debug_texture());

        let mut materials = world
            .get_resource_mut::<Assets<StandardMaterial>>()
            .expect("StandardMaterial Assets");

        let debug_material = materials.add(StandardMaterial {
            base_color_texture: Some(texture),
            ..default()
        });

        Self {
            material: debug_material,
        }
    }
}

#[derive(Resource, Default)]
pub struct Terrain {
    chunks: HashMap<(i32, i32), Entity>,
}

impl Terrain {
    pub fn len(&self) -> usize {
        self.chunks.len()
    }

    pub fn get_chunk(&self, x: i32, z: i32) -> Option<Entity> {
        self.chunks.get(&(x, z)).cloned()
    }

    pub fn set_chunk(&mut self, x: i32, z: i32, entity: Entity) {
        self.chunks.insert((x, z), entity);
    }

    pub fn remove_chunk(&mut self, x: i32, z: i32) {
        self.chunks.remove(&(x, z));
    }

    pub fn get_chunk_global(&self, x: f32, z: f32) -> Option<Entity> {
        let x = x as i32 / CHUNK_SIZE as i32;
        let z = z as i32 / CHUNK_SIZE as i32;

        self.chunks.get(&(x, z)).cloned()
    }
}
