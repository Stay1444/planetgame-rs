use bevy::prelude::*;

pub mod components;
pub mod resources;
mod systems;

pub struct SpectatorPlugin;

impl Plugin for SpectatorPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<resources::SpectatorSettings>();
        app.add_systems(
            Update,
            (
                systems::handle_movement,
                systems::handle_look,
                systems::handle_mouse_lock,
            ),
        );
    }
}
