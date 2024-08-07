use assembler::LoaderPlugin;
use avian3d::PhysicsPlugins;
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
pub struct HammerspacePlugin {
    pub level_folder: String,
}

impl Plugin for HammerspacePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            LoaderPlugin,
            PathFindPlugin,
            PhysicsPlugins::default(),
            #[cfg(feature = "debug")]
            PhysicsDebugPlugin::default(),
        ));

        #[cfg(feature = "editor")]
        app.add_plugins(EditorPlugin);
        app.add_event::<PathEvent>()
            .insert_resource::<LevelFolder>(LevelFolder(self.level_folder.to_string()));
    }
}
