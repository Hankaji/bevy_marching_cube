use std::fmt::Debug;

use bevy::prelude::*;
use endless_terrain::{EndlessTerrainPlugin, CHUNK_SIZE};
use fastnoise_lite::FastNoiseLite;
use map_display::RenderChunk;
use noise_generator::{Noise, VoxelGrid};

use crate::utils::To1DIndex;

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

#[derive(Resource)]
pub struct MapGenerator {
    /// A noise to be used in the generation of terrain
    noise: FastNoiseLite,
    scale: f32,
    octaves: u32,
    persistance: f32,
    lacunarity: f32,
}

impl Debug for MapGenerator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MapGenerator")
            .field("noise", &"FastNoiseLite")
            .finish()
    }
}

impl Default for MapGenerator {
    fn default() -> Self {
        let mut noise = FastNoiseLite::new();
        noise.set_noise_type(Some(fastnoise_lite::NoiseType::Perlin));
        noise.set_seed(Some(6969));
        noise.set_frequency(Some(0.005));

        // cap to closed 0 to avoid devision by 0

        Self {
            noise,
            scale: 1.0,
            octaves: 3,
            persistance: 0.5,
            lacunarity: 2.0,
        }
    }
}

impl MapGenerator {
    pub fn new(
        noise: FastNoiseLite,
        scale: f32,
        octaves: u32,
        persistance: f32,
        lacunarity: f32,
    ) -> Self {
        Self {
            noise,
            scale,
            octaves,
            persistance,
            lacunarity,
        }
    }

    pub fn read(&self, x: usize, y: usize, z: usize) -> f32 {
        [0.0; 256][(x, y, z, 16).to_1d_idx()]
    }

    /// Get noise value in a calculated noise_map in 3D space
    fn get_noise(&self, x: f32, y: f32, z: f32) -> f32 {
        // let half_width = width as f32 / 2_f32;
        // let half_height = height as f32 / 2_f32;

        let mut amplitude = 1f32;
        let mut frequency = 1f32;
        let mut noise_height = 0f32;

        for _ in 0..self.octaves {
            let offset_x = x * 16.0;
            let offset_z = z * 16.0;

            let sample_x = offset_x / self.scale * frequency;
            let sample_z = offset_z / self.scale * frequency;

            // Get noise value and remapping it to 0..1 range
            let mut noise_val = self.noise.get_noise_2d(sample_x, sample_z);
            noise_val = (noise_val + 1.) / 2.;

            noise_height = noise_val * amplitude;

            amplitude *= self.persistance;
            frequency *= self.lacunarity;
        }

        noise_height
        // const MAX_HEIGHT: f32 = 20_f32;
        // y - (noise_height * MAX_HEIGHT)
    }
}
