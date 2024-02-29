use bevy::prelude::*;
use bevy_egui::EguiContexts;
use egui::Slider;

use self::resources::SpectatorSettings;

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

        app.add_systems(Update, spectator_ui);
    }
}

fn spectator_ui(mut contexts: EguiContexts, mut settings: ResMut<SpectatorSettings>) {
    egui::Window::new("Spectator").show(contexts.ctx_mut(), |ui| {
        ui.add(
            Slider::new(&mut settings.sensitivity, 1.0..=100.0)
                .step_by(1.0)
                .text("sensitivity"),
        );

        ui.add(
            Slider::new(&mut settings.speed, 100.0..=5000.0)
                .step_by(150.0)
                .text("Speed"),
        );
    });
}
