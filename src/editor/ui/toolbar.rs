use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use iyes_loopless::prelude::*;

use crate::{
    editor::{modes::creating::clear_created_component, ui::HoveringOverGui},
    types::{skin::Skin, ComponentType, CreatedQuery, EditorState},
};

pub fn ui_sy(
    mut ctx: ResMut<EguiContext>,
    mut _commands: Commands,
    mut hovering_over_gui: ResMut<HoveringOverGui>,
    mut cv: Local<&'static str>,
    _created_query: CreatedQuery,
    _skin: Res<Skin>,
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
                        clear_created_component(&mut _commands, &_created_query, &_skin);
                        _commands.insert_resource(NextState($next_state));
                    }
                };
            }

            button!("", "Select", EditorState::Idle);

            ui.separator();
            button!("edit_nodes", "Edit Nodes", EditorState::EditingNodes);
            button!("move", "Move", EditorState::MovingComponent);
            button!("rotate", "Rotate", EditorState::RotatingComponent);
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
