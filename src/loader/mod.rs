use bevy::prelude::*;

use self::events::LoadLevelEvent;

#[cfg(feature = "level-loader")]
use self::systems::{assemble_level, fetch_level_handle};

pub mod components;
pub mod events;
pub mod resources;
pub mod systems;
pub struct LoaderPlugin;

impl Plugin for LoaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LoadLevelEvent>();
        #[cfg(feature = "level-loader")]
        app.add_systems(
            Update,
            (
                fetch_level_handle,
                assemble_level.run_if(in_state(HammerState::Loading)),
            ),
        );
    }
}
