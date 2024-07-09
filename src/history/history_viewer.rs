use bevy::prelude::{Event, EventReader, ResMut, Trigger};
use bevy_egui::egui;
use itertools::Itertools;

use crate::{
    component::panels::component_editor::{ComponentEditor, OpenComponentEditorAct},
    history::HistoryAct,
    ui::panel::dock::{window_action_handler, DockWindow, PanelDockState, PanelParams, TabViewer},
};

#[derive(Clone, Copy)]
pub struct HistoryViewer;

#[derive(Clone, Copy, Event)]
pub struct OpenHistoryViewerAct;

impl DockWindow for HistoryViewer {
    fn title(self) -> String {
        "History".into()
    }
    fn ui(self, tab_viewer: &mut TabViewer, ui: &mut egui::Ui) {
        let PanelParams {
            history, commands, ..
        } = tab_viewer.params;
        ui.horizontal(|ui| {
            if ui.button("Undo").clicked() {
                commands.trigger(HistoryAct::Undo);
            }
            if ui.button("Redo").clicked() {
                commands.trigger(HistoryAct::Redo);
            }
        });
        for entry in &history.undo_stack {
            ui.label(entry.iter().map(ToString::to_string).join("; "));
        }
        ui.colored_label(egui::Color32::YELLOW, "Current State");
        for entry in history.redo_stack.iter().rev() {
            ui.label(entry.iter().map(ToString::to_string).join("; "));
        }
    }
}

pub fn on_history_viewer(
    _trigger: Trigger<OpenHistoryViewerAct>,
    mut state: ResMut<PanelDockState>,
) {
    window_action_handler(&mut state, HistoryViewer);
}
