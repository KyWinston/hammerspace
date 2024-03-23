use bevy::prelude::*;
use bevy_rapier3d::{dynamics::RigidBody, geometry::Collider};

#[derive(Component)]
pub struct IsSetpiece;

#[derive(Bundle)]
pub struct PrefabBundle {
    rendered_mesh: PbrBundle,
    collider: Collider,
    rigid_body_type: RigidBody,
}

impl PrefabBundle {
    pub fn new(
        rigid_body_type: RigidBody,
        mesh: Handle<Mesh>,
        verts: Vec<Vec3>,
        indices: Vec<[u32; 3]>,
        material: Handle<StandardMaterial>,
    ) -> Self {
        Self {
            rendered_mesh: PbrBundle {
                mesh,
                material,
                ..default()
            },
            collider: Collider::trimesh(verts, indices),
            rigid_body_type,
        }
    }
}
