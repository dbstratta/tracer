use std::sync::Arc;

use crate::{
    color::Color,
    hittables::Hittable,
    ray::Ray,
    vec3::{Point3, Vec3},
};

#[derive(Clone)]
pub struct Scene {
    background: fn(&Ray) -> Color,
    pub objects: Arc<dyn Hittable>,
    pub lights: Arc<dyn Hittable>,
}

impl Scene {
    pub fn new(
        objects: Arc<dyn Hittable>,
        lights: Arc<dyn Hittable>,
        background: fn(&Ray) -> Color,
    ) -> Self {
        Self {
            objects,
            lights,
            background,
        }
    }

    pub fn background(&self, ray: &Ray) -> Color {
        (self.background)(ray)
    }
}
