use std::sync::{Arc, Mutex};

use bevy::prelude::*;
use bevy_egui::egui;

use crate::{info_windows::InfoWindowsAct, misc::Action, ui::popup::Popup};

pub fn manual_asy(mut actions: EventReader<Action>, mut popup: EventWriter<Popup>) {
    for event in actions.iter() {
        if matches!(event.downcast_ref(), Some(InfoWindowsAct::Manual)) {
            popup.send(Popup::new(
                "manual",
                || {
                    egui::Window::new("Manual")
                        .collapsible(false)
                        .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
                        .resizable(false)
                },
                |_, ui, _, show| {
                    ui.label("Our online manual is available here:");
                    ui.hyperlink("https://github.com/MRT-Map/stencil2/wiki");
                    if ui.button("Close").clicked() {
                        *show = false;
                    }
                },
                Mutex::new(Box::new(())),
            ));
        }
    }
}
