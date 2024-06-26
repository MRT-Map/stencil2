use std::collections::HashMap;

use bevy::prelude::*;
use hex_color::HexColor;
use serde::{Deserialize, Serialize};

use crate::component::pla2::ComponentType;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct SkinInfo {
    pub size: u32,
    pub font: HashMap<String, Vec<String>>,
    pub background: HexColor,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "layer")]
pub enum AreaStyle {
    #[serde(rename = "fill")]
    Fill {
        colour: Option<HexColor>,
        outline: Option<HexColor>,
        stripe: Option<(u32, u32, u8)>,
    },
    #[serde(rename = "centertext")]
    CenterText {
        colour: HexColor,
        offset: IVec2,
        size: u32,
    },
    #[serde(rename = "bordertext")]
    BorderText {
        colour: HexColor,
        offset: i32,
        size: u32,
    },
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "layer")]
pub enum LineStyle {
    #[serde(rename = "fore")]
    Fore {
        colour: HexColor,
        width: u32,
        dash: Option<UVec2>,
    },
    #[serde(rename = "back")]
    Back {
        colour: HexColor,
        width: u32,
        dash: Option<UVec2>,
    },
    #[serde(rename = "text")]
    Text {
        colour: HexColor,
        arrow_colour: HexColor,
        size: u32,
        offset: i32,
    },
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "layer")]
pub enum PointStyle {
    #[serde(rename = "image")]
    Image { file: String, offset: IVec2 },
    #[serde(rename = "square")]
    Square {
        colour: HexColor,
        outline: Option<HexColor>,
        size: u32,
        width: u32,
    },
    #[serde(rename = "circle")]
    Circle {
        colour: HexColor,
        outline: Option<HexColor>,
        size: u32,
        width: u32,
    },
    #[serde(rename = "text")]
    Text {
        colour: HexColor,
        size: u32,
        offset: IVec2,
        anchor: Option<String>,
    },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum SkinComponent {
    #[serde(rename = "point")]
    Point {
        tags: Vec<String>,
        style: HashMap<String, Vec<PointStyle>>,
    },
    #[serde(rename = "line")]
    Line {
        tags: Vec<String>,
        style: HashMap<String, Vec<LineStyle>>,
    },
    #[serde(rename = "area")]
    Area {
        tags: Vec<String>,
        style: HashMap<String, Vec<AreaStyle>>,
    },
}
impl SkinComponent {
    #[must_use]
    pub const fn get_type(&self) -> ComponentType {
        match self {
            Self::Point { .. } => ComponentType::Point,
            Self::Line { .. } => ComponentType::Line,
            Self::Area { .. } => ComponentType::Area,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Resource)]
pub struct Skin {
    pub info: SkinInfo,
    pub order: Vec<String>,
    pub types: HashMap<String, SkinComponent>,
}
