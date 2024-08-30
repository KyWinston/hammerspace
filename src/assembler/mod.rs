use self::events::LoadLevelEvent;
use bevy::prelude::*;
use resources::{check_assets_ready, init_resources, ImageAssets, ImageAssetsLoading, MeshAssets, SessionAssets};
use systems::setup_level;

pub mod components;
pub mod events;
pub mod resources;
pub mod systems;
pub struct LoaderPlugin;

impl Plugin for LoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AssetLoadState>()
            .add_event::<LoadLevelEvent>()
            .init_resource::<ImageAssets>()
            .init_resource::<MeshAssets>()
            .add_systems(Update, init_resources.run_if(resource_added::<SessionAssets>))
            .add_systems(OnEnter(AssetLoadState::Loaded), setup_level)
            .add_systems(
                Update,
                check_assets_ready
                    .run_if(resource_exists::<ImageAssetsLoading>)
                    .run_if(in_state(AssetLoadState::Loading)),
            );
    }
}

#[derive(PartialEq, Eq, Debug, Hash, Clone, Copy, Default, States)]
pub enum AssetLoadState {
    #[default]
    Initializing,
    Loading,
    Loaded,
    Failed,
}
