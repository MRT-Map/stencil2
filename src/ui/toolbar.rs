use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::{
    misc::{Action, ChangeStateAct, EditorState},
    ui::HoveringOverGui,
};

#[allow(clippy::needless_pass_by_value)]
pub fn ui_sy(
    state: Res<State<EditorState>>,
    mut ctx: EguiContexts,
    mut actions: EventWriter<Action>,
    mut hovering_over_gui: ResMut<HoveringOverGui>,
) {
    let mut new_state = state.0;
    let panel = egui::TopBottomPanel::top("toolbar").show(ctx.ctx_mut(), |ui| {
        egui::menu::bar(ui, |ui| {
            macro_rules! button {
                ($text:literal, $next_state:expr) => {
                    ui.selectable_value(&mut new_state, $next_state, $text)
                };
            }

            button!("Select", EditorState::Idle);

            ui.separator();
            button!("Edit Nodes", EditorState::EditingNodes);
            button!("Delete", EditorState::DeletingComponent);

            ui.separator();
            ui.label("Create...");
            button!("Point", EditorState::CreatingPoint);
            button!("Line", EditorState::CreatingLine);
            button!("Area", EditorState::CreatingArea);
        });
    });
    if panel.response.hovered() {
        hovering_over_gui.0 = true;
    }
    if new_state != state.0 {
        actions.send(Box::new(ChangeStateAct(new_state)));
    }
}
