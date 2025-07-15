use bevy::{asset::Handle, gltf::GltfNode, prelude::*};

#[derive(Resource)]
pub struct GameWorld(pub Handle<Scene>);

#[derive(Resource)]
pub struct Dungeon(pub String);


#[derive(Resource)]
pub struct Library {
    pub meshes: Vec<Handle<GltfNode>>,
    pub textures: Vec<Handle<Image>>,
    // pub animations: Handle<Anim>,
}

impl Library {
    pub fn new() -> Self {
        Self {
            meshes: vec![],
            textures: vec![],
        }
    }
}
