use assembler::LoaderPlugin;
use bevy::prelude::*;
use pathfind::{events::PathEvent, PathFindPlugin};
use resources::LevelFolder;

pub mod assembler;
pub mod pathfind;
pub mod resources;

pub struct HammerspacePlugin {
    pub level_folder: String,
}

impl Plugin for HammerspacePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((LoaderPlugin, PathFindPlugin))
            .add_event::<PathEvent>()
            .insert_resource::<LevelFolder>(LevelFolder(self.level_folder.to_string()));
    }
}
