use std::sync::Arc;

use crate::{hittable::Hit, pdfs::ScatteringPdf, ray::Ray, rgb_color::RgbColor};

#[derive(Clone)]
pub enum Material {
    Reflective(Arc<dyn ReflectiveMaterial>),
    Emissive(Arc<dyn EmissiveMaterial>),
}

pub trait ReflectiveMaterial: Send + Sync {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> ScatterResult;
}

#[derive(Clone)]
pub enum ScatterResult {
    Diffuse {
        pdf: Arc<dyn ScatteringPdf>,
        attenuation: RgbColor,
    },
    Specular {
        ray: Ray,
        attenuation: RgbColor,
    },
}

pub trait EmissiveMaterial: Send + Sync {
    fn emitted(&self, ray: &Ray, hit: &Hit) -> RgbColor;
}
