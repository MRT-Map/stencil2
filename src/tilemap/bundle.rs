use bevy::{prelude::*, sprite::Anchor};

use crate::tilemap::{tile_coord::TileCoord, zoom::Zoom};

#[derive(Component)]
pub struct Tile;

#[derive(Bundle)]
pub struct TileBundle {
    _t: Tile,
    pub coord: TileCoord,

    #[bundle]
    pub sprite: SpriteBundle,
}

impl TileBundle {
    pub fn from_tile_coord(coord: TileCoord, server: &Res<AssetServer>) -> Self {
        let custom_size = Vec2::splat(Zoom(coord.z as f32).map_size() as f32);
        trace!(coord = coord.to_string(), "Loading tile");
        Self {
            _t: Tile,
            coord,
            sprite: SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(custom_size),
                    anchor: Anchor::TopLeft,
                    ..default()
                },
                texture: server.load(&*coord.url()) as Handle<Image>,
                transform: Transform::from_translation(Vec3::new(
                    coord.x as f32 * Zoom(coord.z as f32).map_size() as f32 - 0.5f32,
                    coord.y as f32 * Zoom(coord.z as f32).map_size() as f32 + 32f32 + 0.5f32,
                    0.0,
                )),
                ..default()
            },
        }
    }
}
