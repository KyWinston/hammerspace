use bevy::prelude::*;

#[derive(Component)]
pub struct LevelTerrain;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Sky;

#[derive(Component, Default)]
pub struct Character;

#[derive(Component)]
pub struct BlueprintInstanceReady;

#[derive(Component)]
pub struct GameWorldTag;
