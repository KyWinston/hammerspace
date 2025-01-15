use bevy::prelude::*;
use events::LocationSpawnEvent;
use systems::send_to_marker;

pub mod components;
pub mod events;
pub mod systems;

pub struct LocationMarkerPlugin;
impl Plugin for LocationMarkerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LocationSpawnEvent>()
            .add_systems(Update, send_to_marker);
    }
}
