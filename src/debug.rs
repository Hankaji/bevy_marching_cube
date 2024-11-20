use bevy::{
    color::palettes::css::{BLUE, GREEN, RED, WHITE},
    prelude::*,
};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.init_gizmo_group::<GizmoDebug>()
            .add_systems(Update, (chunk_gizmos, marching_cube_gizmos));
    }
}

#[derive(Default, Reflect, GizmoConfigGroup)]
struct GizmoDebug;

fn chunk_gizmos(mut gizmos: Gizmos<GizmoDebug>) {
    gizmos.cuboid(
        Transform::from_xyz(8.0, 8.0, 8.0).with_scale(Vec3::splat(16.0)),
        WHITE,
    );

    gizmos.arrow(Vec3::ZERO, Vec3::Y * 2.0, GREEN);
    gizmos.arrow(Vec3::ZERO, Vec3::X * 2.0, RED);
    gizmos.arrow(Vec3::ZERO, Vec3::Z * 2.0, BLUE);
}

#[derive(Resource)]
pub struct MarchingCubeDebug {
    pub marching_pos: Vec3,
}

impl Default for MarchingCubeDebug {
    fn default() -> Self {
        Self {
            marching_pos: Vec3 {
                x: 0.5,
                y: 0.5,
                z: 0.5,
            },
        }
    }
}

fn marching_cube_gizmos(mut gizmos: Gizmos<GizmoDebug>, mc_dbg: Option<Res<MarchingCubeDebug>>) {
    if let Some(mc_dbg) = mc_dbg {
        let pos = mc_dbg.marching_pos;

        gizmos.cuboid(Transform::from_xyz(pos.x, pos.y, pos.z), WHITE);
    }
}
