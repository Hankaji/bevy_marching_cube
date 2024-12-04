use bevy::prelude::*;
use endless_terrain::EndlessTerrainPlugin;
use fastnoise_lite::FastNoiseLite;
use noise_generator::VoxelGrid;
use noise_types::{
    noise_density::NoiseDensity, underwater_cave_noise::UnderwaterCaveNoiseDensity, NoiseGenerator,
};
use sphere_noise::SphereNoiseDensity;

pub mod endless_terrain;
mod map_display;
mod marching_table;
mod noise_generator;
mod noise_types;
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
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut noise = FastNoiseLite::new();
    noise.set_noise_type(Some(fastnoise_lite::NoiseType::OpenSimplex2));
    noise.set_seed(Some(972483));
    noise.set_frequency(Some(0.005));

    let map_gen = MapGenerator::new(NoiseDensity::new(noise, 3.0, 3, 0.5, 2.0));
    // let map_gen = MapGenerator::new(SphereNoiseDensity::new(8.0, (8.0, 8.0, 8.0).into()));
    // let map_gen = MapGenerator::new(UnderwaterCaveNoiseDensity::new(noise, 0.2));
    commands.insert_resource(map_gen);
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
