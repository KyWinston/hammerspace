use bevy::prelude::*;
#[cfg(feature = "multiplayer")]
use comms::CommsPlugin;
#[cfg(feature = "level-loader")]
use loader::LoaderPlugin;
#[cfg(feature = "level-loader")]
use resources::LevelFolder;
use viewer::ViewerPlugin;

#[cfg(feature = "multiplayer")]
pub mod comms;
pub mod controller;
pub mod loader;
pub mod resources;
pub mod viewer;

pub struct HammerspacePlugin {
    #[cfg(feature = "level-loader")]
    pub level_folder: String,
}

impl Plugin for HammerspacePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<HammerState>().add_plugins((
            ViewerPlugin,
            #[cfg(feature = "level-loader")]
            LoaderPlugin,
            #[cfg(feature = "multiplayer")]
            CommsPlugin,
        ));
        #[cfg(feature = "level-loader")]
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
