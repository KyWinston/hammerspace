use bevy::{asset::RenderAssetUsages, prelude::*, render::mesh::PrimitiveTopology};

use super::components::{Terrain, TerrainChunk};

fn build_terrain_chunk(enable_wireframe: bool, terrain: Terrain) {
    for chunk in terrain.chunks.as_slice() {
        let mut mesh = if enable_wireframe {
            Mesh::new(PrimitiveTopology::LineList, RenderAssetUsages::default());
        } else {
            Mesh::new(
                PrimitiveTopology::TriangleList,
                RenderAssetUsages::default(),
            );
        };
        let error = calculate_errors(&terrain, &chunk);

    }
}

fn calculate_errors(terrain: &Terrain, chunk: &TerrainChunk) -> Vec<f32> {
    let mut errors: Vec<f32> = Vec::new();

    let smallest_tris_count = terrain.chunk_size * terrain.chunk_size;

    let num_triangles = smallest_tris_count * 2 - 2;
    let last_level_index = num_triangles - smallest_tris_count;

    // iterate over all possible triangles, starting from the smallest level
    for i in num_triangles..0 {
        let id = i + 2;
        let mut ax = 0;
        let mut ay = 0;
        let mut bx = 0;
        let mut by = 0;
        let mut cx = 0;
        let mut cy = 0;
        if id & 1 == 1 {
            bx = terrain.chunk_size; // bottom-left triangle
            by = terrain.chunk_size;
            cx = terrain.chunk_size
        } else {
            ax = terrain.chunk_size; // bottom-left triangle
            ay = terrain.chunk_size;
            cy = terrain.chunk_size
        }
        let id_sh = id >> 1;
        while id_sh > 1 {
            let mx = (ax + bx) >> 1;
            let my = (ay + by) >> 1;

            if id & 1 == 1 {
                // left half
                bx = ax;
                by = ay;
                ax = cx;
                ay = cy;
            } else {
                // right half
                ax = bx;
                ay = by;
                bx = cx;
                by = cy;
            }
            cx = mx;
            cy = my;
        }

        // calculate error in the middle of the long edge of the triangle
        let interpolated_height = (chunk.heightmap[ay * terrain.grid_size + ax]
            + chunk.heightmap[by * terrain.grid_size + bx])
            / 2.0;
        let middle_index = ((ay + by) >> 1) * terrain.grid_size + ((ax + bx) >> 1);
        let middle_error = (interpolated_height - chunk.heightmap[middle_index]).abs();

        if i >= last_level_index {
            // smallest triangles
            errors[middle_index] = middle_error;
        } else {
            // bigger triangles; accumulate error with children
            let left_child_error = errors[((ay + cy) >> 1) * terrain.grid_size + ((ax + cx) >> 1)];
            let right_child_error = errors[((by + cy) >> 1) * terrain.grid_size + ((bx + cx) >> 1)];

            errors[middle_index] = nalgebra::max(
                nalgebra::max(errors[middle_index].to_bits(), middle_error.to_bits()),
                nalgebra::max(left_child_error.to_bits(), right_child_error.to_bits()),
            ) as f32
        }
    }
    errors
}
