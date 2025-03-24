use bevy::prelude::*;
use serde::{Deserialize, Serialize};

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
#[require(Mesh3d, Transform)]
pub struct Prefab;

#[derive(Component)]
pub struct PrefabReady;
