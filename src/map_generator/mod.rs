use std::{thread::sleep, time::Duration};

use bevy::{
    color::palettes::css::{BLACK, ORANGE, WHITE},
    gizmos::gizmos,
    log::tracing_subscriber::fmt::format,
    prelude::*,
    reflect::List,
};
use bevy_mod_billboard::BillboardTextBundle;
use map_display::march_cube;
use noise_generator::{Noise, VoxelGrid};

use crate::debug::MarchingCubeDebug;

mod map_display;
mod marching_table;
mod noise_generator;

pub struct MapGeneratorPlugin;

impl Plugin for MapGeneratorPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<VoxelGrid>().add_systems(Startup, ready);
    }
}

fn ready(
    mut commands: Commands,
    mut mc_dbg: Option<ResMut<MarchingCubeDebug>>,
    mut voxel_grid: ResMut<VoxelGrid>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    *voxel_grid.as_mut() = Noise::generate_noise_map(16, 1.0, 0, 1, 0.5, 2.0);

    let size = voxel_grid.size;

    // for z in 0..size {
    //     for y in 0..size {
    //         for x in 0..size {
    //             if voxel_grid.read(x, y, z) > 0.0 {
    //                 continue;
    //             }
    //
    //             // Cubes
    //             commands.spawn(PbrBundle {
    //                 mesh: meshes.add(Cuboid::from_length(0.1)),
    //                 material: materials.add(Color::srgb_u8(0, 0, 0)),
    //                 transform: Transform::from_xyz(x as f32, y as f32, z as f32),
    //                 ..default()
    //             });
    //
    //             // commands.spawn(BillboardTextBundle {
    //             //     transform: Transform::from_xyz(x as f32, y as f32 + 0.1, z as f32)
    //             //         .with_scale(Vec3::splat(0.0015)),
    //             //     text: Text::from_sections([TextSection {
    //             //         // value: color.to_string(),
    //             //         value: format!("[{x} {y} {z}]"),
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

    commands.init_resource::<MarchingCubeDebug>();
    let mut positions: Vec<Vec3> = Vec::new();

    for z in 0..(size - 1) {
        for y in 0..(size - 1) {
            for x in 0..(size - 1) {
                march_cube((x, y, z), &voxel_grid, &mut positions);
            }
        }
    }

    for i in 0..(positions.len() / 3) {
        // for i in 0..1 {
        let i = i * 3;
        if positions.get(i).is_none() {
            warn!("Cant find entry at index i = {i}");
            break;
        }
        commands.spawn(PbrBundle {
            mesh: meshes.add(Triangle3d::new(
                positions[i + 2],
                positions[i + 1],
                positions[i],
            )),
            material: materials.add(Color::srgb_u8(255, 0, 0)),
            // transform: Transform::from_xyz(x as f32, y as f32, z as f32),
            ..default()
        });
    }
}

fn march_cubes(
    mut commands: Commands,
    voxel_grid: Res<VoxelGrid>,
    mut mc_dbg: Option<ResMut<MarchingCubeDebug>>,
) {
    let size = voxel_grid.size;

    commands.init_resource::<MarchingCubeDebug>();
    let mut positions: Vec<Vec3> = Vec::new();

    for z in 0..(size - 1) {
        for y in 0..(size - 1) {
            for x in 0..(size - 1) {
                march_cube((x, y, z), &voxel_grid, &mut positions);

                println!("[{x} {y} {z}]");
                // mc_dbg.as_mut().marching_pos = Vec3::new(x as f32, y as f32, z as f32);
                // gizmos.cuboid(Transform::from_xyz(x as f32, y as f32, z as f32), WHITE);
            }
        }
    }
}
