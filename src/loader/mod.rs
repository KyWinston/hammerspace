use bevy::prelude::*;
use bevy_basic_ui::AppState;

use self::systems::assemble_level;

pub mod components;
pub mod systems;

pub struct LoaderPlugin;

impl Plugin for LoaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Loading), assemble_level);
    }
}
