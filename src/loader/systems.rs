use bevy::{gltf::{Gltf, GltfMesh}, prelude::*};
use bevy_basic_ui::AppState;

use crate::resources::LevelFolder;

pub fn assemble_level(
    _lvl_folder: Res<LevelFolder>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    scene: Res<Assets<Gltf>>,
    gltf: Res<Assets<GltfMesh>>,
    mut to_game: ResMut<NextState<AppState>>
) {
        let scene:Handle<Gltf> = asset_server.load("levels/tradewind_town.gltf#Scene0");
}
