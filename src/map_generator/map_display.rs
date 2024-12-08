use std::{
    cell::{Cell, RefCell},
    default,
    sync::{Arc, Mutex, RwLock},
    thread::sleep,
    time::Duration,
};

use bevy::{
    asset::Assets,
    color::palettes::css::{BLUE, GREEN, PURPLE},
    ecs::world::Command,
    math::{f32, u32, IVec3, Vec3},
    pbr::PbrBundle,
    prelude::*,
    reflect::List,
    render::{
        mesh::{Indices, MeshVertexAttribute, PrimitiveTopology, VertexAttributeValues},
        render_asset::RenderAssetUsages,
    },
    utils::hashbrown::hash_set::IntoIter,
};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use super::{
    endless_terrain::CHUNK_SIZE,
    marching_table::{EDGES, TRIANGULATIONS, VERTICES},
    noise_generator::{Noise, VoxelGrid},
    MapGenerator,
};

/// Perform a linear interpolation along the edge
fn interpolate_verts(p1: Vec3, p2: Vec3, s1: f32, s2: f32, isovalue: f32) -> Vec3 {
    let t = (isovalue - s1) / (s2 - s1);
    p1 + (t * (p2 - p1))
}

pub fn march_cube(
    (x, y, z): (usize, usize, usize),
    voxel_grid: &VoxelGrid,
    positions: &mut Vec<Vec3>,
    color: &mut Vec<Vec4>,
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
        let corner_pos_a = UVec3::new((x + xa) as u32, (y + ya) as u32, (z + za) as u32);
        let corner_pos_b = UVec3::new((x + xb) as u32, (y + yb) as u32, (z + zb) as u32);

        // NOTE: TESTING COLOR
        let color_a_value = voxel_grid.read(
            corner_pos_a.x as usize,
            corner_pos_a.y as usize,
            corner_pos_a.z as usize,
        );
        let color_b_value = voxel_grid.read(
            corner_pos_b.x as usize,
            corner_pos_b.y as usize,
            corner_pos_b.z as usize,
        );
        let (color_a_value, color_b_value) = (
            voxel_grid.normalize_value(color_a_value),
            voxel_grid.normalize_value(color_b_value),
        );
        let color_value = (color_a_value + color_b_value) / 2.0;
        // color.push(Vec4::from((Vec3::splat(color_value), 1.0)));
        color.push(match color_value {
            // ..0.2 => BLUE.to_vec4(),
            _ => GREEN.to_vec4(),
        });

        // Read value of 2 points
        let sa = voxel_grid.read(
            corner_pos_a.x as usize,
            corner_pos_a.y as usize,
            corner_pos_a.z as usize,
        );
        let sb = voxel_grid.read(
            corner_pos_b.x as usize,
            corner_pos_b.y as usize,
            corner_pos_b.z as usize,
        );

        // get interpolation point
        let midpoint =
            interpolate_verts(corner_pos_a.as_vec3(), corner_pos_b.as_vec3(), sa, sb, 0.0);

        // Find the midpoint of the edge
        // let midpoint = (corner_pos_a + corner_pos_b).as_vec3() / 2.0;

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

#[derive(Debug, Default)]
pub struct RenderChunk {
    chunk_coord: IVec3,
    render_modes: Vec<RenderMode>,
}

#[derive(Debug, Default)]
pub enum RenderMode {
    #[default]
    MarchingCube,
    Voxel,
    DataPoint,
    TextData,
}

impl IntoIterator for RenderMode {
    type Item = RenderMode;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        vec![self].into_iter()
    }
}

impl RenderChunk {
    pub fn new(chunk_coord: IVec3, render_modes: impl IntoIterator<Item = RenderMode>) -> Self {
        Self {
            chunk_coord,
            render_modes: render_modes.into_iter().collect(),
        }
    }
}

impl Command for RenderChunk {
    fn apply(self, world: &mut bevy::prelude::World) {
        let voxel_grid = world
            .get_resource::<MapGenerator>()
            .expect("Could not find MapGenerator")
            .generate_noise(self.chunk_coord, 16);

        let size = voxel_grid.size;

        // March each cube in world
        let mut vertices: Vec<Vec3> = Vec::new();
        let mut color: Vec<Vec4> = Vec::with_capacity(vertices.len());

        for z in 0..(size - 1) {
            for y in 0..(size - 1) {
                for x in 0..(size - 1) {
                    march_cube((x, y, z), &voxel_grid, &mut vertices, &mut color);
                }
            }
        }

        let mut mesh = Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::RENDER_WORLD,
        );

        let mut normals: Vec<Vec3> = Vec::with_capacity(vertices.len());
        let mut indices: Vec<u32> = Vec::with_capacity(vertices.len());
        for i in 0..(vertices.len() / 3) {
            let i = i as u32 * 3;
            indices.push(i + 2);
            indices.push(i + 1);
            indices.push(i);

            let i = i as usize;
            let a = vertices[i + 1] - vertices[i + 2];
            let b = vertices[i] - vertices[i + 2];
            let normal = a.cross(b);
            normals.push(normal);
            normals.push(normal);
            normals.push(normal);
        }

        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
        mesh.insert_indices(Indices::U32(indices));
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, color);

        let triangle_mesh = world
            .get_resource_mut::<Assets<Mesh>>()
            .expect("Cant find assets for 'Mesh'")
            .add(mesh);

        let material = world
            .get_resource_mut::<Assets<StandardMaterial>>()
            .expect("Cant find assets for 'Material'")
            .add(Color::srgb(1., 1., 1.));

        world.spawn(PbrBundle {
            mesh: triangle_mesh,
            transform: Transform::from_translation(self.chunk_coord.as_vec3() * CHUNK_SIZE as f32),
            material,
            ..default()
        });
    }
}
