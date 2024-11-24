use bevy::{
    color::palettes::css::{BLUE, GREEN, RED, WHITE},
    prelude::*,
};
use f3_info::{toggle_text_visibility, update_curr_chunk, update_player_position};

use crate::map_generator::endless_terrain::{ChunkMap, CHUNK_SIZE};

mod f3_info;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.init_gizmo_group::<GizmoDebug>().add_systems(
            Update,
            (
                chunk_gizmos,
                toggle_text_visibility,
                update_curr_chunk,
                update_player_position,
            ),
        );
    }
}

#[derive(Default, Reflect, GizmoConfigGroup)]
struct GizmoDebug;

fn chunk_gizmos(mut gizmos: Gizmos<GizmoDebug>, chunk_map: Res<ChunkMap>) {
    let chunk_size = CHUNK_SIZE as f32;

    #[allow(clippy::never_loop)]
    for (chunk_coord, chunk) in chunk_map.0.iter() {
        let chunk_coord = chunk_coord.as_vec3() * chunk_size + (chunk_size / 2.0);
        gizmos.cuboid(
            Transform::from_translation(chunk_coord).with_scale(Vec3::splat(chunk_size)),
            match chunk.visible {
                true => WHITE,
                false => RED,
            },
        );
    }

    gizmos.arrow(Vec3::ZERO, Vec3::Y * chunk_size, GREEN);
    gizmos.arrow(Vec3::ZERO, Vec3::X * chunk_size, RED);
    gizmos.arrow(Vec3::ZERO, Vec3::Z * chunk_size, BLUE);
}
