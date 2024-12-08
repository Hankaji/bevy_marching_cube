use std::{
    collections::{hash_map::Entry::Vacant, HashMap},
    thread::sleep,
    time::{Duration, Instant},
};

use bevy::{
    ecs::world::{CommandQueue, OccupiedEntry},
    prelude::*,
    render::{
        mesh::{Indices, PrimitiveTopology},
        render_asset::RenderAssetUsages,
    },
    tasks::{block_on, futures_lite::future, AsyncComputeTaskPool, ComputeTaskPool, Task},
    utils::warn,
};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::{
    map_generator::map_display::march_cube, player::Player, settings::render::RenderSettings,
};

use super::{
    map_display::{RenderChunk, RenderMode},
    MapGenerator,
};

pub const CHUNK_SIZE: u8 = 16;

pub struct EndlessTerrainPlugin;

impl Plugin for EndlessTerrainPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ChunkMap>()
            .add_systems(Update, (update_visible_chunks, update_chunk, handle_tasks));
    }
}

#[derive(Debug, Default, Resource)]
pub struct ChunkMap(pub HashMap<IVec3, Chunk>);

#[derive(Component)]
struct ComputeTransform(Task<CommandQueue>);

fn update_visible_chunks(
    mut commands: Commands,
    mut chunk_map: ResMut<ChunkMap>,
    render_cfg: Res<RenderSettings>,
    player_pos_q: Query<&Transform, With<Player>>,
) {
    let render_distance = render_cfg.render_distance;

    let Ok(player_t) = player_pos_q.get_single() else {
        warn(Err("Could not get Transform from player"));
        return;
    };

    let player_coord: Vec3 = player_t.translation;

    let curr_chunk_coord_x = (player_coord.x / CHUNK_SIZE as f32).floor() as i32;
    let curr_chunk_coord_y = (player_coord.y / CHUNK_SIZE as f32).floor() as i32;
    let curr_chunk_coord_z = (player_coord.z / CHUNK_SIZE as f32).floor() as i32;

    // Loop through all chunks in render_distance
    // and add them to chunk_map
    let thread_pool = AsyncComputeTaskPool::get();
    println!("--------------------------------------------------");
    let time = Instant::now();
    let rd_xz = render_distance.0 as i32;
    let rd_y = render_distance.1 as i32;
    for y in -rd_y..=rd_y {
        for x in -rd_xz..=rd_xz {
            for z in -rd_xz..=rd_xz {
                let viewed_chunk_coord = IVec3::new(
                    curr_chunk_coord_x + x,
                    curr_chunk_coord_y + y,
                    curr_chunk_coord_z + z,
                );

                // if let Vacant(e) = chunk_map.0.entry(viewed_chunk_coord) {
                //     e.insert(Chunk::new());
                //     commands.add(RenderChunk::new(
                //         viewed_chunk_coord,
                //         RenderMode::MarchingCube,
                //     ));
                // }

                if let Vacant(e) = chunk_map.0.entry(viewed_chunk_coord) {
                    e.insert(Chunk::new());
                } else {
                    continue;
                }

                let entity = commands.spawn_empty().id();
                let task = thread_pool.spawn(async move {
                    let mut cmd_queue = CommandQueue::default();
                    cmd_queue.push(move |world: &mut World| {
                        let voxel_grid = world
                            .get_resource::<MapGenerator>()
                            .expect("Could not find MapGenerator")
                            .generate_noise(viewed_chunk_coord, 16);

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

                        world
                            .entity_mut(entity)
                            .insert(PbrBundle {
                                mesh: triangle_mesh,
                                transform: Transform::from_translation(
                                    viewed_chunk_coord.as_vec3() * CHUNK_SIZE as f32,
                                ),
                                material,
                                ..default()
                            })
                            // Task is complete, so remove task component from entity
                            .remove::<ComputeTransform>();
                    });

                    cmd_queue
                });

                commands.entity(entity).insert(ComputeTransform(task));
            }
        }
    }
    let elapsed = time.elapsed();
    println!("Time generated chunks (8x8x8): {:.2?}", elapsed);
    // panic!("\nThis panic is intended so please don't panic\n");
}

fn handle_tasks(mut commands: Commands, mut gen_tasks: Query<&mut ComputeTransform>) {
    for mut task in &mut gen_tasks {
        if let Some(mut cmd_queue) = block_on(future::poll_once(&mut task.0)) {
            commands.append(&mut cmd_queue);
        }
    }
}

#[derive(Debug)]
pub struct Chunk {
    pub visible: bool,
}

impl Chunk {
    pub fn new() -> Self {
        Self { visible: false }
    }
}

pub(super) fn update_chunk(
    mut chunk_map: ResMut<ChunkMap>,
    render_cfg: Res<RenderSettings>,
    player_pos_q: Query<&Transform, With<Player>>,
) {
    let Ok(player_t) = player_pos_q.get_single() else {
        return;
    };

    let curr_chunk_coord = (player_t.translation / CHUNK_SIZE as f32)
        .floor()
        .as_ivec3();

    let (render_distance_xz, render_distance_y) =
        (render_cfg.render_distance.0, render_cfg.render_distance.1);

    for (chunk_coord, chunk) in chunk_map.0.iter_mut() {
        let distance = (curr_chunk_coord - *chunk_coord).abs().as_uvec3();
        let distance_xz = u32::max(distance.x, distance.z);
        chunk.visible = distance_xz <= render_distance_xz && distance.y <= render_distance_y;
    }
}
