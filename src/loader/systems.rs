use bevy::{
    asset::LoadState,
    gltf::{Gltf, GltfMesh, GltfNode},
    prelude::*,
};

#[cfg(feature = "handle-physics")]
use bevy::render::mesh::Indices;

#[cfg(feature = "handle-physics")]
use bevy_rapier3d::dynamics::RigidBody;

use crate::{resources::LevelFolder, HammerState};

use super::{components::PrefabBundle, events::LoadLevelEvent, resources::NextLevel};

pub fn fetch_level_handle(
    lvl_folder: Res<LevelFolder>,
    mut lvl_ev: EventReader<LoadLevelEvent>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut state: ResMut<NextState<HammerState>>,
) {
    for ev in lvl_ev.read() {
        let path = lvl_folder.0.to_string() + "/" + &ev.0;
        let gltf_scene: Handle<Gltf> = asset_server.load(&path);
        match &ev.1 {
            Some(item) => {
                commands.insert_resource(NextLevel(gltf_scene, Some(item.to_string())));
            }
            None => {
                commands.insert_resource(NextLevel(gltf_scene, None));
            }
        }
        state.set(HammerState::Loading);
    }
}

pub fn assemble_level(
    mut commands: Commands,
    next_lvl: Res<NextLevel>,
    assets_nodes: Res<Assets<GltfNode>>,
    assets_meshes: Res<Assets<GltfMesh>>,
    assets_gltf: Res<Assets<Gltf>>,
    meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
    mut game_state: ResMut<NextState<HammerState>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if let Some(gltf) = assets_gltf.get(&next_lvl.0) {
        if next_lvl.1.is_some() {
            for node_id in &gltf.named_nodes {
                if !node_id.0.contains(next_lvl.1.as_ref().unwrap()) {
                    continue;
                }
                let mesh_id = assets_nodes.get(node_id.1).unwrap();

                let mesh = assets_meshes
                    .get(mesh_id.mesh.as_ref().unwrap().id())
                    .unwrap();

                commands.spawn(PbrBundle {
                    mesh: mesh.primitives[0].mesh.clone(),
                    material: materials.add(StandardMaterial {
                        base_color_texture: placehold_texture(
                            &next_lvl.1.as_ref().unwrap(),
                            "diffuse",
                            &asset_server,
                        ),
                        // normal_map_texture: placehold_texture(
                        //     &next_lvl.1.as_ref().unwrap(),
                        //     "normal",
                        //     &asset_server,
                        // ),
                        // flip_normal_map_y: true,
                        // occlusion_texture: Some(textures.2),
                        ..default()
                    }),
                    ..default()
                });
                let mesh_id = assets_nodes.get(node_id.1).unwrap();
                if mesh_id.mesh.is_none() {
                    continue;
                }
                game_state.set(HammerState::Showcase);
            }
        } else {
            for node_id in &gltf.named_nodes {
                if node_id.0.contains("_collider") || node_id.0.contains("_ref") {
                    continue;
                }
                let mesh_id = assets_nodes.get(node_id.1).unwrap();
                if mesh_id.mesh.is_none() {
                    continue;
                }
                let mesh = assets_meshes
                    .get(mesh_id.mesh.as_ref().unwrap().id())
                    .unwrap();
                #[cfg(feature = "handle-physics")]
                let (verts, indices) =
                    get_collision_data(node_id.0, &gltf, &assets_nodes, &assets_meshes, &meshes);
                commands.spawn(PrefabBundle::new(
                    #[cfg(feature = "handle-physics")]
                    RigidBody::Fixed,
                    mesh.primitives[0].mesh.clone(),
                    #[cfg(feature = "handle-physics")]
                    verts,
                    #[cfg(feature = "handle-physics")]
                    indices,
                    materials.add(StandardMaterial {
                        base_color_texture: Some(asset_server.load(
                            "textures/".to_string()
                                + &next_lvl.1.as_ref().unwrap()
                                + "_diffuse.png",
                        )),
                        // normal_map_texture: Some(textures.4),
                        // flip_normal_map_y: true,
                        // occlusion_texture: Some(textures.2),
                        ..default()
                    }),
                ));
            }
            game_state.set(HammerState::Game);
        }
    }
}
#[cfg(feature = "handle-physics")]
fn build_colliders(prim_mesh: Mesh) -> (Vec<Vec3>, Vec<[u32; 3]>) {
    let (vert_buffer, idx_buffer) = (prim_mesh.attributes(), prim_mesh.indices().unwrap());
    let mut vertices: Vec<Vec3> = vec![];
    for (_, verts) in vert_buffer.into_iter().enumerate() {
        if let Some(verts) = verts.1.as_float3() {
            for vert in verts {
                vertices.push(Vec3::new(vert[0], vert[1], vert[2]));
            }
        }
    }
    let mut indices: Vec<[u32; 3]> = vec![];
    match idx_buffer {
        Indices::U32(x) => {
            for (_, iter) in x.chunks(3).enumerate() {
                indices.push(iter.try_into().unwrap());
            }
        }
        Indices::U16(x) => {
            for (_, iter) in x.chunks(3).enumerate() {
                indices.push([iter[0] as u32, iter[1] as u32, iter[2] as u32]);
            }
        }
    }
    (vertices, indices)
}

#[cfg(feature = "handle-physics")]
pub fn get_collision_data(
    base_mesh: &String,
    gltf: &Gltf,
    asset_nodes: &Res<Assets<GltfNode>>,
    asset_meshes: &Res<Assets<GltfMesh>>,
    meshes: &ResMut<Assets<Mesh>>,
) -> (Vec<Vec3>, Vec<[u32; 3]>) {
    let collider_node = &gltf.named_nodes[&(base_mesh.to_owned() + "_collider")];
    let coll_mesh = asset_nodes
        .get(collider_node.id())
        .unwrap()
        .mesh
        .as_ref()
        .unwrap();
    let prim_mesh = meshes
        .get(
            asset_meshes.get(coll_mesh).unwrap().primitives[0]
                .mesh
                .clone(),
        )
        .unwrap()
        .to_owned();
    build_colliders(prim_mesh)
}

fn placehold_texture(
    prefab_name: &str,
    texture_type: &str,
    asset_server: &Res<AssetServer>,
) -> Option<Handle<Image>> {
    let tex =
        asset_server.load("textures/".to_string() + prefab_name + "_" + texture_type + ".png");
    match asset_server.load_state(tex.id()) {
        LoadState::Failed => None,
        _ => Some(tex),
    }
}
