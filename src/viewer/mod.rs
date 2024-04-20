use bevy::prelude::*;

use crate::HammerState;

use self::systems::{
    fly_cam, orbit_cam, spawn_cameras, switch_to_editor_view, switch_to_game_view,
};

pub struct ViewerPlugin;

mod components;
pub mod resources;
mod systems;

pub const PANNING_KEYS: [KeyCode; 6] = [
    KeyCode::KeyA,
    KeyCode::KeyD,
    KeyCode::KeyW,
    KeyCode::KeyS,
    KeyCode::KeyQ,
    KeyCode::KeyE,
];

impl Plugin for ViewerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_cameras)
            .add_systems(OnEnter(HammerState::Editor), switch_to_editor_view)
            .add_systems(OnEnter(HammerState::Showcase), switch_to_editor_view)
            .add_systems(OnEnter(HammerState::Game), switch_to_game_view)
            .add_systems(
                Update,
                (fly_cam.run_if(in_state(HammerState::Editor)), orbit_cam),
            );
    }
}
