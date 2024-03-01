use std::time::Duration;

use bevy::prelude::*;

use super::{lod_tree::LODTree, TerrainMaterial};

#[derive(Resource, Clone)]
pub struct TerrainSettings {
    pub material: Handle<TerrainMaterial>,
    pub wireframe: bool,
    pub size: Vec2,
    pub generation: GenerationSettings,
    pub lod: LODSettings,
}

#[derive(Clone)]
pub struct GenerationSettings {
    pub seed: u32,
    pub amplitude: f64,
    pub scale: f32,
    pub octaves: usize,
    pub lacunarity: f64,
    pub persistence: f64,
    pub frequency: f64,
    pub exponentiation: f64,
    pub height: f64,
}

#[derive(Clone)]
pub struct LODSettings {
    pub recheck_interval: f32,
    pub max: f32,
    pub layer_penalty: f32,
    pub min: f32,
}

impl FromWorld for TerrainSettings {
    fn from_world(world: &mut World) -> Self {
        let mut materials = world
            .get_resource_mut::<Assets<TerrainMaterial>>()
            .expect("TerrainMaterial Assets");

        let mat = materials.add(TerrainMaterial {
            color: Color::RED,
            alpha_mode: AlphaMode::Blend,
        });

        Self {
            material: mat,
            wireframe: false,
            size: Vec2::new(50000.0, 50000.0),

            generation: GenerationSettings {
                seed: 100,
                amplitude: 0.01,
                scale: 0.005,
                octaves: 16,
                lacunarity: 1.7,
                persistence: 0.7,
                frequency: 0.11,
                exponentiation: 0.81,
                height: 550.0,
            },

            lod: LODSettings {
                recheck_interval: 0.0,
                max: 2000.0,
                layer_penalty: 300.0,
                min: 56.0,
            },
        }
    }
}

#[derive(Resource)]
pub struct Terrain {
    pub recheck_timer: Timer,
    pub lod_tree: LODTree,
}

impl FromWorld for Terrain {
    fn from_world(world: &mut World) -> Self {
        let settings = world.get_resource::<TerrainSettings>().unwrap();
        let lod_tree = LODTree::new(12, Rect::from_corners(Vec2::ZERO, settings.size));

        Self {
            recheck_timer: Timer::new(
                Duration::from_secs_f32(settings.lod.recheck_interval),
                TimerMode::Repeating,
            ),
            lod_tree,
        }
    }
}
