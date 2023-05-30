use std::{
    fmt::{Display, Formatter},
    path::PathBuf,
};

use bevy::prelude::*;
use lazy_regex::{lazy_regex, Regex};
use once_cell::sync::Lazy;

use crate::{
    misc::data_dir,
    tile::{settings::TileSettings, zoom::Zoom},
};

pub static URL_REPLACER: Lazy<Regex> = lazy_regex!("[<>:/\\|?*\"]");

#[derive(Component, Default, PartialEq, Eq, Copy, Clone, Debug, Hash)]
pub struct TileCoord {
    pub x: i32,
    pub y: i32,
    pub z: i8,
}

impl Display for TileCoord {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}, {}", self.z, self.y, self.x)
    }
}

impl TileCoord {
    #[must_use]
    pub fn from_world_coords(x: f64, y: f64, z: i8, tile_settings: &TileSettings) -> Self {
        Self {
            x: (x / f64::from(Zoom(f32::from(z)).world_size(tile_settings))) as i32,
            y: (y / f64::from(Zoom(f32::from(z)).world_size(tile_settings))) as i32,
            z,
        }
    }

    #[must_use]
    pub fn get_edges(&self, tile_settings: &TileSettings) -> (f32, f32, f32, f32) {
        (
            self.x as f32 * Zoom(f32::from(self.z)).world_size(tile_settings) as f32,
            self.y as f32 * Zoom(f32::from(self.z)).world_size(tile_settings) as f32,
            (self.x + 1) as f32 * Zoom(f32::from(self.z)).world_size(tile_settings) as f32,
            (self.y + 1) as f32 * Zoom(f32::from(self.z)).world_size(tile_settings) as f32,
        )
    }

    #[must_use]
    pub fn url(&self, tile_settings: &TileSettings) -> String {
        let z = 2.0f64.powi(i32::from(tile_settings.max_tile_zoom - self.z));
        let xy = IVec2::new(self.x, self.y).as_dvec2();

        let group = (xy * z / tile_settings.max_zoom_range).floor().as_ivec2();

        let num_in_group = xy * z;

        let mut zzz = String::new();
        let mut i = tile_settings.max_tile_zoom;
        while i > self.z {
            zzz += "z";
            i -= 1;
        }

        if !zzz.is_empty() {
            zzz += "_";
        };
        format!(
            "{}/{}_{}/{zzz}{}_{}.png",
            tile_settings.url, group.x, group.y, num_in_group.x, num_in_group.y
        )
    }

    pub fn path(&self, tile_settings: &TileSettings) -> PathBuf {
        let path = data_dir("tile-cache")
            .join(URL_REPLACER.replace_all(&tile_settings.url, "").as_ref())
            .join(self.z.to_string())
            .join(self.x.to_string());
        let _ = std::fs::create_dir_all(&path);
        path.join(format!("{}.png", self.y))
    }
}
