use bevy::prelude::*;
use endless_terrain::EndlessTerrainPlugin;
use map_display::RenderChunk;
use noise_generator::{Noise, VoxelGrid};

pub mod endless_terrain;
mod map_display;
mod marching_table;
mod noise_generator;

pub struct MapGeneratorPlugin;

impl Plugin for MapGeneratorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EndlessTerrainPlugin)
            .add_systems(Startup, ready);
    }
}

fn ready(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
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
    //                 transform: Transform::from_translation(Vec3::new(x as f32, y as f32, z as f32)),
    //                 ..default()
    //             });
    //
    //             // commands.spawn(BillboardTextBundle {
    //             //     transform: Transform::from_xyz(x as f32, y as f32 + 0.1, z as f32)
    //             //         .with_scale(Vec3::splat(0.0015)),
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

    // commands.add(RenderChunk::new(IVec3::new(-1, 0, 0)));
    // commands.add(RenderChunk::new(IVec3::new(0, 0, 0)));
    // commands.add(RenderChunk::new(IVec3::new(1, 0, 0)));
    // commands.add(RenderChunk::new(IVec3::new(-1, 0, 1)));
    // commands.add(RenderChunk::new(IVec3::new(0, 0, 1)));
    // commands.add(RenderChunk::new(IVec3::new(1, 0, 1)));
    // commands.add(RenderChunk::new(IVec3::new(-1, 0, -1)));
    // commands.add(RenderChunk::new(IVec3::new(0, 0, -1)));
    // commands.add(RenderChunk::new(IVec3::new(1, 0, -1)));
}
