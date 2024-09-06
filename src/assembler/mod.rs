use bevy::prelude::*;
use resources::{
    check_assets_ready, init_resources, ImageAssets, ImageAssetsLoading, MeshAssets, PreparedScenes, SessionAssets
};
use systems::setup_blueprints;


pub mod components;
pub mod resources;
pub mod systems;
pub struct LoaderPlugin;

impl Plugin for LoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AssetLoadState>()
            .init_resource::<ImageAssets>()
            .init_resource::<MeshAssets>()
            .init_resource::<PreparedScenes>()
            .add_systems(OnEnter(AssetLoadState::Loaded), setup_blueprints)
            .add_systems(
                Update,
                init_resources.run_if(resource_added::<SessionAssets>),
            )
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
