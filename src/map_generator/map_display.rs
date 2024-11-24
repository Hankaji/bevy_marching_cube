use bevy::{
    asset::Assets,
    ecs::world::Command,
    math::{u32, IVec3, Vec3},
    pbr::PbrBundle,
    prelude::*,
    reflect::List,
    render::{
        mesh::{Indices, PrimitiveTopology},
        render_asset::RenderAssetUsages,
    },
};

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

pub struct RenderChunk {
    chunk_coord: IVec3,
    voxel_grid: VoxelGrid,
}

impl RenderChunk {
    pub fn new(coord: IVec3) -> Self {
        Self {
            chunk_coord: coord,
            voxel_grid: VoxelGrid::new(16),
        }
    }
}

impl Command for RenderChunk {
    fn apply(self, world: &mut bevy::prelude::World) {
        let voxel_grid = world
            .get_resource::<VoxelGrid>()
            .expect("Voxel Grid not found");

        let size = voxel_grid.size;

        // March each cube in world
        let mut positions: Vec<Vec3> = Vec::new();

        for z in 0..(size - 1) {
            for y in 0..(size - 1) {
                for x in 0..(size - 1) {
                    march_cube((x, y, z), voxel_grid, &mut positions);
                }
            }
        }

        // render triangular mesh in the world
        for i in 0..(positions.len() / 3) {
            let i = i * 3;
            if positions.get(i).is_none() {
                warn!("Cant find entry at index i = {i}");
                break;
            }

            let triangle_mesh = world
                .get_resource_mut::<Assets<Mesh>>()
                .expect("Cant find assets for 'Mesh'")
                .add(Triangle3d::new(
                    positions[i + 2],
                    positions[i + 1],
                    positions[i],
                ));

            let material = world
                .get_resource_mut::<Assets<StandardMaterial>>()
                .expect("Cant find assets for 'Material'")
                .add(Color::srgb_u8(0, 250, 0));

            world.spawn(PbrBundle {
                mesh: triangle_mesh,
                material,
                ..default()
            });
        }
    }
}
