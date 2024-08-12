use avian3d::prelude::{Collider, RigidBody};
use bevy::prelude::*;

#[derive(Component)]
pub struct MaterialPending;

#[derive(Component)]
pub struct SfxEmitter {
   pub sound: String,
   pub intensity: f32,
   pub looped: bool,
}

#[derive(Component)]
pub struct Setpiece;

#[derive(Bundle)]
pub struct PrefabBundle {
    rendered_mesh: PbrBundle,
    collider: Collider,
    rigid_body_type: RigidBody,
    setpiece: Setpiece,
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
            setpiece: Setpiece,
        }
    }
}
