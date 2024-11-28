use super::NoiseGenerator;

pub struct SphereNoiseDensity {
    radius: f32,
}

impl SphereNoiseDensity {
    pub fn new(radius: f32) -> Self {
        Self { radius }
    }
}

impl NoiseGenerator for SphereNoiseDensity {
    fn get_scalar(&self, x: f32, y: f32, z: f32) -> f32 {
        const CENTER_X: f32 = 8.0;
        const CENTER_Y: f32 = 8.0;
        const CENTER_Z: f32 = 8.0;

        // Function for defining a sphrere r^2 = x^2 + y^2 + z^2
        ((x - CENTER_X).powi(2) + (y - CENTER_Y).powi(2) + (z - CENTER_Z).powi(2)).sqrt()
            - self.radius
    }
}
