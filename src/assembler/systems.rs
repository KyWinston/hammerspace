use bevy::{gltf::GltfMesh, prelude::*, scene::SceneInstanceReady};

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
        let library = asset_server.load(GltfAssetLabel::Scene(0).from_asset("library.gltf"));
        commands.insert_resource(Library::new(library));
    }
}

pub(crate) fn load_level(
    mut commands: Commands,
    game_world: Res<GameWorld>,
    library: Res<Library>,
    gltf: ResMut<Assets<Gltf>>,
) {
    commands.spawn((Level, SceneRoot(game_world.0.clone())));
    println!("{:?}", gltf.get(library.handle.id()));
}

pub fn unpack_prefab(
    trigger: Trigger<OnAdd, Prefab>,
    prefabs: Query<(Entity, &Prefab)>,
    mut commands: Commands,
    gltf_mesh: ResMut<Assets<GltfMesh>>,
    meshes: ResMut<Assets<Gltf>>,
    mut library: ResMut<Library>,
) {
    if let Ok(lib) = library.fetch(&prefabs.get(trigger.entity()).unwrap().1.0, meshes) {
        if let Some(gltf_mesh) = gltf_mesh.get(lib.0.id()) {
            let mesh = gltf_mesh.primitives[0].mesh.clone();
            commands.entity(trigger.entity()).insert(Mesh3d(mesh));
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
