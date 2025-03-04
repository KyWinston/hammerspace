use rand::Rng;
use std::sync::{Arc, RwLock};

use bevy::{gltf::GltfMesh, prelude::*, render::mesh::Mesh};
use vleue_navigator::NavMesh;

use super::{components::FindingPath, events::PathEvent};

// pub fn get_path(
//     mut commands: Commands,
//     mut handles: ResMut<Handles>,
//     gltfs: Res<Assets<Gltf>>,
//     gltf_meshes: Res<Assets<GltfMesh>>,
//     meshes: Res<Assets<Mesh>>,
//     mut navmeshes: ResMut<Assets<NavMesh>>,
//     mut path_ev: EventReader<PathEvent>,
// ) {
//     for ev in path_ev.read() {
//         if handles.0.is_none() {
//             return;
//         }
//         if handles.1.is_none() {
//             // Get the gltf struct loaded from the file
//             let Some(gltf) = gltfs.get(handles.0.as_mut().unwrap()) else {
//                 return;
//             };
//             // Get the mesh called `navmesh`
//             let Some(gltf_mesh) = gltf_meshes.get(&gltf.named_meshes["navmesh"]) else {
//                 return;
//             };
//             // Get the actual mesh
//             let Some(mesh) = meshes.get(&gltf_mesh.primitives[0].mesh) else {
//                 return;
//             };
//             // Build a `NavMesh` from that mesh, then save it as an asset
//             handles.1 = Some(navmeshes.add(NavMesh::from_bevy_mesh(mesh)));
//         } else {
//             // Get the navmesh, then search for a path
//             let Some(navmesh) = navmeshes.get(handles.1.as_ref().unwrap()) else {
//                 return;
//             };

//             let from = ev.1;
//             let to = ev.2;
//             let finding = FindingPath(Arc::new(RwLock::new((
//                 navmesh.path(from.xz(), to.xz()),
//                 false,
//             ))));
//             commands.entity(ev.0).insert(finding);
//         }
//     }
// }

// pub fn pick_point_in_range(center: Vec3, range: (Vec3, Vec3)) -> Vec3 {
//     let range_x = rand::thread_rng().gen_range(center.x - range.0.x..center.x + range.1.x);
//     let range_y = rand::thread_rng().gen_range(center.y - range.0.y..center.y + range.1.y);
//     let range_z = rand::thread_rng().gen_range(center.z - range.0.x..center.x + range.1.z);
//     Vec3::new(range_x, range_y, range_z)
// }
