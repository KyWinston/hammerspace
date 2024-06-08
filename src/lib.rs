use assembler::LoaderPlugin;
use bevy::prelude::*;
use resources::LevelFolder;

pub mod assembler;
pub mod resources;

pub struct HammerspacePlugin {
    pub level_folder: String,
}

impl Plugin for HammerspacePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(LoaderPlugin)
            .insert_resource::<LevelFolder>(LevelFolder(self.level_folder.to_string()));
    }
}
