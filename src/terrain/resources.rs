use bevy::{prelude::*, utils::HashMap};

#[derive(Resource, Clone)]
pub struct TerrainGenerationSettings {
    pub material: Handle<StandardMaterial>,
    pub chunks_radius: u32,
    pub seed: u32,
    pub magnitude: f32,
    pub scale: f32,
    pub octaves: usize,
    pub lacunarity: f64,
    pub persistence: f64,
    pub frequency: f64,
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
            chunks_radius: 1,

            seed: 100,
            magnitude: 1.0,
            scale: 1.0,
            octaves: 1,
            lacunarity: 2.0,
            persistence: 0.7,
            frequency: 0.1,
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

    pub fn chunks(&self) -> Vec<(i32, i32, Entity)> {
        let mut chunks = Vec::new();

        for (k, v) in self.chunks.iter() {
            chunks.push((k.0, k.1, v.clone()));
        }

        chunks
    }
}
