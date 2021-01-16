use crate::{
    ray::Ray,
    vec3::{Point3, Vec3},
};

pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray, max_t: f32) -> Option<Hit>;
}

#[derive(Copy, Clone, Debug)]
pub struct Hit {
    pub point: Point3,
    pub t: f32,
    pub normal: Vec3,
}

impl Hit {
    pub fn new(point: Point3, t: f32, normal: Vec3) -> Self {
        Self { point, t, normal }
    }
}
