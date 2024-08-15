use bevy::{gltf::*, prelude::*, render::mesh::Indices};

use super::{components::SfxEmitter, resources::MeshAssets};
use crate::assembler::components::LevelTerrain;

pub fn build_collider(prim_mesh: Mesh) -> (Vec<Vec3>, Vec<[u32; 3]>) {
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
    mut materials: ResMut<Assets<StandardMaterial>>,
    mesh_assets: Res<MeshAssets>,
    gltfs: Res<Assets<Gltf>>,
    nodes: Res<Assets<GltfNode>>,
    gltf_mesh: Res<Assets<GltfMesh>>,
) {
    info!("setting up level");
    let mut objects_to_spawn: Vec<(String, Transform)> = Vec::new();

    objects_to_spawn.push((
        "level".to_string(),
        Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::from_array([1.0, 1.0, 1.0])),
    ));

    objects_to_spawn.push((
        "furnace".to_string(),
        Transform::from_xyz(0.0, -1.5, -2.0).with_scale(Vec3::from_array([2.0, 2.0, 2.0])),
    ));
    commands.spawn(DirectionalLightBundle::default());

    for object_to_spawn in objects_to_spawn {
        let gltf = gltfs
            .get(mesh_assets.0.get(&object_to_spawn.0).unwrap())
            .unwrap();
        if object_to_spawn.0 == "level" {
            info!("binding colliders for level geometry");
            for level_obj in unpack_gltf(gltf, &nodes, &gltf_mesh, &mut materials) {
                commands.spawn(level_obj);
            }
        } else if object_to_spawn.0 == "furnace" {
            let mut obj = commands.spawn(SceneBundle {
                scene: gltf
                    .default_scene
                    .clone()
                    .expect("Default scene not found for loaded gltf."),
                transform: object_to_spawn.1,
                ..default()
            });
            obj.insert(SfxEmitter {
                sound: "thrusterFire_000.ogg".into(),
                intensity: 1.0,
                looped: true,
            });
        } else {
            commands.spawn(SceneBundle {
                scene: gltf
                    .default_scene
                    .clone()
                    .expect("Default scene not found for loaded gltf."),
                transform: object_to_spawn.1,
                ..default()
            });
        }
    }
}

fn unpack_gltf(
    gltf: &Gltf,
    nodes: &Res<Assets<GltfNode>>,
    gltf_mesh: &Res<Assets<GltfMesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) -> Vec<(MaterialMeshBundle<StandardMaterial>, LevelTerrain)> {
    let mut unpacked_gltf = vec![];
    for mesh in gltf.nodes.clone().into_iter() {
        if let Some(node_data) = nodes.get(mesh.id()) {
            // info!("{:?}", node_data.name);
            if let Some(prim) = gltf_mesh.get(node_data.mesh.clone().unwrap().id()) {
                unpacked_gltf.push((
                    MaterialMeshBundle {
                        mesh: prim.primitives[0].clone().mesh,
                        transform: node_data.transform,
                        material: materials.add(StandardMaterial { ..default() }),
                        ..default()
                    },
                    LevelTerrain,
                ));
            }
        }
    }
    unpacked_gltf
}
