use bevy::{
    app::AppExit,
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};
use bevy_egui::{egui, egui::Align, EguiContext};
use native_dialog::MessageType;

use crate::{
    editor::ui::HoveringOverGui,
    types::pla::{EditorCoords, PlaComponent},
};

pub fn ui_sy(
    mut ctx: ResMut<EguiContext>,
    mut hovering_over_gui: ResMut<HoveringOverGui>,
    mut exit: EventWriter<AppExit>,
    diagnostics: Res<Diagnostics>,
    components: Query<(), With<PlaComponent<EditorCoords>>>,
) {
    let panel = egui::TopBottomPanel::top("menu").show(ctx.ctx_mut(), |ui| {
        egui::menu::bar(ui, |ui| {
            egui::menu::menu_button(
                ui,
                format!("Stencil v{}", env!("CARGO_PKG_VERSION")),
                |ui| {
                    if ui.button("Quit").clicked()
                        && (components.is_empty()
                            || cfg!(debug_assertions)
                            || native_dialog::MessageDialog::default()
                                .set_title("Are you sure you want to exit?")
                                .set_text("You may have unsaved changes")
                                .set_type(MessageType::Warning)
                                .show_confirm()
                                .unwrap())
                    {
                        exit.send(AppExit);
                    }
                },
            );
            egui::menu::menu_button(ui, "File", |ui| {
                ui.label("Coming soon");
            });
            egui::menu::menu_button(ui, "Edit", |ui| {
                ui.label("Coming soon");
            });
            ui.with_layout(egui::Layout::right_to_left(Align::RIGHT), |ui| {
                ui.label(format!(
                    "FPS: {}",
                    if let Some(diagnostic) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
                        if let Some(fps) = diagnostic.average() {
                            format!("{:.2}", fps)
                        } else {
                            "???".to_string()
                        }
                    } else {
                        "???".to_string()
                    }
                ));
            })
        });
    });
    if panel.response.hovered() {
        hovering_over_gui.0 = true;
    }
}
