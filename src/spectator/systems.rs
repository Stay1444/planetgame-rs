use bevy::{
    input::mouse::MouseMotion,
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};
use bevy_xpbd_3d::components::LinearVelocity;

use super::{components::SpectatorCamera, resources::SpectatorSettings};

pub fn handle_movement(
    mut cameras: Query<(&Transform, &mut LinearVelocity), With<SpectatorCamera>>,
    keys: Res<ButtonInput<KeyCode>>,
    settings: Res<SpectatorSettings>,
) {
    for (transform, mut velocity) in cameras.iter_mut() {
        let mut movement = Vec3::ZERO;

        if keys.pressed(settings.controls.forward) {
            movement += transform.forward().xyz();
        } else if keys.pressed(settings.controls.back) {
            movement += transform.back().xyz();
        }

        if keys.pressed(settings.controls.left) {
            movement += transform.left().xyz();
        } else if keys.pressed(settings.controls.right) {
            movement += transform.right().xyz();
        }

        // Move on the absolute Y axis without taking into account camera rotation
        if keys.pressed(settings.controls.up) {
            movement += Vec3::new(0.0, 1.0, 0.0);
        } else if keys.pressed(settings.controls.down) {
            movement += Vec3::new(0.0, -1.0, 0.0);
        }

        velocity.0 = movement.normalize_or_zero() * settings.speed;
    }
}

pub fn handle_mouse_lock(
    mut settings: ResMut<SpectatorSettings>,
    keys: Res<ButtonInput<KeyCode>>,
    mut window: Query<&mut Window, With<PrimaryWindow>>,
) {
    if keys.just_pressed(settings.controls.toggle_mouse_lock) {
        settings.mouse_lock = !settings.mouse_lock;
    } else {
        return;
    }

    if let Ok(mut window) = window.get_single_mut() {
        if !settings.mouse_lock {
            window.cursor.grab_mode = CursorGrabMode::None;
            window.cursor.visible = true;
        } else {
            window.cursor.grab_mode = CursorGrabMode::Locked;
            window.cursor.visible = false;
        }
    }
}

pub fn handle_look(
    mut cameras: Query<&mut Transform, With<SpectatorCamera>>,
    mut motion: EventReader<MouseMotion>,
    settings: Res<SpectatorSettings>,
    window: Query<&Window, With<PrimaryWindow>>,
) {
    if !settings.mouse_lock {
        return;
    }

    let Ok(window) = window.get_single() else {
        return;
    };

    let mut pitch_offset = 0.0;
    let mut yaw_offset = 0.0;

    let window_scale = window.height().min(window.width());

    for event in motion.read() {
        pitch_offset +=
            (settings.sensitivity * 0.000001 * event.delta.y * window_scale).to_radians();
        yaw_offset += (settings.sensitivity * 0.000001 * event.delta.x * window_scale).to_radians();
    }

    for mut transform in cameras.iter_mut() {
        let (mut yaw, mut pitch, _) = transform.rotation.to_euler(EulerRot::YXZ);

        pitch -= pitch_offset;
        yaw -= yaw_offset;

        pitch = pitch.clamp(-1.55, 1.55);

        transform.rotation =
            Quat::from_axis_angle(Vec3::Y, yaw) * Quat::from_axis_angle(Vec3::X, pitch);
    }
}
