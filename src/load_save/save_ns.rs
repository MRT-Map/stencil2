use std::{collections::HashMap, path::PathBuf, sync::Arc};

use bevy::prelude::*;
use itertools::Itertools;

use crate::{
    load_save::LoadSaveAct,
    misc::Action,
    pla2::component::{EditorCoords, MCCoords, PlaComponent},
    ui::{file_explorer::save_single_dir, popup::Popup},
    EventReader,
};

pub fn save_ns_asy(
    mut actions: EventReader<Action>,
    query: Query<&PlaComponent<EditorCoords>>,
    mut popup: EventWriter<Arc<Popup>>,
) {
    for event in actions.iter() {
        if let Some(LoadSaveAct::Save) = event.downcast_ref() {
            save_single_dir("save_ns1", &mut popup, |a| Box::new(LoadSaveAct::Save1(a)));
        } else if let Some(LoadSaveAct::Save1(Some(dir))) = event.downcast_ref() {
            let comps = query.iter().collect::<Vec<_>>();
            let mut files: HashMap<&String, Vec<PlaComponent<MCCoords>>> = HashMap::new();
            for comp in comps {
                if comp.namespace.is_empty() {
                    popup.send(Popup::base_alert(
                        "save_ns_err",
                        "Empty namespace detected!",
                        format!("It is at {}, {}", comp.nodes[0].0.x, comp.nodes[0].0.y),
                    ));
                    continue;
                }
                files
                    .entry(&comp.namespace)
                    .or_default()
                    .push(comp.to_mc_coords())
            }
            for (ns, comps) in &files {
                info!(?ns, "Saving namespace");
                let mut fp = dir.to_owned();
                fp.push(PathBuf::from(format!("{ns}.pla2.msgpack")));
                std::fs::write(fp, rmp_serde::to_vec_named(comps).unwrap()).unwrap();
            }
            popup.send(Popup::base_alert(
                "save_ns_success",
                "Components saved!",
                format!("Namespaces: {}", files.keys().join(", ")),
            ))
        }
    }
}
