use bevy::{asset::Handle, gltf::Gltf, prelude::*};

#[derive(Resource)]
pub struct GameWorld(pub Handle<Scene>);

#[derive(Resource)]
pub struct Library {
    pub handle: Handle<Gltf>,
    pub meshes: Vec<Handle<Mesh>>,
    pub materials: Vec<Handle<Image>>,
    // pub animations: Handle<Anim>,
}

impl Library {
    pub fn new(handle: Handle<Gltf>) -> Self {
        Self {
            handle,
            meshes: vec![],
            materials: vec![],
        }
    }
}
