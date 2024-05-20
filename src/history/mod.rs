pub mod history_viewer;

use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use bevy::prelude::*;

use crate::{
    action::Action,
    component::{
        bundle::{
            AreaComponentBundle, EntityCommandsSelectExt, LineComponentBundle,
            PointComponentBundle, SelectedComponent,
        },
        pla2::{ComponentType, EditorCoords, PlaComponent},
        skin::Skin,
    },
    project::events::ProjectAct,
    state::IntoSystemConfigExt,
    ui::panel::status::Status,
};

#[derive(Clone, Debug)]
pub enum HistoryEntry<T = Entity> {
    Component {
        entity: T,
        before: Option<Box<PlaComponent<EditorCoords>>>,
        after: Option<Box<PlaComponent<EditorCoords>>>,
    },
    Namespace {
        namespace: String,
        visible: bool,
    },
}

pub enum HistoryAct {
    NewHistory(Vec<HistoryEntry>),
    Undo,
    Redo,
}
impl HistoryAct {
    #[must_use]
    pub fn one_history(history: HistoryEntry) -> Self {
        Self::NewHistory(vec![history])
    }
}

#[derive(Resource, Default, Debug)]
pub struct History {
    pub undo_stack: Vec<Vec<HistoryEntry<Arc<RwLock<Entity>>>>>,
    pub redo_stack: Vec<Vec<HistoryEntry<Arc<RwLock<Entity>>>>>,
}

