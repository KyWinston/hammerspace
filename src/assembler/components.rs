use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use super::systems::unpack_prefab;

#[derive(Component)]
pub struct Level;

#[derive(Component, Reflect, Serialize, Deserialize)]
#[reflect(Component,Serialize, Deserialize)]
pub struct Sky {
    pub directional_light_color: Color,
    pub color: Color,
    pub directional_light_exponent: f32,
}

#[derive(Component, Default)]
pub struct Character;

#[derive(Component, Default)]
#[require(Transform)]
pub struct Prefab(pub String);

#[derive(Component)]
pub struct PrefabReady;
