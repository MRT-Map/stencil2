mod editor;
mod setup;
mod types;

use bevy::asset::AssetPlugin;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use bevy::render::texture::ImageSettings;
use bevy::window::WindowMode;
use bevy_egui::EguiPlugin;
use bevy_mod_picking::DefaultPickingPlugins;
use bevy_mouse_tracking_plugin::prelude::MousePosPlugin;
use bevy_prototype_lyon::prelude::ShapePlugin;
use bevy_web_asset::WebAssetPlugin;

use crate::{
    editor::{
        creating_component::CreateComponentPlugin, cursor::CursorPlugin,
        selecting_component::SelectComponentPlugin, tilemap::RenderingPlugin, ui::UiPlugin,
    },
    setup::SetupPlugin,
};

fn main() {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    App::new()
        .insert_resource(WindowDescriptor {
            title: "Stencil".to_string(),
            mode: if cfg!(debug_assertions) { WindowMode::Windowed } else { WindowMode::BorderlessFullscreen },
            ..default()
        })
        .insert_resource(ImageSettings::default_nearest())
        .add_plugins_with(DefaultPlugins, |group| {
            group.add_before::<AssetPlugin, _>(WebAssetPlugin)
        })
        .add_plugins(DefaultPickingPlugins)
        //.add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(SetupPlugin)
        .add_plugin(MousePosPlugin)
        .add_plugin(EguiPlugin)
        .add_plugin(ShapePlugin)
        .add_plugin(UiPlugin)
        .add_plugin(CursorPlugin)
        .add_plugin(RenderingPlugin)
        .add_plugin(CreateComponentPlugin)
        .add_plugin(SelectComponentPlugin)
        .run();
}
