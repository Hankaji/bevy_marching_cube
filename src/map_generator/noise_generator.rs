use bevy::prelude::Resource;
use fastnoise_lite::FastNoiseLite;

pub struct Noise;

impl Noise {
    /// Generate a 3-Dimensional noise map which is converted into a 1-Dimensional array
    pub fn generate_noise_map(
        size: usize,
        mut scale: f32,
        seed: i32,
        octaves: u32,
        persistance: f32,
        lacunarity: f32,
    ) -> VoxelGrid {
        // Init empty (null) list of noise value
        let mut noise_map: VoxelGrid = VoxelGrid::new(size);

        // NOTE: FNL from fastnoise_lite rust crate instead of godot
        let mut noise = FastNoiseLite::new();
        noise.set_noise_type(Some(fastnoise_lite::NoiseType::Perlin));
        noise.set_seed(Some(seed));
        noise.set_frequency(Some(0.005));

        // cap to closed 0 to avoid devision by 0
        if scale <= 0.0 {
            scale = 0.0001
        };

        // let half_width = width as f32 / 2_f32;
        // let half_height = height as f32 / 2_f32;

        for x in 0..size {
            for y in 0..size {
                for z in 0..size {
                    let mut amplitude = 1f32;
                    let mut frequency = 1f32;
                    let mut noise_height = 0f32;

                    // for _ in 0..octaves {
                    //     // (x - half_width) let noise map zoom on the center when scale changes
                    //     let sample_x = (x as f32 - half_width) / scale * frequency;
                    //     let sample_y = (y as f32 - half_height) / scale * frequency;
                    //
                    //     // Get noise value and remapping it to 0..1 range
                    //     let mut noise_val = noise.get_noise_2d(sample_x, sample_y);
                    //     noise_val = (noise_val + 1.) / 2.;
                    //
                    //     // godot::global::godot_print!("value at ({x}, {y}): {noise_val}");
                    //
                    //     noise_height = noise_val * amplitude;
                    //
                    //     amplitude *= persistance;
                    //     frequency *= lacunarity;
                    // }
                    //
                    // const MAX_HEIGHT: f32 = 50_f32;
                    // noise_height = z as f32 - (noise_height * MAX_HEIGHT);
                    // match (x, y, z) {
                    //     (0..=1, 0..=1, 0..=1) => noise_map.push(-1.),
                    //     _ => noise_map.push(Self::scalar_field(x as f32, y as f32, z as f32)),
                    // }

                    noise_map.push(Self::scalar_field(x as f32, y as f32, z as f32));
                }
            }
        }

        noise_map
    }

    fn scalar_field(x: f32, y: f32, z: f32) -> f32 {
        const RADIUS: f32 = 7.0;

        const CENTER_X: f32 = 8.0;
        const CENTER_Y: f32 = 8.0;
        const CENTER_Z: f32 = 8.0;

        // NOTE: Only for testing
        // Function for defining a sphrere r^2 = x^2 + y^2 + z^2
        ((x - CENTER_X).powi(2) + (y - CENTER_Y).powi(2) + (z - CENTER_Z).powi(2)).sqrt() - RADIUS
    }
}

/// A Data type for containing a 3-Dimensional space grid value
///
/// data: A 1D vector hold a list of value in a 3D space
/// Accessing this require both asix x, y and z in this formular
/// `x + size * (y + size * z)`
#[derive(Default, Debug, Clone, Resource)]
pub struct VoxelGrid {
    data: Vec<f32>,
    pub size: usize,
    min: f32,
    max: f32,
}

impl VoxelGrid {
    pub fn new(size: usize) -> Self {
        Self {
            data: Vec::with_capacity(size.pow(3)),
            size,
            min: f32::MAX,
            max: f32::MIN,
        }
    }

    pub fn push(&mut self, value: f32) {
        // Set noise bound
        if value > self.max {
            self.max = value;
        } else if value < self.min {
            self.min = value;
        }

        self.data.push(value);
    }

    pub fn normalize(&mut self) {
        // Inverse lerp the noise value to get a more consistent value
        // for val in self.data.iter_mut() {
        //     *val = inverse_lerp(self.min.into(), self.max.into(), (*val).into()) as f32;
        // }
    }

    pub fn read(&self, x: usize, y: usize, z: usize) -> f32 {
        self.data[Self::to_1d(x, y, z, self.size)]
    }

    pub fn try_read(&self, x: usize, y: usize, z: usize) -> Option<&f32> {
        self.data.get(Self::to_1d(x, y, z, self.size))
    }

    pub fn to_1d(x: usize, y: usize, z: usize, size: usize) -> usize {
        x + y * size + z * size.pow(2)
    }
}
