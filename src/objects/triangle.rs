use crate::{
    hittable::{Hit, Hittable},
    mobile::Mobile,
    objects::Object,
    ray::{Ray, MIN_T},
    vec3::{dot, Point3, Vec3},
};

#[derive(Copy, Clone, Debug)]
pub struct Triangle {
    pub initial_velocity: Vec3,
    pub acceleration: Vec3,
}

impl Triangle {
    pub fn new() -> Self {
        Self {
            initial_velocity: Vec3::zero(),
            acceleration: Vec3::zero(),
        }
    }
}

impl Object for Triangle {}

impl Hittable for Triangle {
    fn hit(&self, ray: &Ray, max_t: f32) -> Option<Hit> {
        None
    }
}

impl Mobile for Triangle {
    fn accelerate(&mut self, acceleration: Vec3, initial_velocity: Vec3) {}
}
