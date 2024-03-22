use bevy::{
    gltf::{Gltf, GltfMesh},
    prelude::*,
};
use bevy_basic_ui::AppState;

use crate::resources::{LevelFolder, LevelToLoad};

pub fn assemble_level(
    lvl_folder: Res<LevelFolder>,
    lvl_to_load: Res<LevelToLoad>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    gltf: Res<Assets<Gltf>>,
    mut to_game: ResMut<NextState<AppState>>,
) {
    let scene: &Gltf = gltf
        .get(asset_server.load(lvl_folder.0.to_string() + "/" + &lvl_to_load.0))
        .unwrap();
    let scene_nodes = &scene.named_nodes;
    println!("{:?}", scene_nodes);
}
