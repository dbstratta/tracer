use std::sync::Arc;

use crate::{hittable::Hit, ray::Ray};

#[derive(Clone)]
pub enum Material {
    Reflective(Arc<dyn ReflectiveMaterial>),
    Emissive(Arc<dyn EmissiveMaterial>),
}

pub trait ReflectiveMaterial: Send + Sync {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<Ray>;
}

pub trait EmissiveMaterial: Send + Sync {}
