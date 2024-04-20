use bevy::prelude::*;

#[cfg(feature = "handle-physics")]
use bevy_rapier3d::{dynamics::RigidBody, geometry::Collider};

#[derive(Component)]
pub struct Setpiece;

#[derive(Bundle)]
pub struct PrefabBundle {
    rendered_mesh: PbrBundle,
    #[cfg(feature = "handle-physics")]
    collider: Collider,
    #[cfg(feature = "handle-physics")]
    rigid_body_type: RigidBody,
    setpiece: Setpiece,
}

impl PrefabBundle {
    #[cfg(feature = "handle-physics")]
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
            #[cfg(feature = "handle-physics")]
            collider: Collider::trimesh(verts, indices),
            #[cfg(feature = "handle-physics")]
            rigid_body_type,
            setpiece: Setpiece,
        }
    }
    #[cfg(not(feature = "handle-physics"))]
    pub fn new(mesh: Handle<Mesh>, material: Handle<StandardMaterial>) -> Self {
        Self {
            rendered_mesh: PbrBundle {
                mesh,
                material,
                ..default()
            },
            setpiece: Setpiece,
        }
    }
}
