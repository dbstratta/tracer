use std::f32::consts::PI;

use crate::vec3::Vec3;

pub trait Pdf<T> {
    fn value(&self, x: T) -> f32;
}

pub struct Normal {
    mean: f32,
    deviation: f32,
    variance: f32,
}

impl Normal {
    pub fn new(mean: f32, deviation: f32) -> Self {
        Self {
            mean,
            deviation,
            variance: deviation.powi(2),
        }
    }
}

impl Pdf<f32> for Normal {
    fn value(&self, x: f32) -> f32 {
        f32::exp(-((x - self.mean).powi(2) / self.variance) / 2.0)
            / (self.deviation * f32::sqrt(2.0 * PI))
    }
}

pub trait ScatteringPdf {
    fn value(&self, direction: Vec3) -> f32;
    fn sample() -> Vec3;
}
