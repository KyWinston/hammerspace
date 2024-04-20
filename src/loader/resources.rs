use bevy::{gltf::Gltf, prelude::*};

#[cfg(feature = "level-loader")]
#[derive(Resource)]
pub struct NextLevel(pub Handle<Gltf>, pub Option<String>);

#[derive(Resource)]
pub struct LoadingTextures(pub Vec<Handle<Image>>);
