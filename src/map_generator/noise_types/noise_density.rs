use fastnoise_lite::FastNoiseLite;

use super::NoiseGenerator;

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

        noise_height = y - (noise_height * Self::HEIGHT_WEIGHT);
        noise_height
    }
}

impl NoiseDensity {
    const HEIGHT_WEIGHT: f32 = 200f32;

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
