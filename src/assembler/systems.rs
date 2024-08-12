use avian3d::collision::Collider;
use bevy::{
    gltf::{Gltf, GltfMesh, GltfNode},
    prelude::*,
    render::mesh::Indices,
};

use super::{components::SfxEmitter, resources::MeshAssets};

pub fn assemble_collider(
    gltf: &Gltf,
    gltf_nodes: &Res<Assets<GltfNode>>,
    meshes: &Res<Assets<Mesh>>,
    gltf_meshes: &Res<Assets<GltfMesh>>,
    // convex_hull: bool,
) -> Collider {
    Collider::compound(
        gltf.nodes
            .iter()
            .filter(|x| {
                !gltf_nodes
                    .get(*x)
                    .expect("Node not found for loaded gltf")
                    .mesh
                    .is_none()
            })
            .map(|x| {
                let collider: Collider;
                let prim = &gltf_meshes
                    .get(
                        &gltf_nodes
                            .get(x)
                            .expect("Node not found for loaded gltf")
                            .mesh
                            .clone()
                            .expect("Gltf Mesh not found in Node"),
                    )
                    .expect("Mesh not found for gltf mesh")
                    .primitives
                    .first()
                    .expect("No primitive found for mesh");
                let mesh = meshes
                    .get(&prim.mesh.clone())
                    .expect("No Mesh found for GLTF mesh");

                let (verts, indices) = build_colliders(mesh.clone());
                // if convex_hull {
                collider = Collider::convex_decomposition(verts, indices);
                // } else {
                // collider = Collider::trimesh(verts, indices);
                // }
                (
                    gltf_nodes
                        .get(x)
                        .expect("Node not found for loaded gltf")
                        .transform
                        .translation,
                    gltf_nodes
                        .get(x)
                        .expect("Node not found for loaded gltf")
                        .transform
                        .rotation,
                    collider,
                )
            })
            .collect(),
    )
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

pub fn setup_level(
    mut commands: Commands,
    mesh_assets: ResMut<MeshAssets>,
    gltfs: Res<Assets<Gltf>>,
    _gltf_nodes: Res<Assets<GltfNode>>,
    _gltf_meshes: Res<Assets<GltfMesh>>,
    _meshes: Res<Assets<Mesh>>,
) {
    let mut objects_to_spawn: Vec<(String, Transform)> = Vec::new();

    objects_to_spawn.push((
        "furnace".to_string(),
        Transform::from_xyz(0.0, -1.5, -2.0).with_scale(Vec3::from_array([2.0, 2.0, 2.0])),
    ));

    for object_to_spawn in objects_to_spawn {
        let gltf = gltfs
            .get(mesh_assets.0.get(&object_to_spawn.0).unwrap())
            .unwrap();
        commands.spawn(DirectionalLightBundle::default());
        let mut obj = commands.spawn(SceneBundle {
            scene: gltf
                .default_scene
                .clone()
                .expect("Default scene not found for loaded gltf."),
            transform: object_to_spawn.1,
            ..default()
        });
        if object_to_spawn.0 == "furnace" {
            obj.insert(SfxEmitter {
                sound: "thrusterFire_000.ogg".into(),
                intensity: 1.0,
                looped: true,
            });
        }
        // .insert(assemble_collider(gltf, &gltf_nodes, &meshes, &gltf_meshes,false));
    }
}


