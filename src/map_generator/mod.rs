use std::fmt::Debug;

use bevy::prelude::*;
use endless_terrain::{EndlessTerrainPlugin, CHUNK_SIZE};
use fastnoise_lite::FastNoiseLite;
use map_display::RenderChunk;
use noise_generator::{Noise, VoxelGrid};
use sphere_noise::SphereNoiseDensity;

use crate::utils::To1DIndex;

pub mod endless_terrain;
mod map_display;
mod marching_table;
mod noise_generator;
mod sphere_noise;

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
    let map_gen = MapGenerator::new(NoiseDensity::default());
    // let map_gen = MapGenerator::new(SphereNoiseDensity::new(18.0, (12.0, 12.0, 12.0).into()));
    commands.insert_resource(map_gen);

    commands.add(RenderChunk::new(IVec3::new(0, 0, 0)));
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
    generation_type: Box<dyn NoiseGenerator>,
}

impl MapGenerator {
    pub fn new(gen_type: impl NoiseGenerator + 'static) -> Self {
        Self {
            generation_type: Box::new(gen_type),
        }
    }

    pub fn generate_noise(&self, chunk_coord: IVec3, size: usize) -> VoxelGrid {
        // Grid size (VoxelGrid size) is increased because as opposed to the chunk size which is correctly 16^3 in
        // size. Block data however start from 0 to 16, included in all of the corners of the
        // grid/chunk.
        let grid_size = size + 1;
        let mut noise_map: VoxelGrid = VoxelGrid::new(grid_size, chunk_coord);

        for z in 0..grid_size {
            for y in 0..grid_size {
                for x in 0..grid_size {
                    let offset = (chunk_coord * size as i32).as_vec3();
                    let x = x as f32 + offset.x;
                    let y = y as f32 + offset.y;
                    let z = z as f32 + offset.z;

                    noise_map.push(self.generation_type.get_scalar(x, y, z));
                }
            }
        }

        noise_map
    }
}

pub trait NoiseGenerator: Send + Sync {
    /// Get a scalar value at [x y z] position using
    /// defined scalar function here
    fn get_scalar(&self, x: f32, y: f32, z: f32) -> f32;

    /// Get Scalar value using Vec3
    #[allow(dead_code)]
    fn get_scalar_v(&self, pos: Vec3) -> f32 {
        self.get_scalar(pos.x, pos.y, pos.z)
    }
}

pub struct NoiseDensity {
    /// A noise to be used in the generation of terrain
    noise: FastNoiseLite,
    scale: f32,
    octaves: u32,
    persistance: f32,
    lacunarity: f32,
}

impl Default for NoiseDensity {
    fn default() -> Self {
        let mut noise = FastNoiseLite::new();
        noise.set_noise_type(Some(fastnoise_lite::NoiseType::Perlin));
        noise.set_seed(Some(6969));
        noise.set_frequency(Some(0.005));

        Self {
            noise,
            scale: 1.0,
            octaves: 3,
            persistance: 0.5,
            lacunarity: 2.0,
        }
    }
}

impl NoiseGenerator for NoiseDensity {
    fn get_scalar(&self, x: f32, y: f32, z: f32) -> f32 {
        let mut amplitude = 1f32;
        let mut frequency = 1f32;
        let mut noise_height = 0f32;

        for _ in 0..self.octaves {
            let sample_x = x / self.scale * frequency;
            let sample_z = z / self.scale * frequency;

            // Get noise value and remapping it to 0..1 range
            let mut noise_val = self.noise.get_noise_2d(sample_x, sample_z);
            noise_val = (noise_val + 1.) / 2.;

            noise_height = noise_val * amplitude;

            amplitude *= self.persistance;
            frequency *= self.lacunarity;
        }

        noise_height = y - (noise_height * 200.0);
        noise_height
    }
}

impl NoiseDensity {
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
}
