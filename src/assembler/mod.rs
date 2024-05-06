use self::events::LoadLevelEvent;
use bevy::prelude::*;

pub mod components;
pub mod events;
pub mod resources;
pub mod systems;
pub struct LoaderPlugin;

impl Plugin for LoaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LoadLevelEvent>();
    }
}
