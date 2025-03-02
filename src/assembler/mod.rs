use bevy::prelude::*;
use components::MaterialMarker;
use events::{BlueprintReadyEvent, LevelLoadedEvent, PostProgresssEvent, PrepareLevelEvent};

use iyes_progress::ProgressPlugin;
use resources::{
    check_assets_ready, init_resources, ImageAssets, ImageAssetsLoading, MeshAssets,
    PreparedScenes, SessionAssets,
};

use systems::{on_blueprint_complete, on_level_loaded, setup_blueprints};

pub mod components;
pub mod events;
pub mod resources;
pub mod systems;
pub struct LoaderPlugin;

impl Plugin for LoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AssetLoadState>()
            .enable_state_scoped_entities::<AssetLoadState>()
            .add_plugins(
                ProgressPlugin::<AssetLoadState>::new()
                    .with_state_transition(AssetLoadState::Loading, AssetLoadState::Loaded),
            )
            .init_resource::<ImageAssets>()
            .init_resource::<MeshAssets>()
            .init_resource::<PreparedScenes>()
            .add_event::<PrepareLevelEvent>()
            .add_event::<PostProgresssEvent>()
            .add_event::<BlueprintReadyEvent>()
            .add_event::<LevelLoadedEvent>()
            .add_systems(
                Update,
                (
                    setup_blueprints.run_if(on_event::<PrepareLevelEvent>),
                    init_resources.run_if(resource_added::<SessionAssets>),
                    check_assets_ready
                        .run_if(resource_exists::<ImageAssetsLoading>)
                        .run_if(in_state(AssetLoadState::Loading)),
                )
                    .chain(),
            )
            .add_observer(on_level_loaded)
            .add_observer(on_blueprint_complete)
            .register_type::<MaterialMarker>();
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
