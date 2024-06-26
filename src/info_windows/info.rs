use std::sync::Mutex;

use bevy::prelude::*;
use bevy_egui::egui;

use crate::{action::Action, info_windows::InfoWindowsAct, ui::popup::Popup};

#[allow(clippy::needless_pass_by_value)]
pub fn info_asy(mut actions: EventReader<Action>, mut popup: EventWriter<Popup>) {
    for event in actions.read() {
        if matches!(event.downcast_ref(), Some(InfoWindowsAct::Info)) {
            popup.send(Popup::new(
                "info",
                || {
                    egui::Window::new(format!("Stencil v{}", env!("CARGO_PKG_VERSION")))
                        .collapsible(false)
                        .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
                },
                move |_, ui, _, shown| {
                    ui.add(
                        egui::Image::new(egui::include_image!("../../assets/ste-light.png"))
                            .fit_to_exact_size(egui::vec2(975.0 / 4.0, 569.0 / 4.0)),
                    );
                    ui.label("Made by __7d for the MRT Mapping Services");
                    ui.hyperlink_to("GitHub", "https://github.com/MRT-Map/stencil2");
                    if ui.button("Close").clicked() {
                        *shown = false;
                    }
                },
                Mutex::new(Box::new(())),
            ));
        }
    }
}
