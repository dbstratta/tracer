use std::sync::Arc;

use crate::{hittable::Hit, pdfs::ScatteringPdf, ray::Ray};

#[derive(Clone)]
pub enum Material {
    Reflective(Arc<dyn ReflectiveMaterial>),
    Emissive(Arc<dyn EmissiveMaterial>),
}

pub trait ReflectiveMaterial: Send + Sync {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<ScatterResult>;
}

#[derive(Clone)]
pub enum ScatterResult {
    Diffuse {
        pdf: Arc<dyn ScatteringPdf>,
        scatter_probability: f32,
    },
    Specular {
        ray: Ray,
    },
}

pub trait EmissiveMaterial: Send + Sync {
    fn intensity(&self, wavelength: f32) -> f32;
}
