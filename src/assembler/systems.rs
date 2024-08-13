use bevy::{gltf::Gltf, prelude::*, render::mesh::Indices};

use super::{components::SfxEmitter, resources::MeshAssets};

pub fn build_colliders(prim_mesh: Mesh) -> (Vec<Vec3>, Vec<[u32; 3]>) {
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
    // gltf_nodes: Res<Assets<GltfNode>>,
    // gltf_meshes: Res<Assets<GltfMesh>>,
    // meshes: Res<Assets<Mesh>>,
) {
    info!("setting up level");
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
    }
}
