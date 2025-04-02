use bevy::{prelude::*, scene::SceneInstance};

use super::{
    components::Level,
    events::{LevelLoadedEvent, PrepareLevelEvent},
    resources::{GameWorld, Library},
};

pub(crate) fn setup_world(
    mut level_ev: EventReader<PrepareLevelEvent>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    for ev in level_ev.read() {
        let gltf =
            asset_server.load(GltfAssetLabel::Scene(0).from_asset(format!("levels/{}.gltf", ev.0)));
        commands.insert_resource(GameWorld(gltf));
        commands.insert_resource(Library::new());
    }
}

pub(crate) fn load_level(mut commands: Commands, game_world: Res<GameWorld>) {
    commands.spawn((Level, SceneRoot(game_world.0.clone())));
}

pub(crate) fn on_level_loaded(
    trigger: Trigger<OnAdd, SceneInstance>,
    mut level_ev: EventWriter<LevelLoadedEvent>,
    levels: Query<Entity, With<Level>>,
) {
    for level in levels.iter() {
        if trigger.entity() == level {
            level_ev.send(LevelLoadedEvent(trigger.entity()));
        }
    }
}
