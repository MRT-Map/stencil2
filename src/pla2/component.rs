use std::{collections::HashMap, fmt::Debug};

use bevy::prelude::*;
use bevy_prototype_lyon::{entity::ShapeBundle, prelude::*};
use hex_color::HexColor;
use serde::{Deserialize, Serialize};

use crate::pla2::skin::{AreaStyle, LineStyle, PointStyle, Skin, SkinComponent};

fn hex_to_color(hex: HexColor) -> Color {
    Color::Rgba {
        red: f32::from(hex.r) / 255.0,
        green: f32::from(hex.g) / 255.0,
        blue: f32::from(hex.b) / 255.0,
        alpha: 1.0,
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq, Component)]
pub struct PlaComponent<T: Coords> {
    pub namespace: String,
    pub id: String,
    pub display_name: String,
    pub description: String,
    pub tags: Vec<String>,
    pub layer: f64,
    #[serde(rename = "type")]
    pub ty: String,
    pub nodes: Vec<T>,
    #[serde(skip)]
    pub attributes: HashMap<String, String>,
}

#[allow(dead_code)]
impl<T: Coords> PlaComponent<T> {
    pub fn new(ty: ComponentType) -> Self {
        Self {
            ty: format!(
                "simple{}",
                match ty {
                    ComponentType::Point => "Point",
                    ComponentType::Line => "Line",
                    ComponentType::Area => "Area",
                }
            ),
            ..default()
        }
    }
    pub fn get_type(&self, skin: &Skin) -> Option<ComponentType> {
        Some(skin.types.get(self.ty.as_str())?.get_type())
    }
    pub fn front_colour<'a>(&self, skin: &'a Skin) -> Option<&'a HexColor> {
        let type_layers = skin.types.get(self.ty.as_str())?;
        match type_layers {
            SkinComponent::Point { style, .. } => style_in_max_zoom(style)?
                .iter()
                .filter_map(|style| match style {
                    PointStyle::Circle { colour, .. } | PointStyle::Square { colour, .. } => {
                        Some(colour)
                    }
                    _ => None,
                })
                .last(),
            SkinComponent::Line { style, .. } => style_in_max_zoom(style)?
                .iter()
                .filter_map(|style| match style {
                    LineStyle::Fore { colour, .. } => Some(colour),
                    _ => None,
                })
                .last(),
            SkinComponent::Area { style, .. } => style_in_max_zoom(style)?
                .iter()
                .filter_map(|style| match style {
                    AreaStyle::Fill { colour, .. } => colour.into(),
                    _ => None,
                })
                .last(),
        }
    }
    pub fn back_colour<'a>(&self, skin: &'a Skin) -> Option<&'a HexColor> {
        let type_layers = skin.types.get(self.ty.as_str())?;
        match type_layers {
            SkinComponent::Point { .. } => None,
            SkinComponent::Line { style, .. } => style_in_max_zoom(style)?
                .iter()
                .filter_map(|style| match style {
                    LineStyle::Back { colour, .. } => Some(colour),
                    _ => None,
                })
                .last(),
            SkinComponent::Area { style, .. } => style_in_max_zoom(style)?
                .iter()
                .filter_map(|style| match style {
                    AreaStyle::Fill { outline, .. } => outline.into(),
                    _ => None,
                })
                .last(),
        }
    }
    pub fn weight(&self, skin: &Skin) -> Option<u32> {
        let type_layers = skin.types.get(self.ty.as_str())?;
        match type_layers {
            SkinComponent::Point { .. } => None,
            SkinComponent::Line { style, .. } => style_in_max_zoom(style)?
                .iter()
                .filter_map(|style| match style {
                    LineStyle::Fore { width, .. } => Some(width / 4),
                    _ => None,
                })
                .last(),
            SkinComponent::Area { .. } => Some(2),
        }
    }
}

impl PlaComponent<MCCoords> {
    pub fn to_editor_coords(&self) -> PlaComponent<EditorCoords> {
        PlaComponent {
            namespace: self.namespace.to_owned(),
            id: self.id.to_owned(),
            display_name: self.display_name.to_owned(),
            description: self.description.to_owned(),
            tags: self.tags.to_owned(),
            layer: self.layer,
            ty: self.ty.to_owned(),
            nodes: self.nodes.iter().map(|a| (*a).into()).collect(),
            attributes: self.attributes.to_owned(),
        }
    }
}

