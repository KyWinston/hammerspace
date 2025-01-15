use bevy::prelude::*;

#[derive(Event)]
pub struct LocationSpawnEvent(pub Name, pub Option<Entity>);
