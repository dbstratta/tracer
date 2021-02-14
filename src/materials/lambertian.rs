use std::sync::Arc;

use crate::{
    hittable::Hit,
    materials::{ReflectiveMaterial, ScatterResult},
    pdfs,
    ray::Ray,
    textures::Texture,
};

pub struct Lambertian {
    pub texture: Arc<dyn Texture>,
    pub reflectance: f32,
}

impl Lambertian {
    pub fn new(texture: Arc<dyn Texture>, reflectance: f32) -> Self {
        Self {
            texture,
            reflectance,
        }
    }

    pub fn scatter_probability(&self, wavelength: f32, hit: &Hit) -> f32 {
        self.reflectance * self.texture.probability(wavelength, hit.u, hit.v)
    }
}

impl ReflectiveMaterial for Lambertian {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<ScatterResult> {
        let scatter_probability = self.scatter_probability(ray.wavelength, hit);

        if scatter_probability == 0.0 {
            return None;
        }

        let pdf = Arc::new(pdfs::Cosine::new(hit.normal));

        Some(ScatterResult::Diffuse {
            pdf,
            scatter_probability,
        })
    }
}
