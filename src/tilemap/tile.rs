use bevy::{ecs::query::ReadOnlyWorldQuery, prelude::*};
use bevy_mouse_tracking_plugin::MainCamera;

use crate::tilemap::{
    bundle::{Tile, TileBundle},
    settings::TileSettings,
    tile_coord::TileCoord,
    utils::get_map_coords_of_edges,
    zoom::Zoom,
};

pub fn get_shown_tiles(
    q_camera: &Query<(&Camera, &GlobalTransform), impl ReadOnlyWorldQuery>,
    zoom: i8,
    tile_settings: &TileSettings,
) -> Vec<TileCoord> {
    let (camera, transform) = q_camera.single();
    let (c_left, c_top, c_right, c_bottom) = get_map_coords_of_edges(camera, transform);
    let TileCoord {
        x: t_left,
        y: t_top,
        ..
    } = TileCoord::from_world_coords(
        c_left as f64,
        c_top as f64,
        zoom.min(tile_settings.max_tile_zoom),
        tile_settings,
    );
    let TileCoord {
        x: t_right,
        y: t_bottom,
        ..
    } = TileCoord::from_world_coords(
        c_right as f64,
        c_bottom as f64,
        zoom.min(tile_settings.max_tile_zoom),
        tile_settings,
    );

    (t_left - 1..=t_right + 1)
        .flat_map(|ref x| {
            (t_top - 1..=t_bottom + 1)
                .map(|y| TileCoord {
                    x: *x,
                    y,
                    z: zoom.min(tile_settings.max_tile_zoom),
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

#[tracing::instrument(skip_all)]
pub fn show_tiles_sy(
    mut commands: Commands,
    q_camera: Query<(&Camera, &GlobalTransform), (With<MainCamera>, Changed<GlobalTransform>)>,
    mut query: Query<(Entity, &TileCoord), With<Tile>>,
    zoom: Res<Zoom>,
    server: Res<AssetServer>,
    tile_settings: Res<TileSettings>,
) {
    if q_camera.is_empty() {
        return;
    }
    let mut shown_tiles = get_shown_tiles(&q_camera, zoom.0.round() as i8, &tile_settings);

    let (camera, transform) = q_camera.single();
    let (ml, mt, mr, mb) = get_map_coords_of_edges(camera, transform);
    for (entity, tile_coord) in query.iter_mut() {
        if (zoom.0 <= tile_settings.max_tile_zoom as f32 && tile_coord.z > zoom.0.round() as i8)
            || (zoom.0 > tile_settings.max_tile_zoom as f32
                && tile_coord.z != tile_settings.max_tile_zoom)
            || (zoom.0 > tile_settings.max_tile_zoom as f32 && {
                let (tl, tt, tr, tb) = tile_coord.get_edges(&tile_settings);
                tr < ml || tl > mr || tb < mt || tt > mb
            })
            || (tile_coord.z <= (tile_settings.max_tile_zoom - 1)
                && zoom.0 <= (tile_settings.max_tile_zoom as f32)
                && !shown_tiles.contains(tile_coord))
        {
            trace!("Hiding {tile_coord}");
            commands.entity(entity).despawn_recursive();
        } else {
            shown_tiles.retain(|t| t != tile_coord);
            trace!("Showing {tile_coord}");
        }
    }
    for tile_coord in shown_tiles {
        if tile_coord.z <= tile_settings.max_tile_zoom {
            trace!("Loading tile {tile_coord}");
            commands.spawn(TileBundle::from_tile_coord(
                tile_coord,
                &server,
                &tile_settings,
            ));
        }
    }
    server.free_unused_assets();
}
