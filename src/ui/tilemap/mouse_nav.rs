use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
    window::PrimaryWindow,
};
use bevy_egui::EguiContexts;
use bevy_mouse_tracking::{MainCamera, MousePos, MousePosWorld};

use crate::{
    misc_config::settings::MiscSettings,
    tile::{
        utils::{get_map_width_height, get_window_width_height},
        zoom::Zoom,
    },
    ui::{
        panel::dock::{within_tilemap, PanelDockState},
        tilemap::settings::TileSettings,
    },
};

#[tracing::instrument(skip_all)]
pub fn mouse_drag_sy(
    buttons: Res<ButtonInput<MouseButton>>,
    mut mouse_origin_pos: Local<Option<MousePos>>,
    mut camera_origin_pos: Local<Option<Vec2>>,
    mouse_pos: Res<MousePos>,
    mut camera: Query<(&Camera, &mut Transform), With<MainCamera>>,
    windows: Query<(Entity, &Window, Option<&PrimaryWindow>)>,
    mut ctx: EguiContexts,
    panel: Res<PanelDockState>,
) {
    if !within_tilemap(&mut ctx, &panel) {
        return;
    }
    let (camera, mut transform) = camera.single_mut();
    if buttons.pressed(MouseButton::Left) && !ctx.ctx_mut().is_using_pointer() {
        if let Some(origin_pos) = *mouse_origin_pos {
            if !mouse_pos.is_changed() {
                return;
            }
            let Some(win_wh) = get_window_width_height(&windows, camera) else {
                return;
            };
            let map_wh = get_map_width_height(camera, &transform);

            let d = map_wh / win_wh * (**mouse_pos - *origin_pos);
            trace!("Mouse moved {d:?} from origin");
            transform.translation.x = camera_origin_pos.unwrap().x - d.x;
            transform.translation.y = camera_origin_pos.unwrap().y + d.y;
        } else {
            *mouse_origin_pos = Some(*mouse_pos.into_inner());
            *camera_origin_pos = Some(transform.translation.truncate());
        }
    } else {
        *mouse_origin_pos = None;
        *camera_origin_pos = None;
    }
}

#[tracing::instrument(skip_all)]
pub fn mouse_zoom_sy(
    mut scroll_evr: EventReader<MouseWheel>,
    mut camera: Query<(&mut OrthographicProjection, &mut Transform), With<MainCamera>>,
    mut zoom: ResMut<Zoom>,
    mouse_pos_world: Query<&MousePosWorld>,
    tile_settings: Res<TileSettings>,
    mut ctx: EguiContexts,
    panel: Res<PanelDockState>,
    misc_settings: Res<MiscSettings>,
) {
    if !within_tilemap(&mut ctx, &panel) {
        return;
    }
    let (mut ort_proj, mut transform) = camera.single_mut();
    for ev in scroll_evr.read() {
        let u = match ev.unit {
            MouseScrollUnit::Line => ev.y * 0.125 * misc_settings.scroll_multiplier_line,
            MouseScrollUnit::Pixel => ev.y * 0.0125 * misc_settings.scroll_multiplier_pixel,
        };
        if 1.0 <= (zoom.0 + u)
            && (zoom.0 + u)
                <= f32::from(
                    tile_settings.basemaps[0].max_tile_zoom + misc_settings.additional_zoom,
                )
        {
            let orig = transform.translation.xy();
            let orig_scale = ort_proj.scale;
            let orig_mouse_pos = mouse_pos_world.single();
            zoom.0 += u;
            trace!("Zoom changed from {orig_scale} to {}", zoom.0);

            ort_proj.scale =
                ((f32::from(tile_settings.basemaps[0].max_tile_zoom) - 1.0) - zoom.0).exp2();

            let d = (orig_mouse_pos.xy() - orig) * (ort_proj.scale / orig_scale);
            let new_mouse_pos = mouse_pos_world.single();
            trace!("View moved by {d:?}");
            transform.translation.x = new_mouse_pos.x - d.x;
            transform.translation.y = new_mouse_pos.y - d.y;
        }
    }
}
