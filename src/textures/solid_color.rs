use crate::{rgb_color::RgbColor, textures::Texture, vec3::Point3};

pub struct SolidColor {
    color: RgbColor,
}

impl SolidColor {
    pub const fn new(red: f32, green: f32, blue: f32) -> Self {
        Self::from_rgb_color(RgbColor::new(red, green, blue))
    }

    pub const fn from_rgb_color(color: RgbColor) -> Self {
        Self { color }
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: f32, _v: f32, _point: Point3) -> RgbColor {
        self.color
    }
}
