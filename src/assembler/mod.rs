use bevy::prelude::*;
use components::Sky;
use events::{LevelLoadedEvent, PostProgresssEvent, PrefabReadyEvent, PrepareLevelEvent};

use iyes_progress::ProgressPlugin;

use resources::GameWorld;
use systems::{load_level, on_level_loaded, setup_world};

pub mod components;
pub mod events;
pub mod resources;
pub mod systems;
pub(crate) struct LoaderPlugin;

impl Plugin for LoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AssetLoadState>()
            .enable_state_scoped_entities::<AssetLoadState>();
            app.add_plugins(
                ProgressPlugin::<AssetLoadState>::new()
                    .with_state_transition(AssetLoadState::Loading, AssetLoadState::Loaded),
            );
            app.register_type::<Sky>()
            .add_event::<PrepareLevelEvent>()
            .add_event::<PostProgresssEvent>()
            .add_event::<PrefabReadyEvent>()
            .add_event::<LevelLoadedEvent>()
            .add_systems(
                Update,
                (
                    setup_world.run_if(on_event::<PrepareLevelEvent>),
                    load_level.run_if(resource_added::<GameWorld>),
                ),
            )
            .add_observer(on_level_loaded);
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
