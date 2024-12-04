use bevy::math::Vec3;
use fastnoise_lite::FastNoiseLite;

use super::NoiseGenerator;

pub struct UnderwaterCaveNoiseDensity {
    noise: FastNoiseLite,
    scale: f32,
}

impl UnderwaterCaveNoiseDensity {
    pub fn new(noise: FastNoiseLite, scale: impl Into<f32>) -> Self {
        Self {
            noise,
            scale: scale.into(),
        }
    }
}

impl NoiseGenerator for UnderwaterCaveNoiseDensity {
    fn get_scalar(&self, x: f32, y: f32, z: f32) -> f32 {
        let sample_x = x / self.scale;
        let sample_y = y / self.scale;
        let sample_z = z / self.scale;

        // Get noise value and remapping it to 0..1 range
        let noise_val = self.noise.get_noise_3d(sample_x, sample_y, sample_z);
        // (noise_val + 1.) / 2.
        noise_val
    }
}
