use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};
use bevy_egui::EguiContexts;

pub struct DiagnosticsPlugin;
impl Plugin for DiagnosticsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FrameTimeDiagnosticsPlugin::default());
        app.add_systems(Update, render);
    }
}

fn render(mut contexts: EguiContexts, diagnostics: Res<DiagnosticsStore>) {
    egui::Window::new("Diagnostics").show(contexts.ctx_mut(), |ui| {
        ui.label(format!(
            "FPS: {}",
            diagnostics
                .get(&FrameTimeDiagnosticsPlugin::FPS)
                .and_then(|fps| fps.smoothed())
                .unwrap_or_default()
                .trunc()
        ));
    });
}
