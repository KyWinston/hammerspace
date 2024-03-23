use bevy::{
    gltf::{Gltf, GltfMesh},
    prelude::*,
    render::mesh::Indices,
};
use bevy_basic_ui::AppState;
use bevy_rapier3d::{dynamics::RigidBody, geometry::Collider};

use crate::resources::LevelFolder;

use super::{events::LoadLevelEvent, resources::NextLevel};

pub fn fetch_level_handle(
    lvl_folder: Res<LevelFolder>,
    mut lvl_ev: EventReader<LoadLevelEvent>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut state: ResMut<NextState<AppState>>,
) {
    for ev in lvl_ev.read() {
        let path = lvl_folder.0.to_string() + "/" + &ev.0;
        let gltf_scene: Handle<Gltf> = asset_server.load(&path);
        commands.insert_resource(NextLevel(gltf_scene));
        state.set(AppState::Loading);
    }
}

pub fn assemble_level(
    mut commands: Commands,
    next_lvl: Res<NextLevel>,
    assets_meshes: Res<Assets<GltfMesh>>,
    assets_gltf: Res<Assets<Gltf>>,
    meshes: ResMut<Assets<Mesh>>,
    mut game_state: ResMut<NextState<AppState>>,
) {
    if let Some(gltf) = assets_gltf.get(&next_lvl.0) {
        for mesh_id in &gltf.named_meshes {
            let mesh = assets_meshes.get(&gltf.named_meshes[mesh_id.0]).unwrap();
            let (verts, indices) = get_collision_data(mesh_id.1, &assets_meshes, &meshes);
            commands.spawn((
                PbrBundle {
                    mesh: mesh.primitives[0].mesh.clone(),
                    material: mesh.primitives[0].material.clone().unwrap(),
                    ..default()
                },
                RigidBody::Fixed,
                Collider::trimesh(verts, indices),
            ));
        }
        game_state.set(AppState::Game);
    }
}

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

pub fn get_collision_data(
    mesh_handle: &Handle<GltfMesh>,
    gltf_meshes: &Res<Assets<GltfMesh>>,
    asset_mesh: &ResMut<Assets<Mesh>>,
) -> (Vec<Vec3>, Vec<[u32; 3]>) {
    let prim_mesh = asset_mesh
        .get(&gltf_meshes.get(mesh_handle).unwrap().primitives[0].mesh)
        .unwrap()
        .to_owned();
    build_colliders(prim_mesh)
}

pub fn assemble_textures(
    prefab_name: &str,
    asset_server: &Res<AssetServer>,
) -> (Handle<Image>, Handle<Image>, Handle<Image>, Handle<Image>) {
    let tex = asset_server.load("textures/".to_string() + prefab_name + "_diffuse.png");
    let metallic_tex = asset_server.load("textures/".to_string() + prefab_name + "_metallic.png");
    let ao_tex = asset_server.load("textures/".to_string() + prefab_name + "_ao.png");
    let emissive_tex = asset_server.load("textures/".to_string() + prefab_name + "_emissive.png");
    (tex, metallic_tex, ao_tex, emissive_tex)
}
