mod events;
pub mod history_viewer;

use std::{
    fmt::{Display, Formatter},
    path::PathBuf,
    sync::{Arc, RwLock},
};

use bevy::prelude::*;

use crate::component::pla2::PlaComponent;

#[derive(Clone, Debug)]
pub enum HistoryEntry<T = Entity> {
    Component {
        e: T,
        before: Option<Box<PlaComponent>>,
        after: Option<Box<PlaComponent>>,
    },
    Namespace {
        namespace: String,
        action: NamespaceAction,
    },
}
#[derive(Clone, Debug)]
pub enum NamespaceAction {
    Hide,
    Show,
    Create(Option<PathBuf>),
    Delete(Option<PathBuf>),
}

impl<T> Display for HistoryEntry<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Component { before, after, .. } => match (before, after) {
                (Some(before), Some(after)) => {
                    let before_id = before.to_string();
                    let after_id = after.to_string();
                    if before_id == after_id {
                        write!(f, "Edit component data of {before_id}")
                    } else {
                        write!(f, "Edit component data of {before_id}/{after_id}")
                    }
                }
                (Some(before), None) => {
                    write!(f, "Delete {before}")
                }
                (None, Some(after)) => {
                    write!(f, "Create {after}")
                }
                (None, None) => {
                    panic!();
                }
            },
            Self::Namespace { namespace, action } => match action {
                NamespaceAction::Create(_) => write!(f, "Create {namespace}"),
                NamespaceAction::Delete(_) => write!(f, "Delete {namespace}"),
                NamespaceAction::Hide => write!(f, "Hide {namespace}"),
                NamespaceAction::Show => write!(f, "Show {namespace}"),
            },
        }
    }
}

#[derive(Clone, Event)]
pub enum HistoryEv {
    NewHistory(Vec<HistoryEntry>),
    Undo,
    Redo,
}
impl HistoryEv {
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

pub struct HistoryPlugin;
impl Plugin for HistoryPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<History>()
            //.add_systems(Update, events::on_history.run_if_not_loading())
            .add_observer(events::on_history)
            .add_observer(history_viewer::on_history_viewer);
    }
}
