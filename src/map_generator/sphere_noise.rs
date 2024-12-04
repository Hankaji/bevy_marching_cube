use bevy::math::{Vec2, Vec3, Vec3Swizzles};

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
    // fn get_scalar(&self, x: f32, y: f32, z: f32) -> f32 {
    //     let offset = Vec3::new(x, y, z);
    //
    //     // Function for defining a sphrere r^2 = x^2 + y^2 + z^2
    //     (offset - self.center).powf(2.0).element_sum() - self.radius.powi(2)
    // }

    fn get_scalar(&self, x: f32, y: f32, z: f32) -> f32 {
        let ra = 20.0;
        let rb = 18.0;
        let d = 10.0;

        let p2 = Vec3::new(x, y, z);

        let p: Vec2 = (p2.x, p2.yz().length()).into();
        let a = (ra * ra - rb * rb + d * d) / (2.0 * d);
        let b = f32::max(ra * ra - a * a, 0.0).sqrt();
        if p.x * b - p.y * a > d * f32::max(b - p.y, 0.0) {
            (p - Vec2::new(a, b)).length()
        } else {
            f32::max(p.length() - ra, -((p - Vec2::new(d, 0.0)).length() - rb))
        }
    }
}
