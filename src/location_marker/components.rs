use bevy::prelude::*;

#[derive(Component, Reflect)]
#[require(Name, Transform)]
pub struct LocationMarker;
