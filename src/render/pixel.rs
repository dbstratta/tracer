use crate::rgb_color::{RgbColor, BLACK};

pub struct Pixel {
    samples: Vec<RgbColor>,
    samples_per_pixel: u32,
}

impl Pixel {
    pub const fn new(samples: Vec<RgbColor>, samples_per_pixel: u32) -> Self {
        Self {
            samples,
            samples_per_pixel,
        }
    }

    pub fn rgb_color(&self) -> RgbColor {
        self.samples.iter().fold(BLACK, |sum, &sample| {
            sum + (sample / self.samples_per_pixel as f32)
        })
    }
}
