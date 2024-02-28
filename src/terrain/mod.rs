use bevy::prelude::*;
use bevy_egui::EguiContexts;
use egui::{DragValue, FontId, Grid, RichText, Slider};

use self::resources::{Terrain, TerrainGenerationSettings};

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

fn terrain_ui(
    mut contexts: EguiContexts,
    mut terrain: ResMut<Terrain>,
    mut settings: ResMut<TerrainGenerationSettings>,
    mut commands: Commands,
) {
    let mut regenerate = false;
    egui::Window::new("Terrain").show(contexts.ctx_mut(), |ui| {
        ui.label(format!("Loaded Chunks: {}", terrain.len()));

        ui.add(
            Slider::new(&mut settings.chunks_radius, 1..=32)
                .text("Chunk Radius")
                .step_by(1.0),
        );

        ui.separator();
        ui.label(RichText::new("Generation").font(FontId::proportional(20.0)));
        Grid::new("generation-grid")
            .num_columns(2)
            .spacing([40.0, 4.0])
            .striped(true)
            .show(ui, |ui| {
                ui.label("Seed");

                if ui.add(DragValue::new(&mut settings.seed)).changed() {
                    regenerate = true;
                }

                ui.end_row();

                ui.label("Magnitude");
                ui.add(DragValue::new(&mut settings.magnitude).speed(0.05));
                ui.end_row();

                ui.label("Scale");
                ui.add(DragValue::new(&mut settings.scale).speed(0.05));
                ui.end_row();

                ui.label("Octaves");
                ui.add(DragValue::new(&mut settings.octaves).speed(1.0));
                ui.end_row();

                ui.label("Lacunarity");
                ui.add(DragValue::new(&mut settings.lacunarity).speed(0.051));
                ui.end_row();

                ui.label("Persistence");
                ui.add(DragValue::new(&mut settings.persistence).speed(0.05));
                ui.end_row();

                ui.label("Frequency");
                ui.add(DragValue::new(&mut settings.frequency).speed(0.05));
            });
    });

    if regenerate {
        for chunk in terrain.chunks() {
            commands.entity(chunk.2).despawn();

            terrain.remove_chunk(chunk.0, chunk.1);
        }
    }
}
