use crate::{rgb_color::RgbColor, vec3::Point3};

pub trait Texture: Send + Sync {
    fn value(&self, u: f32, v: f32, point: Point3) -> RgbColor;
}
