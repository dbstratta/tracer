use crate::{
    hittables::{Hit, Hittable},
    ray::{Ray, MIN_T},
    vec3::{dot, Point3, Vec3},
};

#[derive(Copy, Clone, Debug)]
pub struct Triangle {
    pub velocity: Option<Vec3>,
}

impl Triangle {
    pub fn new(velocity: Option<Vec3>) -> Self {
        Self { velocity }
    }
}

impl Hittable for Triangle {
    fn hit(&self, ray: &Ray, max_t: f32) -> Option<Hit> {
        None
    }
}
