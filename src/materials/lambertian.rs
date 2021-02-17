use std::sync::Arc;

use crate::{
    hittable::Hit,
    materials::{ReflectiveMaterial, ScatterResult},
    pdfs::CosinePdf,
    ray::Ray,
    rgb_color::RgbColor,
    textures::{SolidColor, Texture},
};

pub struct Lambertian {
    pub texture: Arc<dyn Texture>,
}

impl Lambertian {
    pub fn new(texture: Arc<dyn Texture>) -> Self {
        Self { texture }
    }

    pub fn from_rgb_color(color: RgbColor) -> Self {
        Self::new(Arc::new(SolidColor::from_rgb_color(color)))
    }

    pub fn from_rgb(red: f32, green: f32, blue: f32) -> Self {
        Self::from_rgb_color(RgbColor::new(red, green, blue))
    }
}

impl ReflectiveMaterial for Lambertian {
    fn scatter(&self, _ray: &Ray, hit: &Hit) -> ScatterResult {
        ScatterResult::Diffuse {
            pdf: Arc::new(CosinePdf::new(hit.normal)),
            attenuation: self.texture.value(hit.u, hit.v, hit.point),
        }
    }
}
