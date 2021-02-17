use std::sync::Arc;

use crate::{
    hittable::Hit,
    materials::EmissiveMaterial,
    ray::Ray,
    rgb_color::{RgbColor, BLACK},
    textures::{SolidColor, Texture},
};

#[derive(Clone)]
pub struct DiffuseLight {
    pub emit: Arc<dyn Texture>,
}

impl DiffuseLight {
    pub fn new(emit: Arc<dyn Texture>) -> Self {
        Self { emit }
    }

    pub fn from_rgb_color(emit: RgbColor) -> Self {
        Self::new(Arc::new(SolidColor::from_rgb_color(emit)))
    }

    pub fn from_rgb(red: f32, green: f32, blue: f32) -> Self {
        Self::from_rgb_color(RgbColor::new(red, green, blue))
    }
}

impl EmissiveMaterial for DiffuseLight {
    fn emitted(&self, _ray: &Ray, hit: &Hit) -> RgbColor {
        if hit.front_face {
            self.emit.value(hit.u, hit.v, hit.point)
        } else {
            BLACK
        }
    }
}
