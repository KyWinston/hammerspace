use bevy::{
    asset::Handle,
    gltf::{Gltf, GltfMesh},
    prelude::*,
};

#[derive(Resource)]
pub struct GameWorld(pub Handle<Scene>);

#[derive(Resource)]
pub struct Library {
    pub handle: Handle<Gltf>,
    pub meshes: Vec<Handle<GltfMesh>>,
    pub textures: Vec<Handle<Image>>,
    // pub animations: Handle<Anim>,
}

impl Library {
    pub fn new(handle: Handle<Gltf>) -> Self {
        Self {
            handle,
            meshes: vec![],
            textures: vec![],
        }
    }
    pub fn fetch(
        &mut self,
        name: &String,
        assets_gltf: ResMut<Assets<Gltf>>,
    ) -> Result<(Handle<GltfMesh>, Option<Handle<Image>>), String> {
        if let Some(gltf) = assets_gltf.get(self.handle.id()) {
            if gltf.named_meshes.contains_key(name.as_str()) {
                let mesh_handle = gltf.named_meshes[name.as_str()].clone();
                if !self.meshes.contains(&mesh_handle) {
                    self.meshes.push(mesh_handle.clone());
                }
                Ok((mesh_handle, None))
            } else {
                Err("couldn't fetch desired mesh".to_string())
            }
        } else {
            Err("library is missing".to_string())
        }
    }
}
