use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Component)]
pub struct Level;

#[derive(Component, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
pub struct Sky {
    pub first_cascade: f32,
    pub max_distance: f32,
    pub directional_light_exponent: f32,
}

#[derive(Component, Default)]
pub struct Character;

#[derive(Component, Default)]
#[require(Transform)]
pub struct Prefab(pub String);
