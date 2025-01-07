use bevy::prelude::*;

#[derive(Component)]
pub struct LevelTerrain;

#[derive(Component, Default)]
pub struct Character;

#[derive(Component, Default,Reflect)]
pub struct MaterialMarker(pub String);
