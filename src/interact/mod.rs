use systems::check_in_view;
use bevy::prelude::*;

pub mod components;
pub mod systems;

pub struct InteractPlugin;

impl Plugin for InteractPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, check_in_view);
    }
}
