use bevy::prelude::*;

use crate::{misc::data_path, state::LoadingState, ui::tilemap::settings::TileSettings};

fn v2_0_1() {
    info!("Running compatibility upgrades from v2.0.1");
    if data_path("tile_settings.msgpack").is_dir() {
        let _ = std::fs::remove_dir_all(data_path("tile_settings.msgpack"));
    }
}

fn v2_1_0() {
    info!("Running compatibility upgrades from v2.1.0");
    if let Ok(b) = std::fs::read(data_path("tile_settings.msgpack")) {
        if let Ok(t) = rmp_serde::from_slice::<TileSettings>(&b) {
            let _ = t.save();
        }
        let _ = std::fs::remove_file(data_path("tile_settings.msgpack"));
    }
    let _ = std::fs::remove_dir_all(data_path("tile-cache"));
}

#[allow(clippy::needless_pass_by_value)]
pub fn compat_sy(mut commands: Commands) {
    v2_0_1();
    v2_1_0();

    commands.insert_resource(NextState(Some(LoadingState::Compat.next())));
}
