use bevy::math::Vec3;

use super::NoiseGenerator;

pub struct SphereNoiseDensity {
    radius: f32,
    center: Vec3,
}

impl SphereNoiseDensity {
    pub fn new(radius: f32, center: Vec3) -> Self {
        Self { radius, center }
    }
}

impl NoiseGenerator for SphereNoiseDensity {
    fn get_scalar(&self, x: f32, y: f32, z: f32) -> f32 {
        let offset = Vec3::new(x, y, z);

        // Function for defining a sphrere r^2 = x^2 + y^2 + z^2
        (offset - self.center).powf(2.0).element_sum() - self.radius.powi(2)
    }
}
