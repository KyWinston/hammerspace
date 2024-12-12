use bevy::prelude::*;
use vleue_navigator::VleueNavigatorPlugin;

pub mod components;
pub mod events;
pub mod resources;
pub mod systems;

pub struct PathFindPlugin;

#[derive(Component)]
pub struct Obstacle;

impl Plugin for PathFindPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(VleueNavigatorPlugin);
        // .add_systems(Update, get_path);
    }
}

pub const MATERIAL_OBSTACLE_LIVE: Handle<StandardMaterial> = Handle::weak_from_u128(0);
pub const MATERIAL_OBSTACLE_CACHED: Handle<StandardMaterial> = Handle::weak_from_u128(1);
