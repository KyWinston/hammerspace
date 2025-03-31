use bevy::{asset::Handle, gltf::GltfNode, prelude::*};

#[derive(Resource)]
pub struct GameWorld(pub Handle<Scene>);

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
    // pub fn fetch(
    //     &mut self,
    //     name: &String,
    //     assets_gltf: &ResMut<Assets<Gltf>>,
    //     nodes: &ResMut<Assets<GltfNode>>,
    // ) -> Result<(usize, Option<Handle<Image>>), String> {
    //     if let Some(gltf) = assets_gltf.get(self.handle.id()) {
    //         if gltf.named_nodes.contains_key(name.as_str()) {
    //             let node = gltf.named_nodes[name.as_str()].clone();
    //             if !self.nodes.contains(&node) {
    //                 self.nodes.push(node.clone());
    //             }
    //             if let Some(target_node) = nodes.get(node.id()) {
    //                 Ok((target_node.index, None))
    //             } else {
    //                 Err("couldn't fetch desired mesh".to_string())
    //             }
    //         } else {
    //             Err("couldn't fetch desired mesh".to_string())
    //         }
    //     } else {
    //         Err("library is missing".to_string())
    //     }
    // }
}
