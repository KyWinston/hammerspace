use bevy::prelude::*;

use self::systems::{configure_gamepad, gamepad_connections};

pub struct ControllerPlugin;

pub mod resources;
pub mod systems;

impl Plugin for ControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, configure_gamepad)
            .add_systems(Update, (gamepad_connections,));
    }
}
