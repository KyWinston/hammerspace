use crate::location_marker::LocationMarkerPlugin;
use assembler::LoaderPlugin;
use bevy::prelude::*;
use blenvy::BlenvyPlugin;

use interact::InteractPlugin;
#[cfg(feature = "pathfind")]
use pathfind::{events::PathEvent, PathFindPlugin};
use resources::HammerspaceConfig;
#[cfg(feature = "proc_terrain")]
use terrain::TerrainPlugin;

pub mod ai_controller;
pub mod assembler;
pub mod interact;
pub mod location_marker;
pub mod resources;

#[cfg(feature = "pathfind")]
pub mod pathfind;

#[cfg(feature = "proc_terrain")]
pub mod terrain;

pub struct HammerspacePlugin {
    pub config: HammerspaceConfig,
}

impl Plugin for HammerspacePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            LoaderPlugin,
            LocationMarkerPlugin,
            InteractPlugin,
            BlenvyPlugin::default(),
            #[cfg(feature = "pathfind")]
            PathFindPlugin,
            #[cfg(feature = "proc_terrain")]
            TerrainPlugin,
        ));
        #[cfg(feature = "pathfind")]
        app.add_event::<PathEvent>();
        app.insert_resource::<HammerspaceConfig>(self.config.clone());
    }
}
