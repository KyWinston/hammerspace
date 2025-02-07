use bevy::prelude::*;

pub struct TerrainChunk {
    pub id: usize,
    pub vertices: Vec<Vec3>,
    pub indices: Vec<u32>,
    pub heightmap: Vec<f32>,
}

#[derive(Component)]
pub struct Terrain {
    pub chunks: Vec<TerrainChunk>,
    pub chunk_size: usize,
    pub grid_size: usize,
}
