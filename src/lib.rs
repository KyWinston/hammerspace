use assembler::LoaderPlugin;
use bevy::prelude::*;

use blenvy::BlenvyPlugin;
#[cfg(feature = "editor")]
use editor::EditorPlugin;

#[cfg(feature = "pathfind")]
use pathfind::{events::PathEvent, PathFindPlugin};
use resources::HammerspaceConfig;

pub mod assembler;
#[cfg(feature = "editor")]
pub mod editor;
#[cfg(feature = "pathfind")]
pub mod pathfind;

pub mod resources;

#[cfg(feature = "proc_terrain")]
pub mod terrain;

pub struct HammerspacePlugin {
    pub config: HammerspaceConfig,
}

impl Plugin for HammerspacePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            LoaderPlugin,
        
            BlenvyPlugin::default(),
            #[cfg(feature = "pathfind")]
            PathFindPlugin,
            #[cfg(feature = "proc_terrain")]
            TerrainPlugin,
            #[cfg(feature = "editor")]
            EditorPlugin,
        ));
        #[cfg(feature = "pathfind")]
        app.add_event::<PathEvent>();
        app.insert_resource::<HammerspaceConfig>(self.config.clone());
    }
}
