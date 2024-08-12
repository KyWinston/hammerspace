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
pub struct Setpiece {
    pub dynamic: bool,
}

#[derive(Bundle)]
pub struct PrefabBundle {
    rendered_mesh: PbrBundle,
    collision_mesh: Handle<Mesh>,
    setpiece: Setpiece,
}

impl PrefabBundle {
    pub fn new(dynamic: bool, mesh: Handle<Mesh>, material: Handle<StandardMaterial>) -> Self {
        Self {
            rendered_mesh: PbrBundle {
                mesh: mesh.clone(),
                material,
                ..default()
            },
            collision_mesh: mesh,
            setpiece: Setpiece { dynamic },
        }
    }
}
