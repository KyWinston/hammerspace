use bevy::prelude::*;

#[derive(Resource)]
pub struct LevelFolder(pub String);

#[derive(Resource)]
pub struct LevelToLoad(pub String);