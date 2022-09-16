use bevy::{ecs::query::WorldQuery, prelude::*};
use bevy_mod_picking::{HoverEvent, PickingEvent};
use bevy_prototype_lyon::entity::ShapeBundle;
use iyes_loopless::prelude::*;

use crate::{
    editor::{
        bundles::component::{CreatedComponent, EditorComponent, SelectedComponent},
        ui::HoveringOverGui,
    },
    types::{
        pla::ComponentCoords, skin::Skin, DeselectQuery, DetectMouseMoveOnClick,
        DetectMouseMoveOnClickExt, EditorState, SelectQuery,
    },
};

#[derive(Default)]
pub struct HoveringOverComponent(pub bool);

#[allow(clippy::too_many_arguments)]
pub fn selector(
    state: Res<CurrentState<EditorState>>,
    mut events: EventReader<PickingEvent>,
    mut commands: Commands,
    deselect_query: DeselectQuery,
    buttons: Res<Input<MouseButton>>,
    hovering_over_gui: Res<HoveringOverGui>,
    mut hovering_over_comp: ResMut<HoveringOverComponent>,
    mut selected_entity: Local<Option<Entity>>,
    mut mm_detector: DetectMouseMoveOnClick,
) {
    if matches!(&state.0, EditorState::CreatingComponent(_)) {
        return;
    }
    for event in events.iter() {
        if let PickingEvent::Clicked(e) = event {
            if !hovering_over_gui.0 {
                debug!(?e, "Click detected");
                *selected_entity = Some(*e);
                *mm_detector.0 = Some(*mm_detector.1)
            }
        } else if let PickingEvent::Hover(e) = event {
            debug!("Hover detected");
            hovering_over_comp.0 = match e {
                HoverEvent::JustLeft(_) => false,
                HoverEvent::JustEntered(_) => true,
            };
        }
    }
    if buttons.just_released(MouseButton::Left) && !mm_detector.handle_release() {
        if let Some(selected_entity) = *selected_entity {
            select_entity(&mut commands, &deselect_query, &selected_entity)
        } else {
            info!("Selected nothing, deselecting");
            deselect(&mut commands, &deselect_query)
        }
        *selected_entity = None;
    }
}

pub fn highlight_selected(
    state: Res<CurrentState<EditorState>>,
    mut commands: Commands,
    query: Query<(&EditorComponent, &ComponentCoords, Entity), With<SelectedComponent>>,
    skin: Res<Skin>,
) {
    if matches!(&state.0, EditorState::CreatingComponent(_)) {
        return;
    }
    for (data, coords, entity) in query.iter() {
        trace!(?entity, "Highlighting selected component");
        commands
            .entity(entity)
            .insert_bundle(data.get_shape(coords.to_owned(), &skin, true));
    }
}

pub fn deselect(commands: &mut Commands, (selected_query, skin): &DeselectQuery) {
    for (data, coords, entity) in selected_query.iter() {
        debug!(?entity, "Deselecting component");
        commands
            .entity(entity)
            .remove::<SelectedComponent>()
            .remove_bundle::<ShapeBundle>()
            .insert_bundle(data.get_shape(coords.to_owned(), skin, false));
    }
}

pub fn select_entity(commands: &mut Commands, deselect_query: &DeselectQuery, entity: &Entity) {
    info!(?entity, "Selecting entity");
    deselect(commands, deselect_query);
    commands.entity(*entity).insert(SelectedComponent);
}

pub fn select_query(commands: &mut Commands, set: &mut SelectQuery<impl WorldQuery>) {
    if !set.p1().is_empty() {
        deselect(commands, &set.p0())
    }
    let query = set.p1();
    for entity in query.iter() {
        debug!(?entity, "Selecting entity");
        commands
            .entity(entity)
            .remove::<CreatedComponent>()
            .insert(SelectedComponent);
    }
}

pub struct SelectComponentPlugin;
impl Plugin for SelectComponentPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<HoveringOverComponent>()
            .add_system_set(
                ConditionSet::new()
                    .run_not_in_state(EditorState::Loading)
                    .after("highlight_selected")
                    .with_system(selector)
                    .into(),
            )
            .add_system_set(
                ConditionSet::new()
                    .label("highlight_selected")
                    .run_not_in_state(EditorState::Loading)
                    .with_system(highlight_selected)
                    .into(),
            );
    }
}