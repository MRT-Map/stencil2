use bevy::{app::AppExit, prelude::*};

use crate::{
    component::pla2::PlaComponent,
    info_windows::InfoWindowsEv,
    ui::popup::{Popup, Popups},
};

#[expect(clippy::needless_pass_by_value)]
pub fn on_quit(
    trigger: Trigger<InfoWindowsEv>,
    mut popups: ResMut<Popups>,
    mut exit: EventWriter<AppExit>,
    components: Query<(), With<PlaComponent>>,
    mut commands: Commands,
) {
    match trigger.event() {
        InfoWindowsEv::Quit(false) => {
            if components.is_empty() || cfg!(debug_assertions) {
                commands.trigger(InfoWindowsEv::Quit(true));
            } else {
                popups.add(Popup::base_confirm(
                    "confirm_quit",
                    "Are you sure you want to exit?",
                    "You may have unsaved changes",
                    InfoWindowsEv::Quit(true),
                ));
            }
        }
        InfoWindowsEv::Quit(true) => {
            exit.write(AppExit::Success);
        }
        _ => {}
    }
}
