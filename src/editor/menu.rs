use crate::HoveringOverGui;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

pub fn ui(mut ctx: ResMut<EguiContext>, mut hovering: ResMut<HoveringOverGui>) {
    let panel = egui::TopBottomPanel::top("menu").show(ctx.ctx_mut(), |ui| {
        egui::menu::bar(ui, |ui| {
            egui::menu::menu_button(
                ui,
                format!("Stencil v{}", env!("CARGO_PKG_VERSION")),
                |ui| {
                    if ui.button("Quit").clicked() {
                        std::process::exit(0);
                    }
                },
            );
            egui::menu::menu_button(ui, "File", |ui| {
                ui.label("Coming soon");
            });
            egui::menu::menu_button(ui, "Edit", |ui| {
                ui.label("Coming soon");
            });
        });
    });
    if panel.response.hovered() {
        hovering.0 = true;
    }
}