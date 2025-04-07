use systems::check_in_view;
use bevy::prelude::*;

pub mod components;
mod systems;

pub(crate) struct InteractPlugin;

impl Plugin for InteractPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, check_in_view);
    }
}
