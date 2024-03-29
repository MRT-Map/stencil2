use bevy::app::PluginGroupBuilder;

use crate::PluginGroup;

pub mod creating;
pub mod deleting;
pub mod node_editing;

pub struct ComponentToolPlugins;

impl PluginGroup for ComponentToolPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(creating::CreateComponentPlugin)
            .add(deleting::DeleteComponentPlugin)
            .add(node_editing::EditNodePlugin)
    }
}
