use std::{
    collections::HashMap,
    fmt::{Debug, Display, Formatter, Write},
    path::{Path, PathBuf},
    sync::Arc,
};

use eyre::{ContextCompat, Report, Result, eyre};
use serde::{Deserialize, Serialize, de::DeserializeOwned};

use crate::{
    App,
    map::MapWindow,
    project::{Project, skin::SkinType},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(bound = "T: Serialize + DeserializeOwned")]
pub enum PlaNodeBase<T: Debug + Clone + Copy + PartialEq + Eq> {
    Line {
        label: Option<u8>,
        coord: T,
    },
    QuadraticBezier {
        label: Option<u8>,
        ctrl: T,
        coord: T,
    },
    CubicBezier {
        label: Option<u8>,
        ctrl1: T,
        ctrl2: T,
        coord: T,
    },
}
pub type PlaNode = PlaNodeBase<geo::Coord<i32>>;
pub type PlaNodeScreen = PlaNodeBase<egui::Pos2>;
impl PlaNode {
    pub const fn label(self) -> Option<u8> {
        match self {
            Self::Line { label, .. }
            | Self::QuadraticBezier { label, .. }
            | Self::CubicBezier { label, .. } => label,
        }
    }
    pub fn to_screen(
        self,
        app: &App,
        map_window: &MapWindow,
        map_centre: egui::Pos2,
    ) -> PlaNodeScreen {
        let world_to_screen = |coord: geo::Coord<i32>| {
            map_window.world_to_screen(
                app,
                map_centre,
                geo::coord! {
                    x: coord.x as f32,
                    y: coord.y as f32,
                },
            )
        };
        match self {
            Self::Line { coord, label } => PlaNodeScreen::Line {
                coord: world_to_screen(coord),
                label,
            },
            Self::QuadraticBezier { ctrl, coord, label } => PlaNodeBase::QuadraticBezier {
                ctrl: world_to_screen(ctrl),
                coord: world_to_screen(coord),
                label,
            },
            Self::CubicBezier {
                ctrl1,
                ctrl2,
                coord,
                label,
            } => PlaNodeBase::CubicBezier {
                ctrl1: world_to_screen(ctrl1),
                ctrl2: world_to_screen(ctrl2),
                coord: world_to_screen(coord),
                label,
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PlaComponent {
    pub namespace: String,
    pub id: String,
    pub ty: Arc<SkinType>,
    pub display_name: String,
    pub layer: f32,
    pub nodes: Vec<PlaNode>,
    pub misc: HashMap<String, toml::Value>,
}

impl Display for PlaComponent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.namespace, self.id)?;
        if !self.display_name.is_empty() {
            write!(f, " ({})", self.display_name)?;
        }
        Ok(())
    }
}

impl PlaComponent {
    pub fn path(&self, root: &Path) -> PathBuf {
        root.join(&self.namespace).join(format!("{}.pla3", self.id))
    }
    pub fn load_from_string(
        s: &str,
        namespace: String,
        id: String,
        project: &Project,
    ) -> Result<(Self, Option<Report>)> {
        fn get_coord(split: &[&str], i: usize) -> Result<geo::Coord<i32>> {
            let (x, y) = (split[i], split[i + 1]);
            Ok(geo::Coord::from((x.parse()?, y.parse()?)))
        }
        fn get_label(split: &[&str], i: usize) -> Result<Option<u8>> {
            let Some(label) = split.get(i) else {
                return Ok(None);
            };
            let Some(label) = label.strip_suffix("#") else {
                return Err(eyre!("`{label}` does not start with #"));
            };
            label.parse::<u8>().map(Some).map_err(Into::into)
        }

        let mut unknown_type_error = None;
        let (nodes_str, attrs_str) = s
            .split_once("\n---\n")
            .wrap_err(format!("`---` not found in: {s}"))?;

        let nodes = nodes_str
            .split('\n')
            .map(|node_str| {
                let split = node_str.split(' ').collect::<Vec<_>>();
                match split.len() {
                    2 | 3 => Ok(Some(PlaNode::Line {
                        coord: get_coord(&split, 0)?,
                        label: get_label(&split, 2)?,
                    })),
                    4 | 5 => Ok(Some(PlaNode::QuadraticBezier {
                        ctrl: get_coord(&split, 0)?,
                        coord: get_coord(&split, 2)?,
                        label: get_label(&split, 4)?,
                    })),
                    6 | 7 => Ok(Some(PlaNode::CubicBezier {
                        ctrl1: get_coord(&split, 0)?,
                        ctrl2: get_coord(&split, 2)?,
                        coord: get_coord(&split, 4)?,
                        label: get_label(&split, 6)?,
                    })),
                    len => Err(eyre!("`{node_str}` has invalid split length {len}")),
                }
            })
            .filter_map(Result::transpose)
            .collect::<Result<Vec<_>>>()?;

        if !matches!(nodes.first(), Some(PlaNode::Line { .. })) {
            return Err(eyre!(
                "First node must exist and not be a curve (Got {:?})",
                nodes.first()
            ));
        }

        let mut display_name = String::new();
        let mut layer = 0.0f32;
        let mut skin_component = Arc::clone(if nodes.len() == 1 {
            project.skin().unwrap().get_type("simplePoint").unwrap()
        } else {
            project.skin().unwrap().get_type("simpleLine").unwrap()
        });
        let mut misc = HashMap::<String, toml::Value>::new();
        for (k, v) in toml::from_str::<toml::Table>(attrs_str)? {
            match &*k {
                "display_name" => {
                    v.as_str()
                        .wrap_err(format!("`{v}` not string"))?
                        .clone_into(&mut display_name);
                }
                "layer" => {
                    layer = v
                        .as_float()
                        .map(|a| a as f32)
                        .or_else(|| v.as_integer().map(|a| a as f32))
                        .wrap_err(format!("`{v}` not number"))?;
                }
                "type" => {
                    if let Some(s) = project
                        .skin()
                        .unwrap()
                        .get_type(v.as_str().wrap_err(format!("`{v}` not string"))?)
                    {
                        skin_component = Arc::clone(s);
                    } else {
                        unknown_type_error = Some(eyre!(
                            "Unknown skin type for component {namespace}-{id}: {v}"
                        ));
                    }
                }
                _ => {
                    misc.insert(k, v);
                }
            }
        }

        Ok((
            Self {
                namespace,
                id,
                ty: skin_component,
                display_name,
                layer,
                nodes,
                misc,
            },
            unknown_type_error,
        ))
    }
    pub fn save_to_string(&self) -> Result<String> {
        let mut out = String::new();

        for node in &self.nodes {
            match node {
                PlaNode::Line { coord, .. } => write!(out, "{} {}", coord.x, coord.y)?,
                PlaNode::QuadraticBezier { ctrl, coord, .. } => {
                    write!(out, "{} {} {} {}", ctrl.x, ctrl.y, coord.x, coord.y)?;
                }
                PlaNode::CubicBezier {
                    ctrl1,
                    ctrl2,
                    coord,
                    ..
                } => write!(
                    out,
                    "{} {} {} {} {} {}",
                    ctrl1.x, ctrl1.y, ctrl2.x, ctrl2.y, coord.x, coord.y
                )?,
            }
            if let Some(label) = node.label() {
                writeln!(out, " #{label}")?;
            } else {
                out += "\n";
            }
        }
        out += "---\n";

        let attrs = self
            .misc
            .clone()
            .into_iter()
            .chain([
                ("display_name".into(), self.display_name.clone().into()),
                ("layer".into(), self.layer.into()),
                ("type".into(), self.ty.name().as_str().into()),
            ])
            .collect::<toml::Table>();
        out += &toml::to_string_pretty(&attrs)?;
        Ok(out)
    }
    pub fn bounding_rect(&self) -> geo::Rect<f32> {
        let mut x_min = f32::MAX;
        let mut x_max = f32::MIN;
        let mut y_min = f32::MAX;
        let mut y_max = f32::MIN;

        let mut cmp = |coord: geo::Coord<i32>| {
            if (coord.x as f32) < x_min {
                x_min = coord.x as f32
            }
            if (coord.x as f32) > x_max {
                x_max = coord.x as f32
            }
            if (coord.y as f32) < y_min {
                y_min = coord.y as f32
            }
            if (coord.y as f32) > y_max {
                y_max = coord.y as f32
            }
        };

        for node in &self.nodes {
            match node {
                PlaNode::Line { coord, .. } => cmp(*coord),
                PlaNode::QuadraticBezier { ctrl, coord, .. } => {
                    cmp(*ctrl);
                    cmp(*coord);
                }
                PlaNode::CubicBezier {
                    ctrl1,
                    ctrl2,
                    coord,
                    ..
                } => {
                    cmp(*ctrl1);
                    cmp(*ctrl2);
                    cmp(*coord);
                }
            }
        }

        geo::Rect::new(
            geo::coord! {x: x_min, y: y_min},
            geo::coord! {x: x_max, y: y_max},
        )
    }
}
