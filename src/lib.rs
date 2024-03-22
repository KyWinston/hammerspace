use bevy::prelude::*;
use editor::EditorPlugin;
use loader::LoaderPlugin;
use resources::LevelFolder;
use viewer::ViewerPlugin;

pub mod controller;
pub mod editor;
pub mod viewer;
pub mod loader;
pub mod resources;
pub struct HammerspacePlugin {
    pub level_folder: String,
}

impl Plugin for HammerspacePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((EditorPlugin, ViewerPlugin,LoaderPlugin))
        .insert_resource::<LevelFolder>(LevelFolder(self.level_folder.to_string()))
        .insert_resource::<LevelToLoad>(LevelToLoad("test.gltf"))
    }
}
