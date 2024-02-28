use bevy::prelude::*;
use bevy_egui::EguiContexts;
use egui::{Checkbox, FontId, RichText, Slider};

use self::{
    components::DeletedTerrainChunk,
    resources::{Terrain, TerrainGenerationSettings},
};

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
        app.add_systems(Update, systems::process_marked_for_deletion);

        app.add_systems(Update, terrain_ui);
    }
}

fn terrain_ui(
    mut contexts: EguiContexts,
    terrain: Res<Terrain>,
    mut settings: ResMut<TerrainGenerationSettings>,
    mut commands: Commands,
) {
    let mut regenerate = false;
    egui::Window::new("Terrain").show(contexts.ctx_mut(), |ui| {
        ui.label(format!("Loaded Chunks: {}", terrain.len()));

        ui.add(
            Slider::new(&mut settings.chunks_radius, 1..=100)
                .text("Chunk Radius")
                .step_by(1.0),
        );

        if ui
            .add(Checkbox::new(&mut settings.wireframe, "Wireframe"))
            .changed()
        {
            regenerate = true;
        }

        ui.separator();
        ui.label(RichText::new("Generation").font(FontId::proportional(20.0)));

        if ui
            .add(
                Slider::new(&mut settings.seed, 0u32..=u32::MAX / 2)
                    .text("Seed")
                    .step_by(1.0),
            )
            .changed()
        {
            regenerate = true;
        }

        if ui
            .add(
                Slider::new(&mut settings.amplitude, 0.01..=32.0)
                    .text("Amplitude")
                    .step_by(0.05),
            )
            .changed()
        {
            regenerate = true;
        }

        if ui
            .add(
                Slider::new(&mut settings.scale, 0.01..=200.0)
                    .text("Scale")
                    .step_by(0.05),
            )
            .changed()
        {
            regenerate = true;
        }

        if ui
            .add(
                Slider::new(&mut settings.octaves, 0..=5)
                    .text("Octaves")
                    .step_by(1.0),
            )
            .changed()
        {
            regenerate = true;
        }

        if ui
            .add(
                Slider::new(&mut settings.lacunarity, 0.0..=6.0)
                    .text("Lacunarity")
                    .step_by(0.05),
            )
            .changed()
        {
            regenerate = true;
        }

        if ui
            .add(
                Slider::new(&mut settings.persistence, 0.0..=10.0)
                    .text("Persistence")
                    .step_by(0.05),
            )
            .changed()
        {
            regenerate = true;
        }

        if ui
            .add(
                Slider::new(&mut settings.frequency, 0.01..=10.0)
                    .text("Frequency")
                    .step_by(0.05),
            )
            .changed()
        {
            regenerate = true;
        }

        if ui
            .add(
                Slider::new(&mut settings.exponentiation, 0.01..=10.0)
                    .text("Exponentiation")
                    .step_by(0.05),
            )
            .changed()
        {
            regenerate = true;
        }

        if ui
            .add(
                Slider::new(&mut settings.height, 0.01..=500.0)
                    .text("Height")
                    .step_by(0.05),
            )
            .changed()
        {
            regenerate = true;
        }
    });

    if regenerate {
        for chunk in terrain.chunks() {
            commands.entity(chunk.2).insert(DeletedTerrainChunk);
        }
    }
}
