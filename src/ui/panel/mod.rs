pub mod component_editor;
pub mod dock;
pub mod menu;
pub mod status;
pub mod tilemap;
pub mod toolbar;

use std::sync::Arc;

use bevy::prelude::*;
use egui_file_dialog::FileDialog;

use crate::ui::{
    panel::{
        component_editor::PrevNamespaceUsed,
        dock::{FileDialogs, PanelDockState},
        status::Status,
    },
    UiSchedule, UiSet,
};

pub struct PanelPlugin;

impl Plugin for PanelPlugin {
    fn build(&self, app: &mut App) {
        app.world.insert_non_send_resource(FileDialogs::default());
        app.init_resource::<PrevNamespaceUsed>()
            .init_resource::<PanelDockState>()
            .init_resource::<Status>()
            .add_systems(
                UiSchedule,
                menu::ui_sy.in_set(UiSet::Panels).before(dock::panel_sy),
            )
            .add_systems(UiSchedule, dock::panel_sy.in_set(UiSet::Panels));
    }
}
