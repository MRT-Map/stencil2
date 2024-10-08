use bevy::prelude::{Event, ResMut, Trigger};
use bevy_egui::egui;
use chrono::{DateTime, Utc};

use crate::ui::{
    notif::NOTIF_LOG,
    panel::dock::{window_action_handler, DockWindow, PanelDockState, PanelParams, TabViewer},
};

#[derive(Clone, Copy)]
pub struct NotifLogViewer;

#[derive(Clone, Copy, Event)]
pub struct OpenNotifLogViewerEv;

impl DockWindow for NotifLogViewer {
    fn title(self) -> String {
        "Notification Log".into()
    }
    fn ui(self, tab_viewer: &mut TabViewer, ui: &mut egui::Ui) {
        let PanelParams { .. } = tab_viewer.params;
        let Ok(notif_log) = NOTIF_LOG.try_read() else {
            ui.label("Loading...");
            return;
        };
        for entry in notif_log.notifs.iter().rev() {
            ui.colored_label(
                egui::Color32::WHITE,
                format!(
                    "{}",
                    DateTime::<Utc>::from(entry.timestamp).format("%d/%m/%Y %T")
                ),
            );
            ui.colored_label(egui::Color32::YELLOW, &entry.message);
            ui.separator();
        }
        if notif_log.notifs.is_empty() {
            ui.label("No errors, thankfully");
        }
    }
}

#[expect(clippy::needless_pass_by_value)]
pub fn on_log_viewer(_trigger: Trigger<OpenNotifLogViewerEv>, mut state: ResMut<PanelDockState>) {
    window_action_handler(&mut state, NotifLogViewer);
}
