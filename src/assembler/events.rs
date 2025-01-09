use bevy::prelude::*;

#[derive(Event)]
pub struct PrepareLevelEvent(pub String);

#[derive(Event)]
pub struct PostProgresssEvent(pub String, pub u32, pub u32);
