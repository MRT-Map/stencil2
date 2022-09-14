use bevy::prelude::*;
use bevy_mouse_tracking_plugin::MainCamera;
use iyes_loopless::prelude::*;

use crate::{get_cursor_world_pos, pla::{ComponentBundle, ComponentCoords, CreatedComponent, EditorComponent, SelectedComponent}, ComponentType, EditorState, HoveringOverGui, Skin, DeselectQuery, SelectQuery, CreatedQuery};
use crate::editor::selecting_component::{deselect, select};

#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub fn create_component(
    mut set: ParamSet<(
        CreatedQuery,
        DeselectQuery,
        SelectQuery<With<CreatedComponent>>
    )>,
    mut commands: Commands,
    buttons: Res<Input<MouseButton>>,
    skin: Res<Skin>,
    state: Res<CurrentState<EditorState>>,
    windows: Res<Windows>,
    mut camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    hovering: Res<HoveringOverGui>,
) {
    // TODO check if screen is moved
    if let EditorState::CreatingComponent(type_) = &state.0 {
        let (camera, transform): (&Camera, &GlobalTransform) = camera.single_mut();
        let mouse_pos = if let Some(mp) = get_cursor_world_pos(&windows, camera, transform) {
            mp
        } else {
            return;
        };
        if buttons.just_released(MouseButton::Left) && !hovering.0 {
            if *type_ == ComponentType::Point {
                deselect(&mut commands, &set.p1());
                let mut new_point = ComponentBundle::new(
                    EditorComponent::new(type_.to_owned()),
                    mouse_pos.as_ivec2(),
                );
                new_point.update_shape(&skin);
                commands.spawn_bundle(new_point).insert(SelectedComponent);
                return;
            }
            if set.p0().is_empty() {
                let mut new_comp = ComponentBundle::new(
                    EditorComponent::new(type_.to_owned()),
                    mouse_pos.as_ivec2(),
                );
                new_comp.update_shape(&skin);
                commands.spawn_bundle(new_comp).insert(CreatedComponent);
            } else {
                let mut created_query = set.p0();
                let (data, mut coords, entity): (&EditorComponent, Mut<ComponentCoords>, Entity) =
                    created_query.single_mut();
                match data.get_type(&skin).unwrap() {
                    ComponentType::Line | ComponentType::Area => {
                        coords.0.push(mouse_pos.as_ivec2());
                        commands
                            .entity(entity)
                            .insert_bundle(data.get_shape((*coords).to_owned(), &skin, false));
                    }
                    ComponentType::Point => unreachable!(),
                }
            }
        } else if buttons.just_released(MouseButton::Right) { // or double left-click?
            select(&mut commands, &mut set.p2());
        } else if *type_ != ComponentType::Point && !set.p0().is_empty() {
            let mut created_query = set.p0();
            let (data, coords, entity): (&EditorComponent, Mut<ComponentCoords>, Entity) =
                created_query.single_mut();
            let mut coords = coords.to_owned();
            coords.0.push(mouse_pos.as_ivec2());
            commands
                .entity(entity)
                .insert_bundle(data.get_shape(coords, &skin, false));
        }
    }
}

pub fn clear_created_component(
    mut commands: Commands,
    mut created_query: CreatedQuery,
    skin: Res<Skin>,
    state: Res<CurrentState<EditorState>>,
) {
    if matches!(&state.0, EditorState::CreatingComponent(_)) {
        return;
    }
    for (data, coords, entity) in created_query.iter() {
        commands
            .entity(entity)
            .insert_bundle(data.get_shape(coords.to_owned(), &skin, false))
            .remove::<CreatedComponent>();
    }
}
