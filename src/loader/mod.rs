use bevy::prelude::*;

use crate::HammerState;

use self::{
    events::LoadLevelEvent,
    systems::{assemble_level, fetch_level_handle},
};

pub mod components;
pub mod events;
pub mod resources;
pub mod systems;
pub struct LoaderPlugin;

impl Plugin for LoaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LoadLevelEvent>().add_systems(
            Update,
            (
                fetch_level_handle,
                assemble_level.run_if(in_state(HammerState::Loading)),
            ),
        );
    }
}
