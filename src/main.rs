use bevy::{
    pbr::wireframe::WireframePlugin,
    prelude::*,
    render::{
        settings::{RenderCreation, WgpuFeatures, WgpuSettings},
        RenderPlugin,
    },
    tasks::AsyncComputeTaskPool,
    window::{CursorGrabMode, PrimaryWindow},
};
use bevy_egui::{EguiContexts, EguiPlugin};
use bevy_xpbd_3d::{
    components::RigidBody,
    plugins::{
        collision::{Collider, Sensor},
        PhysicsPlugins,
    },
};
use spectator::{components::SpectatorCamera, SpectatorPlugin};
use terrain::{components::PendingTerrainChunk, resources::Terrain, TerrainPlugin};

mod spectator;
mod terrain;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(RenderPlugin {
                    render_creation: RenderCreation::Automatic(WgpuSettings {
                        features: WgpuFeatures::POLYGON_MODE_LINE,
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
            WireframePlugin,
        ))
        .add_plugins(PhysicsPlugins::default())
        .add_plugins(EguiPlugin)
        // -- GAME --
        .add_plugins(SpectatorPlugin)
        .add_plugins(TerrainPlugin)
        .add_systems(Update, ui_example_system)
        .add_systems(Update, start.run_if(run_once()))
        .run();
}

fn ui_example_system(mut contexts: EguiContexts) {
    egui::Window::new("Hello").show(contexts.ctx_mut(), |ui| {
        ui.label("world");
    });
}

fn start(
    mut window: Query<&mut Window, With<PrimaryWindow>>,
    mut commands: Commands,
    mut terrain: ResMut<Terrain>,
) {
    if let Ok(mut window) = window.get_single_mut() {
        window.cursor.grab_mode = CursorGrabMode::Confined;
        window.cursor.visible = false;
        window.set_maximized(true);
    }

    commands.spawn((
        Camera3dBundle {
            camera_3d: Camera3d::default(),
            transform: Transform::from_xyz(10.0, 5.0, 0.0)
                .looking_at(Vec3::ZERO, Vec3::new(0.0, 1.0, 0.0)),
            ..Default::default()
        },
        SpectatorCamera,
        RigidBody::Kinematic,
        Collider::default(),
        Sensor,
    ));

    let thread_pool = AsyncComputeTaskPool::get();

    let task = thread_pool.spawn(async move { terrain::generate_mesh() });

    let chunk = commands
        .spawn((
            TransformBundle::default(),
            PendingTerrainChunk(task),
            VisibilityBundle::default(),
        ))
        .id();

    terrain.set_chunk(0, 0, chunk);
}

fn uv_debug_texture() -> Image {
    const TEXTURE_SIZE: usize = 8;

    let mut palette: [u8; 32] = [
        255, 102, 159, 255, 255, 159, 102, 255, 236, 255, 102, 255, 121, 255, 102, 255, 102, 255,
        198, 255, 102, 198, 255, 255, 121, 102, 255, 255, 236, 102, 255, 255,
    ];

    let mut texture_data = [0; TEXTURE_SIZE * TEXTURE_SIZE * 4];
    for y in 0..TEXTURE_SIZE {
        let offset = TEXTURE_SIZE * y * 4;
        texture_data[offset..(offset + TEXTURE_SIZE * 4)].copy_from_slice(&palette);
        palette.rotate_right(4);
    }

    Image::new_fill(
        bevy::render::render_resource::Extent3d {
            width: TEXTURE_SIZE as u32,
            height: TEXTURE_SIZE as u32,
            depth_or_array_layers: 1,
        },
        bevy::render::render_resource::TextureDimension::D2,
        &texture_data,
        bevy::render::render_resource::TextureFormat::Rgba8UnormSrgb,
        bevy::render::render_asset::RenderAssetUsages::RENDER_WORLD,
    )
}
