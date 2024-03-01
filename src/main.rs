use bevy::{
    pbr::{wireframe::WireframePlugin, CascadeShadowConfigBuilder},
    prelude::*,
    render::{
        settings::{RenderCreation, WgpuFeatures, WgpuSettings},
        RenderPlugin,
    },
    window::{CursorGrabMode, PrimaryWindow},
};
use bevy_egui::EguiPlugin;
use bevy_xpbd_3d::{
    components::RigidBody,
    plugins::{
        collision::{Collider, Sensor},
        PhysicsPlugins,
    },
};
use diagnostics::DiagnosticsPlugin;
use spectator::{components::SpectatorCamera, SpectatorPlugin};
use terrain::TerrainPlugin;

mod diagnostics;
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
        .add_plugins(DiagnosticsPlugin)
        .add_systems(Update, start.run_if(run_once()))
        .run();
}

fn start(mut window: Query<&mut Window, With<PrimaryWindow>>, mut commands: Commands) {
    if let Ok(mut window) = window.get_single_mut() {
        window.cursor.grab_mode = CursorGrabMode::Locked;
        window.cursor.visible = false;
        window.set_maximized(true);
    }

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::from_xyz(500.0, 10000.0, 5000.0).looking_at(Vec3::ZERO, Vec3::Y),
        cascade_shadow_config: CascadeShadowConfigBuilder {
            first_cascade_far_bound: 15.0,
            maximum_distance: 1000.0,
            ..Default::default()
        }
        .into(),
        ..Default::default()
    });

    commands.spawn((
        Camera3dBundle {
            camera_3d: Camera3d::default(),
            transform: Transform::from_xyz(250.0, 250.0, 250.0)
                .looking_at(Vec3::ZERO, Vec3::new(250.0, 0.0, 250.0)),
            ..Default::default()
        },
        SpectatorCamera,
        RigidBody::Kinematic,
        Collider::default(),
        Sensor,
    ));
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
