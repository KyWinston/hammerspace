use bevy::prelude::*;

#[derive(Component)]
pub struct MaterialPending;

#[derive(Component, Default)]
pub struct Setpiece {
    pub dynamic: bool,
}

#[derive(Component)]
pub struct LevelTerrain;

#[derive(Component)]
#[require(MeshMaterial3d<StandardMaterial>, Setpiece)]
pub struct Prefab {
    pub rendered_mesh: MeshMaterial3d<StandardMaterial>,
    pub collision_mesh: Mesh3d,
}
