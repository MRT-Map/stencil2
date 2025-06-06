use bevy::prelude::*;

use crate::{
    dirs_paths::data_path,
    state::LoadingState,
    ui::popup::{Popup, Popups},
};

pub fn welcome_sy(mut commands: Commands, mut popups: ResMut<Popups>) {
    if !data_path(".welcome_shown").exists() {
        popups.add(Popup::base_alert(
            "welcome",
            "Welcome to Stencil!",
            "Remember to read our manual at https://github.com/MRT-Map/stencil2/wiki. \n\n\
                If you have any questions not covered in the wiki, or you have found a bug, \
                feel free to ask for help on the MRT Mapping Services server (if you have access), \
                contact __7d (if you know my Discord account), or open an issue on our GitHub page \
                at https://github.com/MRT-Map/stencil2/issues.",
        ));
        let _ = std::fs::write(data_path(".welcome_shown"), "");
    }

    commands.insert_resource(NextState::Pending(LoadingState::Welcome.next()));
}
