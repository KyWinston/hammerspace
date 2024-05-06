use bevy::{
    gltf::{Gltf, GltfMesh, GltfNode},
    prelude::*,
};

use bevy_rapier3d::geometry::{Collider, ComputedColliderShape};

pub fn assemble_collider(
    gltf: &Gltf,
    gltf_nodes: &Res<Assets<GltfNode>>,
    meshes: &Res<Assets<Mesh>>,
    gltf_meshes: &Res<Assets<GltfMesh>>,
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
                    Collider::from_bevy_mesh(
                        meshes
                            .get(
                                gltf_meshes
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
                                    .expect("No primitive found for Mesh")
                                    .mesh
                                    .clone(),
                            )
                            .expect("No Mesh found for GLTF mesh"),
                        &ComputedColliderShape::ConvexHull,
                    )
                    .expect("Could not create collider from bevy mesh"),
                )
            })
            .collect(),
    )
}
