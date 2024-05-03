use bevy::prelude::*;
use loader::LoaderPlugin;
use resources::LevelFolder;
use viewer::ViewerPlugin;

pub mod controller;
pub mod loader;
pub mod resources;
pub mod viewer;

pub struct HammerspacePlugin {
    pub level_folder: String,
}

impl Plugin for HammerspacePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<HammerState>()
            .add_plugins((ViewerPlugin, LoaderPlugin));
        app.insert_resource::<LevelFolder>(LevelFolder(self.level_folder.to_string()));
    }
}

#[derive(States, Default, Debug, Hash, Eq, PartialEq, Clone)]
pub enum HammerState {
    #[default]
    Menu,
    Loading,
    Editor,
    Game,
}
