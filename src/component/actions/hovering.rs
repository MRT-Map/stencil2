use bevy::prelude::*;
use crate::component::actions::rendering::RenderEv;
use crate::ui::panel::dock::PanelDockState;

#[tracing::instrument(skip_all)]
pub fn on_hover_over(
    trigger: Trigger<Pointer<Over>>,
    pickables: Query<(), With<RayCastPickable>>,
    mut commands: Commands,
    panel: Res<PanelDockState>,
) {
    let entity = trigger.entity();
    if !panel.pointer_within_tilemap || !pickables.contains(entity) {
        return;
    }
    debug!(?entity, "Hovering over component");
    commands.entity(entity).insert(HoveredComponent).trigger(RenderEv::default());
}

#[tracing::instrument(skip_all)]
pub fn on_hover_out(
    trigger: Trigger<Pointer<Out>>,
    pickables: Query<(), With<RayCastPickable>>,
    mut commands: Commands,
    panel: Res<PanelDockState>,
) {
    let entity = trigger.entity();
    if !panel.pointer_within_tilemap || !pickables.contains(entity) {
        return;
    }
    debug!(?entity, "Hovering out of component");
    commands.entity(entity).remove::<HoveredComponent>().trigger(RenderEv::default());
}

pub struct HoverComponentPlugin;
impl Plugin for HoverComponentPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_hover_over).add_observer(on_hover_out);
    }
}

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct HoveredComponent;