use bevy_egui::{egui, egui::InnerResponse};

use crate::{
    misc::Action,
    state::{ChangeStateAct, EditorState},
    ui::panel::dock::{PanelParams, TabViewer},
};

#[allow(clippy::needless_pass_by_value)]
pub fn toolbar(ui: &mut egui::Ui, tab_viewer: &mut TabViewer) -> InnerResponse<()> {
    let PanelParams {
        editor_state,
        actions,
        ..
    } = &mut tab_viewer.params;
    let mut new_state = ***editor_state;
    let resp = egui::TopBottomPanel::top("toolbar").show_inside(ui, |ui| {
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
    if new_state != ***editor_state {
        actions.send(Action::new(ChangeStateAct(new_state)));
    }
    resp
}
