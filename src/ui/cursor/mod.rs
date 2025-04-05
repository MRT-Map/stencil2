use bevy::{prelude::*, sprite::Anchor};
use bevy_egui::{egui, EguiContexts};

use crate::{
    component::actions::hovering::HoveredComponent,
    init::load_assets::ImageAssets,
    misc_config::settings::MiscSettings,
    state::{EditorState, IntoSystemConfigExt},
    tile::zoom::Zoom,
    ui::{
        cursor::mouse_pos::{MousePos, MousePosWorld},
        panel::dock::{PanelDockState},
        tilemap::settings::TileSettings,
        UiSchedule, UiSet,
    },
};
use crate::ui::cursor::mouse_events::{on_emit_click2_down, on_emit_click2_up};

pub mod mouse_events;
pub mod mouse_pos;

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct Crosshair;

#[tracing::instrument(skip_all)]
pub fn crosshair_sy(
    mut commands: Commands,
    state: Option<Res<State<EditorState>>>,
    mut ch: Query<(Entity, &mut Transform, &mut Sprite), With<Crosshair>>,
    images: Res<ImageAssets>,
    zoom: Res<Zoom>,
    mouse_pos_world: Res<MousePosWorld>,
    tile_settings: Res<TileSettings>,
    panel: Res<PanelDockState>,
    misc_settings: Res<MiscSettings>,
) {
    if let Some(state) = state {
        if state.component_type().is_none()
            || (state.component_type().is_some() && !panel.pointer_within_tilemap)
        {
            for (e, _, _) in ch.iter() {
                debug!("Despawning crosshair");
                commands.entity(e).despawn_recursive();
            }
            return;
        }
    } else {
        return;
    }
    let translation = mouse_pos_world.round();
    let new_transform = Transform::from_translation(translation.extend(100.0));
    let new_custom_size = Some(Vec2::splat(
        (f32::from(tile_settings.basemaps[0].max_tile_zoom) - zoom.0).exp2()
            * 16f32
            * misc_settings.crosshair_size,
    ));
    if ch.is_empty() {
        debug!("Spawning crosshair");
        commands
            .spawn((
                Sprite {
                    custom_size: new_custom_size,
                    anchor: Anchor::Center,
                    image: images.crosshair.clone(),
                    ..default()
                },
                new_transform,
            ))
            .insert(Crosshair);
    } else {
        trace!("Updating crosshair location");
        let (_, mut transform, mut sprite) = ch.single_mut();
        *transform = new_transform;
        sprite.custom_size = new_custom_size;
    }
}

#[tracing::instrument(skip_all)]
pub fn cursor_icon_sy(
    buttons: Res<ButtonInput<MouseButton>>,
    mut windows: Query<(Entity, &mut Window)>,
    mut ctx: EguiContexts,
    state: Option<Res<State<EditorState>>>,
    hovered_comp: Query<(), With<HoveredComponent>>,
    panel: Res<PanelDockState>,
) {
    let state = state.map_or(EditorState::Loading, |state| **state);

    for (e, mut window) in &mut windows {
        window.cursor_options.visible = state.component_type().is_none() || !panel.pointer_within_tilemap;
        if !panel.pointer_within_tilemap {
            continue;
        }

        ctx.ctx_for_entity_mut(e).set_cursor_icon(match state {
            EditorState::Loading => egui::CursorIcon::Wait,
            EditorState::Idle | EditorState::DeletingComponent | EditorState::EditingNodes => {
                if buttons.pressed(MouseButton::Left) {
                    egui::CursorIcon::Grabbing
                } else if !hovered_comp.is_empty() {
                    egui::CursorIcon::PointingHand
                } else {
                    egui::CursorIcon::Grab
                }
            }
            EditorState::CreatingLine | EditorState::CreatingArea | EditorState::CreatingPoint => {
                egui::CursorIcon::Cell
            }
        });
    }
}

pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MousePos>()
            .init_resource::<MousePosWorld>()
            .add_systems(
                UiSchedule,
                (mouse_events::emit_deselect_click_sy,).in_set(UiSet::Mouse),
            )
            .add_observer(on_emit_click2_down)
            .add_observer(on_emit_click2_up)
            .add_systems(
                Last,
                (cursor_icon_sy, crosshair_sy.run_if_not_loading()),
            )
            .add_systems(First, mouse_pos::update_mouse_pos_sy);
    }
}
