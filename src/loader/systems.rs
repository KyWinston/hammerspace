use bevy::{gltf::Gltf, prelude::*};
use bevy_basic_ui::AppState;

use crate::resources::LevelFolder;

use super::events::LoadLevelEvent;

pub fn assemble_level(
    lvl_folder: Res<LevelFolder>,
    mut lvl_ev: EventReader<LoadLevelEvent>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    gltf: Res<Assets<Gltf>>,
    mut to_game: ResMut<NextState<AppState>>,
) {
    for ev in lvl_ev.read() {
        let scene: &Gltf = gltf
            .get(asset_server.load(lvl_folder.0.to_string() + "/" + &ev.0))
            .unwrap();
        let scene_nodes = &scene.named_nodes;
        println!("{:?}", scene_nodes);
    }
}
