use bevy::{
    prelude::*,
    render::{mesh::Indices, render_asset::RenderAssetUsages, render_resource::PrimitiveTopology},
};
use bevy_egui::EguiContexts;
use noise::{NoiseFn, SuperSimplex};

use self::resources::Terrain;

pub mod components;
mod generation;
pub mod resources;
mod systems;

pub const CHUNK_SIZE: u32 = 128;

pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<resources::TerrainGenerationSettings>();
        app.init_resource::<resources::Terrain>();
        app.add_systems(Update, systems::poll_pending_chunks);
        app.add_systems(Update, systems::enqueue_chunks_around_player);

        app.add_systems(Update, terrain_ui);
    }
}

fn terrain_ui(mut contexts: EguiContexts, terrain: Res<Terrain>) {
    egui::Window::new("Terrain").show(contexts.ctx_mut(), |ui| {
        ui.label(format!("Loaded Chunks: {}", terrain.len()));
    });
}

pub fn generate_mesh() -> Mesh {
    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD | RenderAssetUsages::MAIN_WORLD,
    );

    let simplex = SuperSimplex::new(69);

    mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        generate_vertices(CHUNK_SIZE, CHUNK_SIZE, &simplex),
    );

    mesh.insert_attribute(
        Mesh::ATTRIBUTE_NORMAL,
        generate_normals(CHUNK_SIZE, CHUNK_SIZE),
    );

    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, generate_uvs(CHUNK_SIZE, CHUNK_SIZE));

    mesh.insert_indices(Indices::U32(generate_indices(CHUNK_SIZE, CHUNK_SIZE)));

    mesh
}

fn generate_vertices(width: u32, height: u32, simplex: &SuperSimplex) -> Vec<[f32; 3]> {
    let mut vertices = Vec::new();
    for i in 0..width {
        for j in 0..height {
            let x = i as f32;
            let z = j as f32;
            let y = simplex.get([(x * 0.01) as f64, (z * 0.01) as f64]) as f32 * 20.0;

            vertices.push([x, y, z]);
        }
    }

    vertices
}

fn generate_indices(width: u32, height: u32) -> Vec<u32> {
    let mut indices = Vec::new();

    for i in 0..width - 1 {
        for j in 0..height - 1 {
            let index = i * width + j;
            let next_index = index + 1;
            let bottom_index = (i + 1) * width + j;
            let next_bottom_index = bottom_index + 1;

            indices.push(index);
            indices.push(bottom_index);
            indices.push(next_index);

            indices.push(next_index);
            indices.push(bottom_index);
            indices.push(next_bottom_index);
        }
    }

    indices
}

fn generate_normals(width: u32, height: u32) -> Vec<[f32; 3]> {
    let mut normals = Vec::new();
    for _ in 0..width {
        for _ in 0..height {
            normals.push([0.0, 1.0, 0.0]); // Assuming normal vector pointing upwards (0, 0, 1)
        }
    }
    normals
}

fn generate_uvs(width: u32, height: u32) -> Vec<[f32; 2]> {
    let mut uvs = Vec::new();
    for i in 0..width {
        for j in 0..height {
            let u = j as f32 / (width - 1) as f32; // Normalize to [0, 1]
            let v = i as f32 / (height - 1) as f32; // Normalize to [0, 1]
            uvs.push([u, v]);
        }
    }
    uvs
}