impl PlaComponent<EditorCoords> {
    pub fn to_mc_coords(&self) -> PlaComponent<MCCoords> {
        PlaComponent {
            namespace: self.namespace.to_owned(),
            id: self.id.to_owned(),
            display_name: self.display_name.to_owned(),
            description: self.description.to_owned(),
            tags: self.tags.to_owned(),
            layer: self.layer,
            ty: self.ty.to_owned(),
            nodes: self.nodes.iter().map(|a| (*a).into()).collect(),
            attributes: self.attributes.to_owned(),
        }
    }
    pub fn get_shape(&self, skin: &Skin, selected: bool) -> (ShapeBundle, Fill, Stroke) {
        if self.get_type(skin) == Some(ComponentType::Point) {
            return (
                ShapeBundle {
                    path: GeometryBuilder::build_as(&shapes::Rectangle {
                        extents: Vec2::splat(2.0),
                        origin: RectangleOrigin::Center,
                    }),
                    transform: Transform::from_xyz(
                        self.nodes[0].0.x as f32,
                        self.nodes[0].0.y as f32,
                        10.0,
                    ),
                    ..default()
                },
                Fill::color(if selected {
                    Color::YELLOW
                } else if let Some(hex) = self.front_colour(skin) {
                    hex_to_color(*hex)
                } else {
                    Color::WHITE
                }),
                Stroke::color(Color::NONE),
            );
        }
        let options = StrokeOptions::default()
            .with_start_cap(LineCap::Round)
            .with_end_cap(LineCap::Round)
            .with_line_join(LineJoin::Round)
            .with_line_width(self.weight(skin).unwrap_or(2) as f32);
        let path = GeometryBuilder::build_as(&{
            let mut pb = PathBuilder::new();
            for coord in &self.nodes {
                pb.line_to(coord.0.as_vec2());
            }
            if self.get_type(skin) == Some(ComponentType::Area) {
                if let Some(coord) = self.nodes.first() {
                    pb.line_to(coord.0.as_vec2());
                }
            }
            pb.build()
        });
        let (fill, stroke) = if self.get_type(skin) == Some(ComponentType::Area) {
            (
                Fill::color(if selected {
                    *Color::YELLOW.clone().set_a(0.5)
                } else if let Some(hex) = self.front_colour(skin) {
                    *hex_to_color(*hex).set_a(0.25)
                } else {
                    Color::NONE
                }),
                Stroke {
                    color: if selected {
                        Color::YELLOW
                    } else if let Some(hex) = self.back_colour(skin) {
                        hex_to_color(*hex)
                    } else {
                        Color::NONE
                    },
                    options,
                },
            )
        } else {
            (
                Fill::color(Color::NONE),
                Stroke {
                    color: if selected {
                        Color::YELLOW
                    } else if let Some(hex) = self.front_colour(skin) {
                        hex_to_color(*hex)
                    } else {
                        Color::WHITE
                    },
                    options,
                },
            )
        };
        let transform = Transform::from_xyz(0.0, 0.0, {
            let order = skin
                .order
                .iter()
                .enumerate()
                .find(|(_, a)| **a == self.ty)
                .map_or(0, |a| a.0);
            (order as f32).mul_add(f32::EPSILON, self.layer as f32 + 20.0)
        });
        (
            ShapeBundle {
                path,
                transform,
                ..default()
            },
            fill,
            stroke,
        )
    }
}

fn style_in_max_zoom<T>(style: &HashMap<String, Vec<T>>) -> Option<&Vec<T>> {
    Some(
        style
            .iter()
            .map(|(zl, data)| (zl.split(", ").next().unwrap().parse::<u8>().unwrap(), data))
            .find(|(min, _)| *min == 0)?
            .1,
    )
}

pub trait Coords: Debug + Default + Copy + Clone {}

#[derive(Component, Debug, Default, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct MCCoords(pub Vec2);

#[derive(Component, Debug, Default, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EditorCoords(pub IVec2);

impl From<EditorCoords> for MCCoords {
    fn from(c: EditorCoords) -> Self {
        Self(Vec2::new(c.0.x as f32, -c.0.y as f32))
    }
}

impl From<MCCoords> for EditorCoords {
    fn from(c: MCCoords) -> Self {
        Self(IVec2::new(c.0.x as i32, -c.0.y as i32))
    }
}

impl From<IVec2> for EditorCoords {
    fn from(c: IVec2) -> Self {
        Self(c)
    }
}

impl Coords for MCCoords {}

impl Coords for EditorCoords {}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub enum ComponentType {
    #[serde(rename = "point")]
    Point,
    #[serde(rename = "line")]
    Line,
    #[serde(rename = "area")]
    Area,
}
