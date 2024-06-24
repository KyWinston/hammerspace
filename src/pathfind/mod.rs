use bevy::prelude::*;
use vleue_navigator::VleueNavigatorPlugin;

pub mod components;
pub mod systems;
pub mod events;
pub mod resources;
pub struct PathFindPlugin;

impl Plugin for PathFindPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(VleueNavigatorPlugin);
    }
}
