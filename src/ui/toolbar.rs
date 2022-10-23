use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use iyes_loopless::prelude::*;

use crate::{
    component_tools::creating::{clear_created_component, CreatedQuery},
    pla2::{component::ComponentType, skin::Skin},
    setup::EditorState,
    ui::{component_panel::PrevNamespaceUsed, HoveringOverGui},
};

pub fn ui_sy(
    mut ctx: ResMut<EguiContext>,
    mut _commands: Commands,
    mut hovering_over_gui: ResMut<HoveringOverGui>,
    mut cv: Local<&'static str>,
    mut _created_query: CreatedQuery,
    _skin: Res<Skin>,
    _prev_namespace_used: Res<PrevNamespaceUsed>,
) {
    let mut current_value = *cv;
    let panel = egui::TopBottomPanel::top("toolbar").show(ctx.ctx_mut(), |ui| {
        egui::menu::bar(ui, |ui| {
            macro_rules! button {
                ($value:literal, $text:literal, $next_state:expr) => {
                    if ui
                        .selectable_value(&mut current_value, $value, $text)
                        .clicked()
                    {
                        clear_created_component(
                            &mut _commands,
                            &mut _created_query,
                            &_skin,
                            &_prev_namespace_used.0,
                        );
                        _commands.insert_resource(NextState($next_state));
                    }
                };
            }

            button!("", "Select", EditorState::Idle);

            ui.separator();
            button!("edit_nodes", "Edit Nodes", EditorState::EditingNodes);
            button!("delete", "Delete", EditorState::DeletingComponent);

            ui.separator();
            ui.label("Create...");
            button!(
                "point",
                "Point",
                EditorState::CreatingComponent(ComponentType::Point)
            );
            button!(
                "line",
                "Line",
                EditorState::CreatingComponent(ComponentType::Line)
            );
            button!(
                "area",
                "Area",
                EditorState::CreatingComponent(ComponentType::Area)
            );
        });
    });
    if panel.response.hovered() {
        hovering_over_gui.0 = true;
    }
    *cv = current_value;
}