use std::time::Duration;

use bevy::{prelude::*, render::render_resource::AsBindGroup};
use bevy_egui::EguiContexts;
use egui::{
    emath::RectTransform, Checkbox, CollapsingHeader, Color32, Frame, Pos2, Sense, Shape, Slider,
    Stroke,
};

use crate::{
    spectator::components::SpectatorCamera,
    terrain::lod_tree::{LODLeaf, LODTree},
};

use self::{
    components::DeletedTerrainChunk,
    resources::{Terrain, TerrainSettings},
};

pub mod components;
mod generation;
mod lod_tree;
pub mod resources;
mod systems;

pub const CHUNK_SIZE: u32 = 4;

pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<TerrainMaterial>::default());
        app.init_resource::<resources::TerrainSettings>();
        app.init_resource::<resources::Terrain>();

        app.add_systems(PreUpdate, systems::update_lod_tree);
        app.add_systems(Update, systems::poll_pending_chunks);
        app.add_systems(Update, systems::process_marked_for_deletion);

        app.add_systems(Update, terrain_ui);
    }
}

impl Material for TerrainMaterial {
    fn fragment_shader() -> bevy::render::render_resource::ShaderRef {
        "shaders/terrain/fragment.wgsl".into()
    }

    fn vertex_shader() -> bevy::render::render_resource::ShaderRef {
        "shaders/terrain/vertex.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct TerrainMaterial {
    #[uniform(0)]
    pub color: Color,
    pub alpha_mode: AlphaMode,
}

fn terrain_ui(
    mut contexts: EguiContexts,
    mut terrain: ResMut<Terrain>,
    mut settings: ResMut<TerrainSettings>,
    player: Query<&Transform, With<SpectatorCamera>>,
    mut commands: Commands,
) {
    let Ok(player) = player.get_single() else {
        return;
    };

    let mut regenerate = false;
    egui::Window::new("Terrain").show(contexts.ctx_mut(), |ui| {
        CollapsingHeader::new("Information")
            .default_open(true)
            .show(ui, |ui| {
                if ui
                    .add(Checkbox::new(&mut settings.wireframe, "Wireframe"))
                    .changed()
                {
                    regenerate = true;
                }
            });

        CollapsingHeader::new("Generation Parameters")
            .default_open(false)
            .show(ui, |ui| {
                let settings = &mut settings.generation;
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
                    .add(Slider::new(&mut settings.scale, 0.001..=1.0).text("Scale"))
                    .changed()
                {
                    regenerate = true;
                }

                if ui
                    .add(
                        Slider::new(&mut settings.octaves, 0..=10)
                            .text("Octaves")
                            .step_by(1.0),
                    )
                    .changed()
                {
                    regenerate = true;
                }

                if ui
                    .add(
                        Slider::new(&mut settings.lacunarity, 0.0..=5.0)
                            .text("Lacunarity")
                            .step_by(0.05),
                    )
                    .changed()
                {
                    regenerate = true;
                }

                if ui
                    .add(
                        Slider::new(&mut settings.persistence, 0.0..=1.0)
                            .text("Persistence")
                            .step_by(0.05),
                    )
                    .changed()
                {
                    regenerate = true;
                }

                if ui
                    .add(
                        Slider::new(&mut settings.frequency, 0.01..=1.0)
                            .text("Frequency")
                            .step_by(0.05),
                    )
                    .changed()
                {
                    regenerate = true;
                }

                if ui
                    .add(
                        Slider::new(&mut settings.exponentiation, 0.01..=3.0)
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

        CollapsingHeader::new("LOD Tree")
            .default_open(true)
            .show(ui, |ui| {
                if ui
                    .add(
                        Slider::new(&mut settings.lod.recheck_interval, 0.0..=5.0)
                            .text("Validate Interval"),
                    )
                    .changed()
                {
                    terrain.recheck_timer = Timer::new(
                        Duration::from_secs_f32(settings.lod.recheck_interval),
                        TimerMode::Repeating,
                    );
                }
                ui.add(Slider::new(&mut settings.lod.max, 10.0..=2000.0).text("Max"));
                ui.add(
                    Slider::new(&mut settings.lod.min, 0.0..=200.0)
                        .step_by(2.0)
                        .text("Min"),
                );
                ui.add(
                    Slider::new(&mut settings.lod.layer_penalty, 10.0..=500.0)
                        .text("Layer Penalty"),
                );

                Frame::canvas(ui.style()).show(ui, |ui| {
                    let (response, painter) =
                        ui.allocate_painter(ui.available_size_before_wrap(), Sense::hover());

                    let to_screen = egui::emath::RectTransform::from_to(
                        egui::Rect::from_min_size(Pos2::ZERO, response.rect.square_proportions()),
                        response.rect,
                    );

                    fn draw_tree(
                        tree: &LODTree,
                        transform: &RectTransform,
                        painter: &egui::Painter,
                    ) {
                        let tree_rect = tree.boundary;

                        let points = vec![
                            transform * Pos2::new(tree_rect.min.x, tree_rect.min.y),
                            transform * Pos2::new(tree_rect.max.x, tree_rect.min.y),
                            transform * Pos2::new(tree_rect.max.x, tree_rect.max.y),
                            transform * Pos2::new(tree_rect.min.x, tree_rect.max.y),
                            transform * Pos2::new(tree_rect.min.x, tree_rect.min.y),
                        ];

                        painter.extend(vec![Shape::rect_filled(
                            egui::Rect::from_min_max(
                                transform * Pos2::new(tree_rect.min.x, tree_rect.min.y),
                                transform * Pos2::new(tree_rect.max.x, tree_rect.max.y),
                            ),
                            0.0,
                            match &tree.leaf {
                                LODLeaf::Children(_) => Color32::from_rgb(255, 255, 255),
                                LODLeaf::Chunk(_) => Color32::from_rgb(118, 220, 118),
                                LODLeaf::Pending => Color32::from_rgb(167, 198, 231),
                            },
                        )]);

                        painter.extend(vec![Shape::line(
                            points,
                            Stroke::new(1.5, Color32::from_rgb(255, 255, 255)),
                        )]);

                        if let LODLeaf::Children(children) = &tree.leaf {
                            for child in children.iter() {
                                draw_tree(child, transform, painter);
                            }
                        }
                    }
                    let tree_rect = terrain.lod_tree.boundary;
                    let tree_size = egui::Rect::from_min_size(
                        Pos2::new(tree_rect.min.x, tree_rect.min.y),
                        egui::Vec2::new(tree_rect.max.x, tree_rect.max.y),
                    );

                    let to_canvas = egui::emath::RectTransform::from_to(
                        tree_size,
                        to_screen.transform_rect(egui::Rect::from_min_size(
                            Pos2::ZERO,
                            egui::Vec2::new(1.0, 1.0),
                        )),
                    );

                    draw_tree(&terrain.lod_tree, &to_canvas, &painter);

                    painter.extend(vec![Shape::circle_filled(
                        to_canvas * Pos2::new(player.translation.x, player.translation.z),
                        5.0,
                        egui::Color32::from_rgb(0, 100, 255),
                    )]);

                    response
                });
            });

        if regenerate {
            let mut chunks = Vec::new();
            terrain.lod_tree.get_child_chunks_recursive(&mut chunks);
            for chunk in chunks {
                commands.entity(chunk).insert(DeletedTerrainChunk);
            }

            terrain.lod_tree.leaf = LODLeaf::default();
        }
    });
}
