use bevy::prelude::*;
use loader::LoaderPlugin;
use resources::LevelFolder;
use viewer::ViewerPlugin;

pub mod editor;
pub mod loader;
pub mod resources;
pub mod viewer;
pub mod controller;
pub struct HammerspacePlugin {
    pub level_folder: String,
}

impl Plugin for HammerspacePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((ViewerPlugin, LoaderPlugin))
            .insert_resource::<LevelFolder>(LevelFolder(self.level_folder.to_string()));
    }
}
