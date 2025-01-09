use bevy::prelude::*;

#[derive(Event)]
pub struct PrepareLevelEvent(pub String);

#[derive(Event)]
pub struct PostProgresssEvent(pub String, pub usize, pub usize);
