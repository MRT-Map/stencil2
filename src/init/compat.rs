use bevy::prelude::*;

use crate::{misc::data_file, state::LoadingState};

fn v2_0_1() {
    info!("Running compatibility upgrades from v2.0.1");
    if data_file("tile_settings.msgpack").is_dir() {
        let _ = std::fs::remove_dir_all(data_file("tile_settings.msgpack"));
    }
}

#[allow(clippy::needless_pass_by_value)]
pub fn compat_sy(mut commands: Commands) {
    v2_0_1();

    commands.insert_resource(NextState(Some(LoadingState::Compat.next())));
}