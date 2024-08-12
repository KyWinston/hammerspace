use assembler::LoaderPlugin;
use bevy::prelude::*;

#[cfg(feature = "editor")]
use editor::EditorPlugin;

use pathfind::{events::PathEvent, PathFindPlugin};
use resources::LevelFolder;

pub mod assembler;
#[cfg(feature = "editor")]
pub mod editor;
pub mod pathfind;
pub mod resources;

#[cfg(feature = "proc_terrain")]
pub mod terrain;

pub struct HammerspacePlugin {
    pub level_folder: String,
}

impl Plugin for HammerspacePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            LoaderPlugin,
            PathFindPlugin,
            #[cfg(feature = "proc_terrain")]
            TerrainPlugin,
            #[cfg(feature = "editor")]
            EditorPlugin,
        ))
        .add_event::<PathEvent>()
        .insert_resource::<LevelFolder>(LevelFolder(self.level_folder.to_string()));
    }
}
