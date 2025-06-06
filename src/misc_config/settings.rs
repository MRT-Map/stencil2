use std::sync::LazyLock;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::{
    dirs_paths::data_path,
    file::{load_toml, save_toml_with_header},
    ui::map::mouse_nav::ScrollMode,
};

macro_rules! field {
    ($s:ty, $f:ident, $f2:ident, $i:ident, $t:ty) => {
        #[expect(clippy::allow_attributes)]
        #[allow(clippy::float_cmp)]
        fn $f(v: &$t) -> bool {
            *v == <$s>::default().$i
        }
        fn $f2() -> $t {
            <$s>::default().$i
        }
    };
}
field!(
    MiscSettings,
    skin_url_is_default,
    default_skin_url,
    skin_url,
    String
);
field!(
    MiscSettings,
    big_handle_size_is_default,
    default_big_handle_size,
    big_handle_size,
    f32
);
field!(
    MiscSettings,
    small_handle_size_is_default,
    default_small_handle_size,
    small_handle_size,
    f32
);
field!(
    MiscSettings,
    hide_far_handles_threshold_is_default,
    default_hide_far_handles_threshold,
    hide_far_handles_threshold,
    usize
);
field!(
    MiscSettings,
    hide_far_handles_distance_is_default,
    default_hide_far_handles_distance,
    hide_far_handles_distance,
    f32
);
field!(
    MiscSettings,
    crosshair_size_is_default,
    default_crosshair_size,
    crosshair_size,
    f32
);
field!(
    MiscSettings,
    scroll_multiplier_line_is_default,
    default_scroll_multiplier_line,
    scroll_multiplier_line,
    f32
);
field!(
    MiscSettings,
    scroll_multiplier_pixel_is_default,
    default_scroll_multiplier_pixel,
    scroll_multiplier_pixel,
    f32
);

field!(
    MiscSettings,
    scroll_mode_is_default,
    default_scroll_mode,
    scroll_mode,
    ScrollMode
);
field!(
    MiscSettings,
    additional_zoom_is_default,
    default_additional_zoom,
    additional_zoom,
    i8
);
field!(
    MiscSettings,
    autosave_interval_is_default,
    default_autosave_interval,
    autosave_interval,
    u64
);
field!(
    MiscSettings,
    notif_duration_is_default,
    default_notif_duration,
    notif_duration,
    u64
);

#[derive(Deserialize, Serialize, Clone, PartialEq, Resource)]
pub struct MiscSettings {
    #[serde(
        default = "default_skin_url",
        skip_serializing_if = "skin_url_is_default"
    )]
    pub skin_url: String,
    #[serde(
        default = "default_big_handle_size",
        skip_serializing_if = "big_handle_size_is_default"
    )]
    pub big_handle_size: f32,
    #[serde(
        default = "default_small_handle_size",
        skip_serializing_if = "small_handle_size_is_default"
    )]
    pub small_handle_size: f32,
    #[serde(
        default = "default_hide_far_handles_threshold",
        skip_serializing_if = "hide_far_handles_threshold_is_default"
    )]
    pub hide_far_handles_threshold: usize,
    #[serde(
        default = "default_hide_far_handles_distance",
        skip_serializing_if = "hide_far_handles_distance_is_default"
    )]
    pub hide_far_handles_distance: f32,
    #[serde(
        default = "default_crosshair_size",
        skip_serializing_if = "crosshair_size_is_default"
    )]
    pub crosshair_size: f32,
    #[serde(
        default = "default_scroll_multiplier_line",
        skip_serializing_if = "scroll_multiplier_line_is_default"
    )]
    pub scroll_multiplier_line: f32,
    #[serde(
        default = "default_scroll_multiplier_pixel",
        skip_serializing_if = "scroll_multiplier_pixel_is_default"
    )]
    pub scroll_multiplier_pixel: f32,
    #[serde(
        default = "default_scroll_mode",
        skip_serializing_if = "scroll_mode_is_default"
    )]
    pub scroll_mode: ScrollMode,
    #[serde(
        default = "default_additional_zoom",
        skip_serializing_if = "additional_zoom_is_default"
    )]
    pub additional_zoom: i8,
    #[serde(
        default = "default_autosave_interval",
        skip_serializing_if = "autosave_interval_is_default"
    )]
    pub autosave_interval: u64,
    #[serde(
        default = "default_notif_duration",
        skip_serializing_if = "notif_duration_is_default"
    )]
    pub notif_duration: u64,
}

impl Default for MiscSettings {
    fn default() -> Self {
        Self {
            skin_url: "https://github.com/MRT-Map/tile-renderer/releases/latest/download/default.nofontfiles.skin.json".into(),
            big_handle_size: 1.0,
            small_handle_size: 0.5,
            hide_far_handles_threshold: 50,
            hide_far_handles_distance: 10000.0,
            crosshair_size: 1.0,
            scroll_multiplier_line: 1.0,
            scroll_multiplier_pixel: 1.0,
            scroll_mode: ScrollMode::default(),
            additional_zoom: 3,
            autosave_interval: 60,
            notif_duration: 2,
        }
    }
}

impl MiscSettings {
    pub fn load() -> Self {
        if !data_path("misc_settings.toml").exists() {
            let s = Self::default();
            let _ = s.save();
            return s;
        }
        match load_toml::<Self>(&data_path("misc_settings.toml"), Some("misc settings")) {
            Ok(mut str) => {
                info!("Found misc settings file");
                if str.skin_url == "https://raw.githubusercontent.com/MRT-Map/tile-renderer/main/renderer/skins/default.json" {
                    str.skin_url = Self::default().skin_url;
                }
                str
            }
            Err(e) => {
                info!("Couldn't open or parse misc settings file: {e:?}");

                Self::default()
            }
        }
    }
    pub fn save(&self) -> eyre::Result<()> {
        save_toml_with_header(self, &data_path("misc_settings.toml"), "# Documentation is at https://github.com/MRT-Map/stencil2/wiki/Advanced-Topics#misc_settingstoml", Some("misc settings"))
    }
}

pub static INIT_MISC_SETTINGS: LazyLock<MiscSettings> = LazyLock::new(MiscSettings::load);
