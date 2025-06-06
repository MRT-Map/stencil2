use bevy::prelude::*;
use bevy_egui::egui;
use serde::{Deserialize, Serialize};
use surf::Url;

use crate::{
    dirs_paths::{cache_path, data_path},
    file::safe_delete,
    misc_config::settings::MiscSettings,
    ui::{
        map::mouse_nav::ScrollMode,
        panel::dock::{open_dock_window, DockLayout, DockWindow, PanelParams},
    },
};

#[derive(Clone, Copy, Event)]
pub struct OpenMiscSettingsEv;

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct MiscSettingsEditor;

impl DockWindow for MiscSettingsEditor {
    fn title(self) -> String {
        "Misc Settings".into()
    }
    fn ui(self, params: &mut PanelParams, ui: &mut egui::Ui) {
        let PanelParams { misc_settings, .. } = params;
        let mut invalid = false;
        let old_settings = misc_settings.to_owned();
        if ui
            .add_enabled(
                **misc_settings != MiscSettings::default(),
                egui::Button::new("Reset"),
            )
            .clicked()
        {
            **misc_settings = MiscSettings::default();
        }
        ui.colored_label(
            egui::Color32::YELLOW,
            format!(
                "Misc settings can also be edited at: {}",
                data_path("misc_settings.toml").to_string_lossy()
            ),
        );
        ui.separator();

        ui.add(egui::TextEdit::singleline(&mut misc_settings.skin_url).hint_text("Skin URL"));
        if let Err(e) = Url::try_from(&*misc_settings.skin_url) {
            ui.colored_label(egui::Color32::RED, format!("Invalid URL: {e:?}"));
            invalid = true;
        }
        ui.label("The URL for the skin used to render components. Will be cached and retrieved from cache if available. Must be JSON.");
        if ui
            .add_enabled(
                cache_path("skin.msgpack").exists(),
                egui::Button::new("Clear skin cache"),
            )
            .clicked()
            && cache_path("skin.msgpack").exists()
        {
            let _ = safe_delete(&cache_path("skin.msgpack"), Some("cached skin file"));
        }
        ui.separator();

        ui.add(
            egui::Slider::new(&mut misc_settings.big_handle_size, 0.1..=4.0)
                .text("Big Handle Size"),
        );
        ui.add(
            egui::Slider::new(&mut misc_settings.small_handle_size, 0.1..=4.0)
                .text("Small Handle Size"),
        );
        ui.separator();

        ui.add(
            egui::Slider::new(
                &mut misc_settings.hide_far_handles_threshold,
                0..=0x0001_0000,
            )
            .text("Threshold for hiding far handles"),
        );
        ui.add(
            egui::Slider::new(&mut misc_settings.hide_far_handles_distance, 0.0..=65536.0)
                .text("Distance limit for far handles"),
        );
        ui.label("Above the threshold, if the distance between the mouse and the handle is larger than the limit, it is hidden");
        ui.separator();

        ui.add(
            egui::Slider::new(&mut misc_settings.crosshair_size, 0.1..=4.0).text("Crosshair size"),
        );
        ui.separator();

        ui.add(
            egui::Slider::new(&mut misc_settings.scroll_multiplier_line, 0.1..=4.0)
                .text("Scroll multiplier (line unit)"),
        );
        ui.add(
            egui::Slider::new(&mut misc_settings.scroll_multiplier_pixel, 0.1..=4.0)
                .text("Scroll multiplier (pixel unit)"),
        );
        ui.separator();

        egui::ComboBox::from_label("Scroll Mode")
            .selected_text(misc_settings.scroll_mode.to_string())
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut misc_settings.scroll_mode, ScrollMode::Zoom, "Zoom");
                ui.selectable_value(&mut misc_settings.scroll_mode, ScrollMode::Pan, "Pan");
            });
        ui.label("Zoom: scroll to zoom; left-click+drag to pan");
        ui.label("Pan: scroll/left-click+drag to pan; ctrl+scroll to zoom");
        ui.separator();

        ui.add(
            egui::Slider::new(&mut misc_settings.additional_zoom, 0..=4)
                .text("Additional zoom levels"),
        );
        ui.label("Increases the maximum zoom so you can zoom in further");
        ui.separator();

        ui.add(
            egui::Slider::new(&mut misc_settings.autosave_interval, 0..=600)
                .suffix("s")
                .text("Autosave interval"),
        );
        ui.label("Set to 0 to disable autosave");
        ui.separator();

        ui.add(
            egui::Slider::new(&mut misc_settings.notif_duration, 0..=10)
                .suffix("s")
                .text("Notification duration"),
        );
        ui.label("Time before success and info notifications expire. Set to 0 to disable expiry");

        if !invalid && old_settings != **misc_settings {
            misc_settings.save().unwrap();
            if old_settings.skin_url != misc_settings.skin_url
                && cache_path("skin.msgpack").exists()
            {
                let _ = safe_delete(&cache_path("skin.msgpack"), Some("cached skin file"));
            }
        }
    }
}

pub fn on_misc_settings(_trigger: Trigger<OpenMiscSettingsEv>, mut state: ResMut<DockLayout>) {
    open_dock_window(&mut state, MiscSettingsEditor);
}
