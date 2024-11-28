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
        app.init_gizmo_group::<GizmoDebug>()
            .add_systems(Startup, setup)
            .add_systems(
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

fn setup(mut cfg_store: ResMut<GizmoConfigStore>) {
    let (gizmo_cfg, _) = cfg_store.config_mut::<DefaultGizmoConfigGroup>();
    gizmo_cfg.depth_bias = 0.1;
}

#[derive(Default, Reflect, GizmoConfigGroup)]
struct GizmoDebug;

fn chunk_gizmos(mut gizmo: Gizmos, mut chunk_gizmos: Gizmos<GizmoDebug>, chunk_map: Res<ChunkMap>) {
    let chunk_size = CHUNK_SIZE as f32;

    #[allow(clippy::never_loop)]
    for (chunk_coord, chunk) in chunk_map.0.iter() {
        let chunk_coord = chunk_coord.as_vec3() * chunk_size + (chunk_size / 2.0);
        match chunk.visible {
            true => {
                chunk_gizmos.cuboid(
                    Transform::from_translation(chunk_coord).with_scale(Vec3::splat(chunk_size)),
                    WHITE,
                );
            }
            false => {
                gizmo.cuboid(
                    Transform::from_translation(chunk_coord).with_scale(Vec3::splat(chunk_size)),
                    RED,
                );
            }
        }
    }

    chunk_gizmos.arrow(Vec3::ZERO, Vec3::Y * chunk_size, GREEN);
    chunk_gizmos.arrow(Vec3::ZERO, Vec3::X * chunk_size, RED);
    chunk_gizmos.arrow(Vec3::ZERO, Vec3::Z * chunk_size, BLUE);
}
