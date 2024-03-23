use bevy::{gltf::Gltf, prelude::*};

#[derive(Resource)]
pub struct NextLevel(pub Handle<Gltf>);
