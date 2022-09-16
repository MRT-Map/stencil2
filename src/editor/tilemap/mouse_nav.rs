use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
};
use bevy_mouse_tracking_plugin::{MainCamera, MousePos};

use crate::{
    editor::{
        cursor::get_cursor_world_pos,
        tilemap::utils::{get_map_width_height, get_window_width_height},
    },
    types::zoom::Zoom,
};

pub fn mouse_drag(
    buttons: Res<Input<MouseButton>>,
    mut mouse_origin_pos: Local<Option<MousePos>>,
    mut camera_origin_pos: Local<Option<Vec2>>,
    mouse_pos: Res<MousePos>,
    mut camera: Query<(&Camera, &mut GlobalTransform), With<MainCamera>>,
    windows: Res<Windows>,
) {
    let (camera, mut transform): (&Camera, Mut<GlobalTransform>) = camera.single_mut();
    if buttons.pressed(MouseButton::Left) {
        if let Some(origin_pos) = *mouse_origin_pos {
            if !mouse_pos.is_changed() {
                return;
            }
            let win_wh = if let Some(win_wh) = get_window_width_height(&windows, camera) {
                win_wh
            } else {
                return;
            };
            let map_wh = get_map_width_height(camera, &transform);

            let dx = map_wh.x / win_wh.x * (mouse_pos.x - origin_pos.x);
            let dy = map_wh.y / win_wh.y * (mouse_pos.y - origin_pos.y);
            trace!("Mouse moved {dx}, {dy} from origin");
            transform.translation_mut().x = camera_origin_pos.unwrap().x - dx;
            transform.translation_mut().y = camera_origin_pos.unwrap().y - dy;
        } else {
            *mouse_origin_pos = Some(*mouse_pos.into_inner());
            *camera_origin_pos = Some(transform.translation().truncate());
        }
    } else {
        *mouse_origin_pos = None;
        *camera_origin_pos = None;
    }
}

pub fn mouse_zoom(
    mut scroll_evr: EventReader<MouseWheel>,
    mut camera: Query<
        (&Camera, &mut OrthographicProjection, &mut GlobalTransform),
        With<MainCamera>,
    >,
    mut zoom: ResMut<Zoom>,
    windows: Res<Windows>,
) {
    let (camera, mut ort_proj, mut transform): (
        &Camera,
        Mut<OrthographicProjection>,
        Mut<GlobalTransform>,
    ) = camera.single_mut();
    for ev in scroll_evr.iter() {
        let u = match ev.unit {
            MouseScrollUnit::Line => ev.y * 0.125,
            MouseScrollUnit::Pixel => ev.y * 0.0125,
        };
        if 1.0 <= (zoom.0 + u) && (zoom.0 + u) <= 11.0 {
            let orig_x = transform.translation().x;
            let orig_y = transform.translation().y;
            let orig_scale = ort_proj.scale;
            let orig_mouse_pos =
                if let Some(mp) = get_cursor_world_pos(&windows, camera, &transform) {
                    mp
                } else {
                    return;
                };
            zoom.0 += u;
            trace!("Zoom changed from {orig_scale} to {{zoom.0}}");

            ort_proj.scale = 2f32.powf(7.0 - zoom.0);

            let dx = (orig_mouse_pos.x - orig_x) * (ort_proj.scale / orig_scale);
            let dy = (orig_mouse_pos.y - orig_y) * (ort_proj.scale / orig_scale);
            let new_mouse_pos = if let Some(mp) = get_cursor_world_pos(&windows, camera, &transform)
            {
                mp
            } else {
                return;
            };
            trace!("View moved by {dx}, {dy}");
            transform.translation_mut().x = new_mouse_pos.x - dx;
            transform.translation_mut().y = new_mouse_pos.y - dy;

            /*
            var mousePos = {x: mouseEvent.offsetX, y: mouseEvent.offsetY};
            var mouseGridPos = plus(multiply(mousePos, scale), gridPos); orig_mouse_pos
            var delta = mouseEvent.deltaY; u
            zoom += delta;
            zoom = Math.min(zoom, 3000);
            zoom = Math.max(zoom, -1000);
            scale = Math.pow(2,(zoom / 1000));
            gridPos = minus(mouseGridPos, multiply(mousePos, scale));
            */
        }
    }
}