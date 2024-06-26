use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::init::LoadingState as SLoadingState;

#[derive(AssetCollection, Resource)]
pub struct ImageAssets {
    #[asset(path = "crosshair.png")]
    pub crosshair: Handle<Image>,
    #[asset(path = "ste-light.png")]
    pub stencil_text: Handle<Image>,
}

pub struct LoadAssetsPlugin;

impl Plugin for LoadAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(SLoadingState::LoadAssets)
                .continue_to_state(SLoadingState::LoadAssets.next())
                .load_collection::<ImageAssets>(),
        );
    }
}
