use bevy::prelude::*;

pub enum LevelType {
    Premade,
    DungeonTile,
}
#[derive(Event)]
pub struct PrepareLevelEvent(pub String, pub LevelType);

#[derive(Event)]
pub struct PostProgresssEvent(pub String, pub u32, pub u32);

#[derive(Event)]
pub struct PrefabReadyEvent(pub Entity);

#[derive(Event)]
pub struct LevelLoadedEvent(pub Entity);
