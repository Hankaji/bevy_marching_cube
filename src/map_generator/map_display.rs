use bevy::{
    asset::Assets,
    color::palettes::css::ORANGE,
    ecs::world::Command,
    math::{u32, IVec3, Vec3},
    pbr::PbrBundle,
    prelude::*,
    reflect::List,
    render::{
        mesh::{Indices, MeshVertexAttribute, PrimitiveTopology},
        render_asset::RenderAssetUsages,
        render_resource::ShaderType,
    },
};
use bevy_mod_billboard::BillboardTextBundle;
use rand::Rng;

use super::{
    endless_terrain::CHUNK_SIZE,
    marching_table::{EDGES, TRIANGULATIONS, VERTICES},
    noise_generator::{Noise, VoxelGrid},
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
}

impl RenderChunk {
    pub fn new(chunk_coord: IVec3) -> Self {
        Self { chunk_coord }
    }
}

impl Command for RenderChunk {
    fn apply(self, world: &mut bevy::prelude::World) {
        let voxel_grid = Noise::generate_noise_map(
            CHUNK_SIZE as usize + 1,
            self.chunk_coord,
            2.0,
            297543,
            3,
            0.5,
            2.0,
        );
        let size = voxel_grid.size;

        // let cuboid_mesh = world
        //     .get_resource_mut::<Assets<Mesh>>()
        //     .expect("Cant find assets for 'Mesh'")
        //     .add(Cuboid::from_length(0.2));

        // let mut rand = rand::thread_rng();
        // let rand_color = Color::srgb(
        //     rand.gen_range(0.0..1.0),
        //     rand.gen_range(0.0..1.0),
        //     rand.gen_range(0.0..1.0),
        // );

        // for z in 0..size {
        //     for y in 0..size {
        //         for x in 0..size {
        //             if voxel_grid.read(x, y, z) > 0.0 {
        //                 continue;
        //             }
        //
        //             let material = world
        //                 .get_resource_mut::<Assets<StandardMaterial>>()
        //                 .expect("Cant find assets for 'Material'")
        //                 .add(Color::srgb(
        //                     voxel_grid.read(x, y, z),
        //                     voxel_grid.read(x, y, z),
        //                     voxel_grid.read(x, y, z),
        //                 ));
        //
        //             // Cubes
        //             world.spawn(PbrBundle {
        //                 mesh: cuboid_mesh.clone(),
        //                 material,
        //                 transform: Transform::from_translation(
        //                     Vec3::new(x as f32, y as f32, z as f32)
        //                         + (self.chunk_coord * 15).as_vec3(),
        //                 ),
        //                 ..default()
        //             });
        //
        //             // world.spawn(BillboardTextBundle {
        //             //     transform: Transform::from_xyz(x as f32, y as f32 + 0.1, z as f32)
        //             //         .with_scale(Vec3::splat(0.0015) + (self.chunk_coord * 15).as_vec3()),
        //             //     text: Text::from_sections([TextSection {
        //             //         // value: color.to_string(),
        //             //         value: format!("[{x} {y} {z}] | {}", voxel_grid.read(x, y, z)),
        //             //         style: TextStyle {
        //             //             font_size: 60.0,
        //             //             color: ORANGE.into(),
        //             //             ..Default::default()
        //             //         },
        //             //     }]),
        //             //     ..Default::default()
        //             // });
        //         }
        //     }
        // }

        // March each cube in world
        let mut vertices: Vec<Vec3> = Vec::new();

        for z in 0..(size - 1) {
            for y in 0..(size - 1) {
                for x in 0..(size - 1) {
                    march_cube((x, y, z), &voxel_grid, &mut vertices);
                }
            }
        }

        let mut mesh = Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::RENDER_WORLD,
        );

        let mut normals: Vec<Vec3> = Vec::with_capacity(vertices.len() / 3);

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

        let triangle_mesh = world
            .get_resource_mut::<Assets<Mesh>>()
            .expect("Cant find assets for 'Mesh'")
            .add(mesh);

        let material = world
            .get_resource_mut::<Assets<StandardMaterial>>()
            .expect("Cant find assets for 'Material'")
            .add(Color::srgb_u8(0, 250, 0));

        world.spawn(PbrBundle {
            mesh: triangle_mesh,
            transform: Transform::from_translation(self.chunk_coord.as_vec3() * CHUNK_SIZE as f32),
            material,
            ..default()
        });

        // render triangular mesh in the world
        // for i in 0..(vertices.len() / 3) {
        //     let i = i * 3;
        //     if vertices.get(i).is_none() {
        //         warn!("Cant find entry at index i = {i}");
        //         break;
        //     }
        //
        // let triangle_mesh = world
        //     .get_resource_mut::<Assets<Mesh>>()
        //     .expect("Cant find assets for 'Mesh'")
        //     .add(Triangle3d::new(
        //         vertices[i + 2],
        //         vertices[i + 1],
        //         vertices[i],
        //     ));
        //
        // let material = world
        //     .get_resource_mut::<Assets<StandardMaterial>>()
        //     .expect("Cant find assets for 'Material'")
        //     .add(Color::srgb_u8(0, 250, 0));
        //
        // world.spawn(PbrBundle {
        //     mesh: triangle_mesh,
        //     transform: Transform::from_translation(
        //         self.chunk_coord.as_vec3() * CHUNK_SIZE as f32,
        //     ),
        //     material,
        //     ..default()
        // });
        // }
    }
}
