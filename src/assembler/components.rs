use bevy::prelude::*;

#[derive(Component)]
pub struct LevelTerrain;

#[derive(Component)]
pub struct Sky;

#[derive(Component, Default)]
pub struct Character;

#[derive(Component, Default, Reflect)]
pub struct MaterialMarker<M>(pub Handle<M>) where M:blenvy::Material;
