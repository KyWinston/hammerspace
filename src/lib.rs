use bevy::prelude::*;
use comms::CommsPlugin;
use loader::LoaderPlugin;
use resources::LevelFolder;
use viewer::ViewerPlugin;

pub mod comms;
pub mod controller;
pub mod editor;
pub mod loader;
pub mod resources;
pub mod viewer;

pub struct HammerspacePlugin {
    pub level_folder: String,
}

impl Plugin for HammerspacePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<HammerState>()
            .add_plugins((ViewerPlugin, LoaderPlugin, CommsPlugin))
            .insert_resource::<LevelFolder>(LevelFolder(self.level_folder.to_string()));
    }
}

#[derive(States,Default, Debug, Hash, Eq, PartialEq, Clone)]
pub enum HammerState {
    #[default]
    Menu,
    Loading,
    Editor,
    Game,
    Showcase,
}