#[allow(
    clippy::needless_pass_by_value,
    clippy::cognitive_complexity,
    clippy::implicit_hasher
)]
pub fn history_asy(
    mut commands: Commands,
    mut actions: ParamSet<(EventReader<Action>, EventWriter<Action>)>,
    mut ids: Local<HashMap<Entity, Arc<RwLock<Entity>>>>,
    mut history: ResMut<History>,
    selected_entity: Query<Entity, With<SelectedComponent>>,
    skin: Res<Skin>,
    mut status: ResMut<Status>,
) {
    let selected = selected_entity.get_single().ok();
    let mut send_queue: Vec<Action> = vec![];
    for event in actions.p0().read() {
        if let Some(HistoryAct::NewHistory(histories)) = event.downcast_ref() {
            let histories = histories
                .iter()
                .map(|history| match history {
                    HistoryEntry::Component {
                        before,
                        after,
                        entity: component_id,
                    } => HistoryEntry::Component {
                        before: before.to_owned(),
                        after: after.to_owned(),
                        entity: {
                            let component_id = Arc::clone(
                                ids.entry(*component_id)
                                    .or_insert_with(|| Arc::new(RwLock::new(*component_id))),
                            );
                            debug!(?component_id, "Added entry to undo stack");
                            component_id
                        },
                    },
                    HistoryEntry::Namespace { namespace, visible } => HistoryEntry::Namespace {
                        namespace: namespace.to_owned(),
                        visible: visible.to_owned(),
                    },
                })
                .collect();
            history.redo_stack.clear();
            history.undo_stack.push(histories);
        } else if matches!(event.downcast_ref(), Some(HistoryAct::Undo)) {
            let Some(mut histories) = history.undo_stack.pop() else {
                continue;
            };
            for history in &mut histories {
                match history {
                    HistoryEntry::Component {
                        before,
                        after,
                        entity: component_id,
                    } => {
                        if let Some(before) = before {
                            if after.is_none() {
                                debug!(?component_id, "Undoing deletion");
                                status.0 = format!("Undid deletion of {before}").into();
                                let entity = match before.get_type(&skin) {
                                    ComponentType::Point => commands.spawn(
                                        PointComponentBundle::new((**before).to_owned(), &skin),
                                    ),
                                    ComponentType::Line => commands.spawn(
                                        LineComponentBundle::new((**before).to_owned(), &skin),
                                    ),
                                    ComponentType::Area => commands.spawn(
                                        AreaComponentBundle::new((**before).to_owned(), &skin),
                                    ),
                                }
                                .id();
                                *component_id.write().unwrap() = entity;
                                ids.insert(entity, Arc::clone(component_id));
                            } else {
                                let component_id = component_id.read().unwrap();
                                debug!(?component_id, "Undoing edit");
                                status.0 = format!("Undid edit of {before}").into();
                                commands.entity(*component_id).insert((**before).to_owned());
                                if Some(*component_id) == selected {
                                    commands
                                        .entity(*component_id)
                                        .select_component(&skin, before);
                                } else {
                                    commands
                                        .entity(*component_id)
                                        .component_display(&skin, before);
                                }
                            }
                        } else {
                            let component_id = component_id.read().unwrap();
                            debug!(?component_id, "Undoing creation");
                            status.0 = format!(
                                "Undid creation of {}",
                                after.as_ref().map_or_else(String::new, |a| format!("{a}"))
                            )
                            .into();
                            commands.entity(*component_id).despawn_recursive();
                            ids.remove(&component_id);
                        }
                    }
                    HistoryEntry::Namespace { namespace, visible } => {
                        send_queue.push(if *visible {
                            Action::new(ProjectAct::Hide {
                                ns: namespace.to_owned(),
                                history_invoked: true,
                            })
                        } else {
                            Action::new(ProjectAct::Show {
                                ns: namespace.to_owned(),
                                history_invoked: true,
                            })
                        });
                    }
                }
            }
            history.redo_stack.push(histories);
        } else if matches!(event.downcast_ref(), Some(HistoryAct::Redo)) {
            let Some(mut histories) = history.redo_stack.pop() else {
                continue;
            };
            for history in &mut histories {
                match history {
                    HistoryEntry::Component {
                        before,
                        after,
                        entity: component_id,
                    } => {
                        if let Some(after) = after {
                            debug!(?component_id, "Redoing creation");
                            status.0 = format!("Redid creation of {after}").into();
                            if before.is_none() {
                                let entity = match after.get_type(&skin) {
                                    ComponentType::Point => commands.spawn(
                                        PointComponentBundle::new((**after).to_owned(), &skin),
                                    ),
                                    ComponentType::Line => commands.spawn(
                                        LineComponentBundle::new((**after).to_owned(), &skin),
                                    ),
                                    ComponentType::Area => commands.spawn(
                                        AreaComponentBundle::new((**after).to_owned(), &skin),
                                    ),
                                }
                                .id();
                                *component_id.write().unwrap() = entity;
                                ids.insert(entity, Arc::clone(component_id));
                            } else {
                                let component_id = component_id.read().unwrap();
                                debug!(?component_id, "Redoing edit");
                                status.0 = format!("Redid edit of {after}").into();
                                commands.entity(*component_id).insert((**after).to_owned());
                                if Some(*component_id) == selected {
                                    commands
                                        .entity(*component_id)
                                        .select_component(&skin, after);
                                } else {
                                    commands
                                        .entity(*component_id)
                                        .component_display(&skin, after);
                                }
                            }
                        } else {
                            let component_id = component_id.read().unwrap();
                            debug!(?component_id, "Redoing deletion");
                            status.0 = format!(
                                "Redid deletion of {}",
                                before.as_ref().map_or_else(String::new, |a| format!("{a}"))
                            )
                            .into();
                            commands.entity(*component_id).despawn_recursive();
                            ids.remove(&component_id);
                        }
                    }
                    HistoryEntry::Namespace { namespace, visible } => {
                        send_queue.push(if *visible {
                            Action::new(ProjectAct::Show {
                                ns: namespace.to_owned(),
                                history_invoked: true,
                            })
                        } else {
                            Action::new(ProjectAct::Hide {
                                ns: namespace.to_owned(),
                                history_invoked: true,
                            })
                        });
                    }
                }
            }
            history.undo_stack.push(histories);
        }
    }

    for action in send_queue {
        actions.p1().send(action);
    }
}

pub struct HistoryPlugin;
impl Plugin for HistoryPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<History>()
            .add_systems(Update, history_asy.run_if_not_loading());
    }
}
