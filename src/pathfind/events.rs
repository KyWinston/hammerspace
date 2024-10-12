use bevy::prelude::*;

#[derive(Event)]
pub struct PathEvent(pub Entity, pub Vec3, pub Vec3);
