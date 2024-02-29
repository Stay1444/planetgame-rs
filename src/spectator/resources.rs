use bevy::prelude::*;

#[derive(Resource)]
pub struct SpectatorSettings {
    pub sensitivity: f32,
    pub speed: f32,
    pub controls: SpectatorControls,
    pub mouse_lock: bool,
}

impl Default for SpectatorSettings {
    fn default() -> Self {
        Self {
            mouse_lock: true,
            sensitivity: 50.0,
            speed: 150.0,
            controls: Default::default(),
        }
    }
}

pub struct SpectatorControls {
    pub toggle_mouse_lock: KeyCode,

    pub forward: KeyCode,
    pub back: KeyCode,

    pub left: KeyCode,
    pub right: KeyCode,

    pub up: KeyCode,
    pub down: KeyCode,

    pub rot_left: KeyCode,
    pub rot_right: KeyCode,
}

impl Default for SpectatorControls {
    fn default() -> Self {
        Self {
            toggle_mouse_lock: KeyCode::Escape,

            forward: KeyCode::KeyW,
            back: KeyCode::KeyS,

            left: KeyCode::KeyA,
            right: KeyCode::KeyD,

            up: KeyCode::Space,
            down: KeyCode::ShiftLeft,

            rot_left: KeyCode::KeyQ,
            rot_right: KeyCode::KeyE,
        }
    }
}
