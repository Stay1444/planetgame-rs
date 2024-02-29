use bevy::{
    prelude::*,
    render::{mesh::Indices, render_asset::RenderAssetUsages, render_resource::PrimitiveTopology},
};
use noise::{NoiseFn, SuperSimplex};

use crate::terrain::resources::GenerationSettings;

pub use super::super::CHUNK_SIZE;

pub struct ChunkGenerator {
    pub resolution: i32,
    pub position: (i32, i32),
    pub scale: Vec2,
    settings: GenerationSettings,
}

impl ChunkGenerator {
    pub fn new(settings: GenerationSettings) -> Self {
        Self {
            settings,
            scale: Vec2::new(1.0, 1.0),
            position: (0, 0),
            resolution: 1,
        }
    }

    pub fn generate(&self) -> Mesh {
        let mut mesh = Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::RENDER_WORLD | RenderAssetUsages::MAIN_WORLD,
        );

        let noise = SuperSimplex::new(self.settings.seed);

        let position = super::chunk_to_global_position(self.position.0, self.position.1);

        mesh.insert_attribute(
            Mesh::ATTRIBUTE_POSITION,
            generate_vertices(position, &noise, self.scale, &self.settings),
        );

        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, generate_normals());

        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, generate_uvs());

        mesh.insert_indices(Indices::U32(generate_indices()));

        mesh
    }
}

fn generate_vertices<T: NoiseFn<f64, 2>>(
    position: (f32, f32),
    noise: &T,
    scale: Vec2,
    settings: &GenerationSettings,
) -> Vec<[f32; 3]> {
    let mut vertices = Vec::new();

    for i in 0..CHUNK_SIZE + 2 {
        for j in 0..CHUNK_SIZE + 2 {
            let x = i as f32;
            let z = j as f32;

            let nx = (((position.0 + x) / settings.scale) as f64) * scale.x as f64;
            let nz = (((position.1 + z) / settings.scale) as f64) * scale.y as f64;

            let g = 2.0f64.powf(-settings.persistence);
            let mut total = 0f64;
            let mut normalization = 0f64;
            let mut amplitude = settings.amplitude;
            let mut frequency = settings.frequency;

            for _ in 0..settings.octaves {
                let noise_v = noise.get([nx * frequency * 0.5 + 0.5, nz * frequency * 0.5 + 0.5]);

                total += noise_v * amplitude;
                normalization += amplitude;
                amplitude *= g;
                frequency *= settings.lacunarity;
            }

            total /= normalization;

            let mut y = total.powf(settings.exponentiation) * settings.height;

            if y.is_nan() {
                y = 0.0;
            }

            vertices.push([x, y as f32, z]);
        }
    }

    vertices
}

fn generate_indices() -> Vec<u32> {
    let mut indices = Vec::new();

    let width = CHUNK_SIZE + 2;
    let height = CHUNK_SIZE + 2;

    for i in 0..width - 1 {
        for j in 0..height - 1 {
            let index = i * width + j;
            let next_index = index + 1;
            let bottom_index = index + width;
            let next_bottom_index = bottom_index + 1;

            indices.push(index);
            indices.push(next_index);
            indices.push(bottom_index);

            indices.push(next_index);
            indices.push(next_bottom_index);
            indices.push(bottom_index);
        }
    }

    indices
}

fn generate_normals() -> Vec<[f32; 3]> {
    let mut normals = Vec::new();

    let width = CHUNK_SIZE + 2;
    let height = CHUNK_SIZE + 2;

    for _ in 0..width {
        for _ in 0..height {
            normals.push([0.0, 1.0, 0.0]); // Assuming normal vector pointing upwards (0, 0, 1)
        }
    }
    normals
}

fn generate_uvs() -> Vec<[f32; 2]> {
    let mut uvs = Vec::new();
    let width = CHUNK_SIZE + 2;
    let height = CHUNK_SIZE + 2;
    for i in 0..width {
        for j in 0..height {
            let u = j as f32 / (width - 1) as f32; // Normalize to [0, 1]
            let v = i as f32 / (height - 1) as f32; // Normalize to [0, 1]
            uvs.push([u, v]);
        }
    }
    uvs
}
