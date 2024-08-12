use assembler::LoaderPlugin;
use avian3d::PhysicsPlugins;
use bevy::prelude::*;
use bevy_console::{ConsoleConfiguration, ConsolePlugin};

use bevy_ggrs::GgrsApp;
#[cfg(feature = "editor")]
use editor::EditorPlugin;
#[cfg(feature = "netcode")]
use network::NetworkPlugin;

use pathfind::{events::PathEvent, PathFindPlugin};
use resources::LevelFolder;

pub mod assembler;
#[cfg(feature = "editor")]
pub mod editor;
pub mod pathfind;
pub mod resources;

#[cfg(feature = "netcode")]
pub mod network;

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
            PhysicsPlugins::default(),
            ConsolePlugin,
            #[cfg(feature = "netcode")]
            NetworkPlugin,
            #[cfg(feature = "debug")]
            PhysicsDebugPlugin::default(),
            #[cfg(feature = "proc_terrain")]
            TerrainPlugin,
            #[cfg(feature = "editor")]
            EditorPlugin,
        ))
        .insert_resource(ConsoleConfiguration {
            ..Default::default()
        })
        .add_event::<PathEvent>()
        .insert_resource::<LevelFolder>(LevelFolder(self.level_folder.to_string()))
        .rollback_component_with_clone::<Transform>();
    }
}
