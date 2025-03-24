use bevy::{
    gltf::{GltfMesh, GltfNode},
    prelude::*,
    scene::SceneInstanceReady,
};

use super::{
    components::{Level, Prefab, PrefabReady},
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
        let library = asset_server.load("library.gltf");
        commands.insert_resource(Library::new(library));
    }
}

pub(crate) fn load_level(mut commands: Commands, game_world: Res<GameWorld>) {
    commands.spawn((Level, SceneRoot(game_world.0.clone())));
}

pub fn unpack_prefabs(
    prefabs: Query<(Entity, &Prefab), Without<PrefabReady>>,
    mut commands: Commands,
    gltf_nodes: ResMut<Assets<GltfNode>>,
    meshes: ResMut<Assets<Gltf>>,
    mut library: ResMut<Library>,
    asset_server: Res<AssetServer>,
) {
    let lib_meshes = meshes;
    for (ent, prefab) in prefabs.iter() {
        if let Ok(lib) = library.fetch(&prefab.0, &lib_meshes, &gltf_nodes) {
            let chosen_node =
                asset_server.load(GltfAssetLabel::Node(lib.0).from_asset("library.gltf"));
            println!("{:?}", gltf_nodes.get(chosen_node.id()));
            commands.entity(ent).insert((PrefabReady,));
        }
    }
}

pub(crate) fn on_level_loaded(
    trigger: Trigger<OnAdd, SceneInstanceReady>,
    mut level_ev: EventWriter<LevelLoadedEvent>,
    levels: Query<Entity, With<Level>>,
) {
    for level in levels.iter() {
        if trigger.entity() == level {
            println!("level is loaded");
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
