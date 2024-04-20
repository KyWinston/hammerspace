use bevy::prelude::*;

#[derive(Event)]
pub struct LoadLevelEvent(pub String, pub Option<String>);
