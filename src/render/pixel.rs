use crate::color::{Color, BLACK};

pub struct Pixel {
    samples: Vec<Color>,
}

impl Pixel {
    pub const fn new(samples: Vec<Color>) -> Self {
        Self { samples }
    }

    pub fn process(&self, gamma: f32) -> Color {
        self.samples.iter().fold(BLACK, |sum, &sample| {
            sum + (sample / self.samples.len() as f32).gamma(gamma)
        })
    }
}
