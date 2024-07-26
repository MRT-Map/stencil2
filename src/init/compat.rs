use bevy::prelude::*;
use serde::Deserialize;
use serde_json::Value;
use toml::Table;

use crate::{
    component::skin::Skin,
    dirs_paths::{cache_path, data_path},
    file::{load_msgpack, safe_delete},
    init::load_skin::Step,
    state::LoadingState,
    ui::tilemap::settings::TileSettings,
};

fn v2_0_1() {
    info!("Running compatibility upgrades from v2.0.1");
    if data_path("tile_settings.msgpack").is_dir() {
        let _ = safe_delete(&data_path("tile_settings.msgpack"), None);
    }
}

fn v2_1_0() {
    info!("Running compatibility upgrades from v2.1.0");
    if let Ok(b) = std::fs::read(data_path("tile_settings.msgpack")) {
        if let Ok(t) = rmp_serde::from_slice::<TileSettings>(&b) {
            let _ = t.save();
        }
        let _ = safe_delete(&data_path("tile_settings.msgpack"), None);
    }
    let _ = safe_delete(&data_path("tile-cache"), None);
}

fn v2_2_0() {
    info!("Running compatibility upgrades from v2.2.0");
    if let Ok(b) = std::fs::read_to_string(data_path("tile_settings.toml")) {
        if let Ok(t) = toml::from_str::<Table>(&b) {
            let mut new = TileSettings::default();
            if let Some(v) = t.get("init_zoom").and_then(toml::Value::as_float) {
                new.init_zoom = v as f32;
            }
            if let Some(v) = t.get("max_get_requests").and_then(toml::Value::as_integer) {
                new.max_get_requests = v as usize;
            }
            if let Some(v) = t
                .get("clear_cache_on_startup")
                .and_then(toml::Value::as_bool)
            {
                new.clear_cache_on_startup = v;
            }
            if let Some(v) = t.get("url").and_then(|a| a.as_str()) {
                new.basemaps[0].url = v.into();
            }
            if let Some(v) = t.get("max_tile_zoom").and_then(toml::Value::as_integer) {
                new.basemaps[0].max_tile_zoom = v as i8;
            }
            if let Some(v) = t.get("max_zoom_range").and_then(toml::Value::as_float) {
                new.basemaps[0].max_zoom_range = v;
            }

            let _ = new.save();
        }
    }
}

fn v2_2_2() {
    info!("Running compatibility upgrades from v2.2.2");
    if cache_path("skin.msgpack").exists() {
        let _ = safe_delete(&cache_path("skin.msgpack"), None);
    }

    #[allow(clippy::items_after_statements)]
    #[derive(Deserialize, Default)]
    pub struct GenericSkin {
        #[serde(default)]
        version: u8,
    }

    if let Ok(b) = std::fs::read(cache_path("skin.msgpack")) {
        if let Ok(v) = rmp_serde::from_slice::<GenericSkin>(&b) {
            if v.version < 2 {
                let _ = safe_delete(&cache_path("skin.msgpack"), None);
            }
        }
    }
}

#[allow(clippy::needless_pass_by_value)]
pub fn compat_sy(mut commands: Commands) {
    v2_0_1();
    v2_1_0();
    v2_2_0();
    v2_2_2();

    commands.insert_resource(NextState::Pending(LoadingState::Compat.next()));
}
