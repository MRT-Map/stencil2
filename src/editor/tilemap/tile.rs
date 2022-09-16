use bevy::prelude::*;
use bevy_mouse_tracking_plugin::MainCamera;

use crate::{
    editor::{
        bundles::tile::{Tile, TileBundle},
        tilemap::utils::get_map_coords_of_edges,
    },
    types::{tile_coord::TileCoord, zoom::Zoom},
};

pub fn get_shown_tiles(
    q_camera: &Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    zoom: i8,
) -> Vec<TileCoord> {
    let (camera, transform): (&Camera, &GlobalTransform) = q_camera.single();
    let (c_left, c_top, c_right, c_bottom) = get_map_coords_of_edges(camera, transform);
    let TileCoord {
        x: t_left,
        y: t_top,
        ..
    } = TileCoord::from_world_coords(c_left as f64, c_top as f64, zoom);
    let TileCoord {
        x: t_right,
        y: t_bottom,
        ..
    } = TileCoord::from_world_coords(c_right as f64, c_bottom as f64, zoom);

    (t_left - 1..=t_right + 1)
        .flat_map(|ref x| {
            (t_top - 1..=t_bottom + 1)
                .map(|y| TileCoord { x: *x, y, z: zoom })
                .collect::<Vec<_>>()
        })
        .collect()
}

pub fn show_tiles(
    mut commands: Commands,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut query: Query<(&mut Visibility, &TileCoord), With<Tile>>,
    zoom: Res<Zoom>,
    server: Res<AssetServer>,
) {
    //if !zoom.is_changed() {return}
    let mut shown_tiles = get_shown_tiles(&q_camera, zoom.0.round() as i8);

    let (camera, transform): (&Camera, &GlobalTransform) = q_camera.single();
    let (ml, mt, mr, mb) = get_map_coords_of_edges(camera, transform);
    for (mut visibility, tile_coord) in query.iter_mut() {
        if (zoom.0 <= 8f32 && tile_coord.z > zoom.0.round() as i8)
            || (zoom.0 > 8f32 && tile_coord.z != 8)
            || (zoom.0 > 8f32 && {
                let (tl, tt, tr, tb) = tile_coord.get_edges();
                tr < ml || tl > mr || tb < mt || tt > mb
            })
            || (tile_coord.z <= 7 && zoom.0 <= 8f32 && !shown_tiles.contains(tile_coord))
        {
            trace!("Hiding {tile_coord}");
            visibility.is_visible = false;
        } else {
            shown_tiles.retain(|t| t != tile_coord);
            trace!("Showing {tile_coord}");
            visibility.is_visible = true;
        }
    }
    for tile_coord in shown_tiles {
        if tile_coord.z <= 8 {
            trace!("Loading tile {tile_coord}");
            commands.spawn_bundle(TileBundle::from_tile_coord(tile_coord, &server));
        }
    }
}