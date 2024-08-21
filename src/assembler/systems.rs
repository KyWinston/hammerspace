use bevy::{gltf::*, prelude::*, render::mesh::Indices};

use super::resources::MeshAssets;
use crate::assembler::components::LevelTerrain;

pub fn build_collider(prim_mesh: Mesh) -> (Vec<Vec3>, Vec<[u32; 3]>) {
    let (vert_buffer, idx_buffer) = (prim_mesh.attributes(), prim_mesh.indices().unwrap());
    let mut vertices: Vec<Vec3> = vec![];
    for verts in vert_buffer.into_iter() {
        if let Some(verts) = verts.1.as_float3() {
            for vert in verts {
                vertices.push(Vec3::new(vert[0], vert[1], vert[2]));
            }
        }
    }
    let mut indices: Vec<[u32; 3]> = vec![];
    match idx_buffer {
        Indices::U32(x) => {
            for iter in x.chunks(3) {
                indices.push(iter.try_into().unwrap());
            }
        }
        Indices::U16(x) => {
            for iter in x.chunks(3) {
                indices.push([iter[0] as u32, iter[1] as u32, iter[2] as u32]);
            }
        }
    }
    (vertices, indices)
}

pub fn setup_level(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mesh_assets: Res<MeshAssets>,
    gltfs: Res<Assets<Gltf>>,
    nodes: Res<Assets<GltfNode>>,
    gltf_mesh: Res<Assets<GltfMesh>>,
) {
    info!("setting up level");
    let objects_to_spawn = vec![(
        "level".to_string(),
        Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::from_array([1.0, 1.0, 1.0])),
    )];

    commands.spawn(DirectionalLightBundle::default());

    for object_to_spawn in objects_to_spawn {
        let gltf = gltfs
            .get(mesh_assets.0.get(&object_to_spawn.0).unwrap())
            .unwrap();
        if object_to_spawn.0 == "level" {
            for level_obj in unpack_gltf(
                gltf,
                &gltfs,
                &nodes,
                &gltf_mesh,
                &mut materials,
                &mesh_assets,
            ) {
                commands.spawn(level_obj);
            }
            for ex_node in gltf.nodes.clone() {
                let node = nodes.get(ex_node.id());
                if let Some(node) = node {
                    if node.name.contains("Light") {
                        commands.spawn(PointLightBundle {
                            point_light: PointLight::default(),
                            transform: node.transform,
                            global_transform: node.transform.into(),
                            ..default()
                        });
                    }
                }
            }
        }
    }
}

fn unpack_gltf(
    gltf: &Gltf,
    gltfs: &Res<Assets<Gltf>>,
    nodes: &Res<Assets<GltfNode>>,
    gltf_mesh: &Res<Assets<GltfMesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    mesh_assets: &Res<MeshAssets>,
) -> Vec<(MaterialMeshBundle<StandardMaterial>, LevelTerrain)> {
    let mut unpacked_gltf = vec![];
    for mesh in gltf.nodes.clone().into_iter() {
        if let Some(node_data) = nodes.get(mesh.id()) {
            if node_data.mesh.is_none() {
                continue;
            } else {
                let prim = node_data.mesh.clone().unwrap();
                let prefab_replacement =
                    substitute_prefab(&node_data.name, gltfs, &prim, mesh_assets);
                let replacement_prim = gltf_mesh.get(prefab_replacement.id()).unwrap();
                info!("{:?}", replacement_prim.name);
                unpacked_gltf.push((
                    MaterialMeshBundle {
                        mesh: replacement_prim.primitives[0].mesh.clone(),
                        transform: node_data.transform,
                        global_transform: node_data.transform.into(),
                        material: replacement_prim.primitives[0]
                            .clone()
                            .material
                            .unwrap_or_else(|| materials.add(StandardMaterial::default())),
                        ..default()
                    },
                    LevelTerrain,
                ));
            }
        }
    }
    unpacked_gltf
}

fn substitute_prefab(
    name: &String,
    gltfs: &Res<Assets<Gltf>>,
    mesh: &Handle<GltfMesh>,
    mesh_assets: &Res<MeshAssets>,
) -> Handle<GltfMesh> {
    let mut new_mesh = mesh.clone();
    for asset in mesh_assets.0.clone() {
        if asset.0 == *name {
            new_mesh = gltfs.get(asset.1.id()).unwrap().meshes[0].clone();
        }
    }
    new_mesh
}
