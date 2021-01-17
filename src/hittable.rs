use std::sync::Arc;

use crate::{
    materials::Material,
    ray::Ray,
    vec3::{dot, Point3, Vec3},
};

pub trait Hittable {
    fn hit(&self, ray: &Ray, max_t: f32) -> Option<Hit>;
}

#[derive(Clone)]
pub struct Hit {
    pub point: Point3,
    pub t: f32,
    pub normal: Vec3,
    pub material: Arc<Material>,
    pub u: f32,
    pub v: f32,
}

impl Hit {
    pub fn new(
        point: Point3,
        t: f32,
        outward_normal: Vec3,
        ray: &Ray,
        material: Arc<Material>,
        u: f32,
        v: f32,
    ) -> Self {
        let normal = Self::normal_from_outward_normal(outward_normal, ray);

        Self {
            point,
            t,
            normal,
            material,
            u,
            v,
        }
    }

    pub fn normal_from_outward_normal(outward_normal: Vec3, ray: &Ray) -> Vec3 {
        if dot(outward_normal, ray.direction) > 0.0 {
            outward_normal
        } else {
            -outward_normal
        }
    }
}
