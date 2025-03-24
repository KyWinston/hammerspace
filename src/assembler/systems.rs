// use crate::interact::components::Actor;
use bevy::{prelude::*, scene::SceneInstanceReady};

use super::{
    components::{Level, PrefabReady},
    events::{LevelLoadedEvent, PrefabReadyEvent, PrepareLevelEvent},
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
        // let library = asset_server.load(GltfAssetLabel::Scene(0).from_asset("library.gltf"));
        // commands.insert_resource(Library::new(library));
    }
}

pub(crate) fn load_level(mut commands: Commands, game_world: Res<GameWorld>) {
    commands.spawn((Level, SceneRoot(game_world.0.clone())));
}

// pub fn spawn_actor<'a>(
//     commands: &'a mut Commands,
//     mut scenes: ResMut<Assets<Scene>>,
//     mut gltf: ResMut<Assets<Gltf>>,
//     mut meshes:
//     mut library: ResMut<Library>,
//     name: String,
//     location: Transform,
// ) -> Result<EntityCommands<'a>, String> {
//     if let Some(actor) = gltf.get(library.0.id()) {
//         let mesh = scenes.get(actor.scenes[0].id());
//         Ok(commands.spawn((Prefab, Actor, Name::from(name), location)))
//     } else {
//         Err("actor could not be found".to_string())
//     }
// }

pub(crate) fn on_level_loaded(
    trigger: Trigger<OnAdd, SceneInstanceReady>,
    mut level_ev: EventWriter<LevelLoadedEvent>,
    levels: Query<Entity, With<Level>>,
) {
    for level in levels.iter() {
        if trigger.entity() == level {
            level_ev.send(LevelLoadedEvent(trigger.entity()));
        }
    }
}

pub(crate) fn on_prefab_loaded(
    trigger: Trigger<OnAdd, PrefabReady>,
    mut ev: EventWriter<PrefabReadyEvent>,
) {
    ev.send(PrefabReadyEvent(trigger.entity()));
}
