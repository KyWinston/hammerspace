use bevy::prelude::*;
use bevy_basic_ui::AppState;

use self::{events::LoadLevelEvent, systems::assemble_level};

pub mod components;
pub mod events;
pub mod systems;
pub struct LoaderPlugin;

impl Plugin for LoaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LoadLevelEvent>()
            .add_systems(Update, assemble_level.run_if(in_state(AppState::Loading)));
    }
}
