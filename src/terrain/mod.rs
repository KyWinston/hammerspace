use bevy::prelude::*;

pub mod components;
pub mod systems;

pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {}
}
