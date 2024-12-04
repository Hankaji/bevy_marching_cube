use std::{
    collections::{hash_map::Entry::Vacant, HashMap},
    time::Instant,
};

use bevy::{prelude::*, utils::warn};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::{player::Player, settings::render::RenderSettings};

use super::map_display::{RenderChunk, RenderMode};

pub const CHUNK_SIZE: u8 = 16;

pub struct EndlessTerrainPlugin;

impl Plugin for EndlessTerrainPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ChunkMap>()
            .add_systems(Update, (update_visible_chunks, update_chunk));
    }
}

#[derive(Debug, Default, Resource)]
pub struct ChunkMap(pub HashMap<IVec3, Chunk>);

fn update_visible_chunks(
    mut commands: Commands,
    mut commands_par: ParallelCommands,
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

                if let Vacant(e) = chunk_map.0.entry(viewed_chunk_coord) {
                    e.insert(Chunk::new());
                    commands.add(RenderChunk::new(
                        viewed_chunk_coord,
                        RenderMode::MarchingCube,
                    ));
                }
            }
        }
    }
    let elapsed = time.elapsed();
    println!("Time generated chunks (8x8x8): {:.2?}", elapsed);
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
