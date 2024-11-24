use std::collections::HashMap;

use bevy::{prelude::*, utils::warn};

use crate::player::Player;

pub const CHUNK_SIZE: u8 = 15;

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
    mut chunk_map: ResMut<ChunkMap>,
    player_pos_q: Query<&Transform, With<Player>>,
) {
    let render_distance = (1, 0);

    let Ok(player_t) = player_pos_q.get_single() else {
        warn(Err("Could not get Transform from player"));
        return;
    };

    let player_coord: Vec3 = player_t.translation;

    let curr_chunk_coord_x = (player_coord.x / CHUNK_SIZE as f32).floor() as i32;
    let curr_chunk_coord_y = (player_coord.y / CHUNK_SIZE as f32).floor() as i32;
    let curr_chunk_coord_z = (player_coord.z / CHUNK_SIZE as f32).floor() as i32;

    let rd_xz = render_distance.0;
    let rd_y = render_distance.1;
    for y in -rd_y..=rd_y {
        for x in -rd_xz..=rd_xz {
            for z in -rd_xz..=rd_xz {
                let viewed_chunk_coord = IVec3::new(
                    curr_chunk_coord_x + x,
                    curr_chunk_coord_y + y,
                    curr_chunk_coord_z + z,
                );

                chunk_map
                    .0
                    .entry(viewed_chunk_coord)
                    .or_insert(Chunk::new(viewed_chunk_coord));
            }
        }
    }
}

#[derive(Debug)]
pub struct Chunk {
    pos: IVec3,
    pub visible: bool,
}

impl Chunk {
    pub fn new(coord: IVec3) -> Self {
        Self {
            pos: coord * (CHUNK_SIZE as i32),
            visible: false,
        }
    }
}

pub(super) fn update_chunk(
    mut chunk_map: ResMut<ChunkMap>,
    player_pos_q: Query<&Transform, With<Player>>,
) {
    let Ok(player_t) = player_pos_q.get_single() else {
        return;
    };

    let curr_chunk_coord = (player_t.translation / CHUNK_SIZE as f32)
        .floor()
        .as_ivec3();

    for (chunk_coord, chunk) in chunk_map.0.iter_mut() {
        let distance = (curr_chunk_coord - *chunk_coord).abs();
        let distance_xz = i32::max(distance.x, distance.z);
        chunk.visible = distance_xz <= 1 && distance.y <= 0;
    }
}
