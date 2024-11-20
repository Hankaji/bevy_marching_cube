use bevy::math::Vec3;

use super::{
    marching_table::{EDGES, TRIANGULATIONS, VERTICES},
    noise_generator::VoxelGrid,
};

pub fn march_cube(
    (x, y, z): (usize, usize, usize),
    voxel_grid: &VoxelGrid,
    positions: &mut Vec<Vec3>,
) {
    let triangulation = get_triangulation((x, y, z), voxel_grid);

    for edge_idx in triangulation {
        if edge_idx.is_negative() {
            break;
        }

        // Get the 2 vertices' local position from edge
        let vertex_positions = EDGES[edge_idx as usize];

        // Get the vertex's position value
        let (xa, ya, za) = VERTICES[vertex_positions.0];
        let (xb, yb, zb) = VERTICES[vertex_positions.1];

        // Calculate the actual in-world position of 2 edge's vertices
        let pos_a = Vec3::new((x + xa) as f32, (y + ya) as f32, (z + za) as f32);
        let pos_b = Vec3::new((x + xb) as f32, (y + yb) as f32, (z + zb) as f32);

        // Find the midpoint of the edge
        let midpoint = (pos_a + pos_b) / 2.0;

        positions.push(midpoint);
    }
}

fn get_triangulation((x, y, z): (usize, usize, usize), voxel_grid: &VoxelGrid) -> [i8; 15] {
    let mut cube_idx = 0b00000000;

    #[allow(clippy::needless_range_loop)]
    for i in 0..8 {
        let offset = VERTICES[i];
        let (x, y, z) = (x + offset.0, y + offset.1, z + offset.2);
        let point_value = voxel_grid.read(x, y, z);

        // if value at pos is negative => asign 1 to the corresponding bit location
        cube_idx |= (point_value.is_sign_negative() as u8) << i;
    }

    TRIANGULATIONS[cube_idx as usize]
}
