use bevy::math::Vec3;

pub mod noise_density;
pub mod sphere_noise;
pub mod underwater_cave_noise;

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
