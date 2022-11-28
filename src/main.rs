#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(clippy::type_complexity, clippy::too_many_arguments)]

use std::io::Cursor;

use bevy::{
    asset::AssetPlugin, diagnostic::FrameTimeDiagnosticsPlugin, log::LogPlugin, prelude::*,
    window::WindowMode,
};
use bevy_egui::EguiPlugin;
use bevy_mod_picking::DefaultPickingPlugins;
use bevy_mouse_tracking_plugin::prelude::MousePosPlugin;
use bevy_prototype_lyon::prelude::ShapePlugin;
use bevy_web_asset::WebAssetPlugin;
use tracing::Level;
use tracing_subscriber::{fmt::writer::MakeWriterExt, EnvFilter};
use zip::ZipArchive;

use crate::{
    component_actions::ComponentActionPlugins, component_tools::ComponentToolPlugins,
    cursor::CursorPlugin, info_windows::InfoWindowsPlugin, load_save::LoadSavePlugin,
    misc::data_dir, setup::SetupPlugin, tilemap::RenderingPlugin, ui::UiPlugin,
};

mod component_actions;
mod component_tools;
mod cursor;
mod error_handling;
mod info_windows;
mod load_save;
mod misc;
mod pla2;
mod setup;
mod tilemap;
mod ui;

fn main() {
    std::panic::set_hook(Box::new(error_handling::panic));

    tracing_subscriber::fmt()
        .event_format(tracing_subscriber::fmt::format().compact())
        .with_env_filter(
            EnvFilter::try_new(
                "info,\
            wgpu_core::device=warn,\
            bevy_asset::asset_server=error,\
            surf::middleware::logger::native=off,\
            isahc::handler=error,\
            stencil2=debug",
            )
            .unwrap(),
        )
        .with_writer(
            std::io::stdout
                .with_max_level(Level::DEBUG)
                .and(tracing_appender::rolling::hourly(data_dir("logs"), "log")),
        )
        .init();
    info!("Logger initialised");

    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    let mut zip_file = ZipArchive::new(Cursor::new(include_bytes!(concat!(
        env!("OUT_DIR"),
        "/assets.zip"
    ))))
    .unwrap();
    let dir = data_dir("../build/assets");
    zip_file.extract(&dir).unwrap();

    App::new()
        .add_plugin(WebAssetPlugin {
            asset_plugin: AssetPlugin {
                asset_folder: dir.to_string_lossy().to_string(),
                ..default()
            },
        })
        .add_plugins({
            DefaultPlugins
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        title: "Stencil".to_string(),
                        mode: WindowMode::BorderlessFullscreen,
                        ..default()
                    },
                    ..default()
                })
                .set(ImagePlugin::default_nearest())
                .disable::<LogPlugin>()
                .disable::<AssetPlugin>()
        })
        .add_plugins(DefaultPickingPlugins)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(MousePosPlugin)
        .add_plugin(EguiPlugin)
        .add_plugin(ShapePlugin)
        .add_plugin(SetupPlugin)
        .add_plugin(UiPlugin)
        .add_plugin(CursorPlugin)
        .add_plugin(RenderingPlugin)
        .add_plugins(ComponentToolPlugins)
        .add_plugins(ComponentActionPlugins)
        .add_plugin(LoadSavePlugin)
        .add_plugin(InfoWindowsPlugin)
        .run();
}
