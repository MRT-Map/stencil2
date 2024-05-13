use std::sync::{Arc, Mutex};

use bevy::{ecs::system::SystemParam, prelude::*, utils::synccell::SyncCell};
use bevy_egui::{egui, egui::Margin, EguiContexts};
use egui_dock::{DockArea, DockState, NodeIndex, Style, TabBodyStyle, TabStyle};
use egui_file_dialog::FileDialog;
use enum_dispatch::enum_dispatch;

use crate::{
    misc::Action,
    pla2::{
        bundle::SelectedComponent,
        component::{EditorCoords, PlaComponent},
        skin::Skin,
    },
    state::EditorState,
    ui::{
        panel::{
            component_editor::{ComponentEditor, PrevNamespaceUsed},
            status::Status,
            tilemap::Tilemap,
        },
        popup::Popup,
        tilemap::{settings::TileSettings, settings_editor::TileSettingsEditor},
    },
    window_settings::{settings::WindowSettings, settings_editor::WindowSettingsEditor},
};

#[enum_dispatch(DockWindows)]
pub trait DockWindow: Copy {
    fn title(self) -> String;
    fn ui(self, tab_viewer: &mut TabViewer, ui: &mut egui::Ui);
    fn allowed_in_windows(self) -> bool {
        true
    }
    fn closeable(self) -> bool {
        true
    }
}

#[enum_dispatch]
#[derive(Clone, Copy)]
pub enum DockWindows {
    Tilemap,
    ComponentEditor,
    WindowSettingsEditor,
    TileSettingsEditor,
}

#[derive(Resource)]
pub struct PanelDockState {
    pub state: DockState<DockWindows>,
    pub viewport_rect: egui::Rect,
    pub layer_id: egui::LayerId,
}

impl Default for PanelDockState {
    fn default() -> Self {
        let mut state = DockState::new(vec![Tilemap.into()]);
        let tree = state.main_surface_mut();
        let [_, _inspector] =
            tree.split_left(NodeIndex::root(), 0.15, vec![ComponentEditor.into()]);

        Self {
            state,
            layer_id: egui::LayerId::background(),
            viewport_rect: egui::Rect::NOTHING,
        }
    }
}

impl PanelDockState {
    fn ui(&mut self, params: PanelParams, file_dialogs: Res<FileDialogs>, ctx: &mut egui::Context) {
        let mut tab_viewer = TabViewer {
            params,
            viewport_rect: &mut self.viewport_rect,
            layer_id: &mut self.layer_id,
            file_dialogs,
        };

        DockArea::new(&mut self.state)
            .style(Style::from_egui(ctx.style().as_ref()))
            .show(ctx, &mut tab_viewer);
    }
}

#[derive(Resource)]
pub struct TempUi<'a>(pub &'a mut egui::Ui);

pub struct TabViewer<'a, 'w, 's> {
    pub params: PanelParams<'w, 's>,
    pub viewport_rect: &'a mut egui::Rect,
    pub layer_id: &'a mut egui::LayerId,
    pub file_dialogs: Res<'w, FileDialogs>,
}

#[derive(Resource)]
pub struct FileDialogs {
    pub tile_settings: SyncCell<FileDialog>,
}

impl Default for FileDialogs {
    fn default() -> Self {
        Self {
            tile_settings: TileSettingsEditor::dialog(),
        }
    }
}

impl egui_dock::TabViewer for TabViewer<'_, '_, '_> {
    type Tab = DockWindows;

    fn title(&mut self, window: &mut Self::Tab) -> egui::WidgetText {
        window.title().into()
    }

    fn ui(&mut self, ui: &mut egui::Ui, window: &mut Self::Tab) {
        window.ui(self, ui);
    }

    fn closeable(&mut self, tab: &mut Self::Tab) -> bool {
        tab.closeable()
    }

    fn tab_style_override(&self, tab: &Self::Tab, global_style: &TabStyle) -> Option<TabStyle> {
        matches!(tab, DockWindows::Tilemap(_)).then(|| TabStyle {
            tab_body: TabBodyStyle {
                inner_margin: Margin::ZERO,
                ..global_style.tab_body
            },
            ..global_style.to_owned()
        })
    }

    fn allowed_in_windows(&self, tab: &mut Self::Tab) -> bool {
        tab.allowed_in_windows()
    }

    fn clear_background(&self, window: &Self::Tab) -> bool {
        !matches!(window, DockWindows::Tilemap(_))
    }
}

#[derive(SystemParam)]
#[non_exhaustive]
pub struct PanelParams<'w, 's> {
    pub selected:
        Query<'w, 's, (Entity, &'static mut PlaComponent<EditorCoords>), With<SelectedComponent>>,
    pub commands: Commands<'w, 's>,
    pub skin: Res<'w, Skin>,
    pub prev_namespace_used: ResMut<'w, PrevNamespaceUsed>,
    pub actions: EventWriter<'w, Action>,
    pub editor_state: Res<'w, State<EditorState>>,
    pub window_settings: ResMut<'w, WindowSettings>,
    pub tile_settings: ResMut<'w, TileSettings>,
    pub status: ResMut<'w, Status>,
    pub popup: EventWriter<'w, Popup>,
}

pub fn panel_sy(
    mut state: ResMut<PanelDockState>,
    file_dialogs: Res<FileDialogs>,
    mut ctx: EguiContexts,
    params: PanelParams,
) {
    state.ui(params, file_dialogs, ctx.ctx_mut());
}

#[must_use]
pub fn within_tilemap(ctx: &mut EguiContexts, panel: &Res<PanelDockState>) -> bool {
    ctx.ctx_mut()
        .rect_contains_pointer(panel.layer_id, panel.viewport_rect)
}