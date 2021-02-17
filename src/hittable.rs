use std::sync::Arc;

use crate::{
    materials::Material,
    ray::{Ray, MIN_T},
    vec3::{dot, Point3, Vec3},
};

pub trait Hittable {
    fn hit(&self, ray: &Ray, max_t: f32) -> Option<Hit>;

    fn ray_to_self_probability(&self, ray: &Ray) -> f32;
    fn random_direction_to_self(&self, origin: Point3, time: f32) -> Vec3;
}

#[derive(Clone)]
pub struct Hit {
    pub point: Point3,
    pub t: f32,
    pub normal: Vec3,
    pub material: Arc<Material>,
    pub u: f32,
    pub v: f32,
    pub front_face: bool,
}

impl Hit {
    pub fn new(
        ray: &Ray,
        t: f32,
        outward_normal: Vec3,
        material: Arc<Material>,
        u: f32,
        v: f32,
    ) -> Self {
        let front_face = dot(outward_normal, ray.direction) < 0.0;
        let normal = Self::normal_from_outward_normal(outward_normal, front_face);

        Self {
            point: ray.at(t),
            t,
            normal,
            material,
            u,
            v,
            front_face,
        }
    }

    pub fn normal_from_outward_normal(outward_normal: Vec3, front_face: bool) -> Vec3 {
        if front_face {
            outward_normal
        } else {
            -outward_normal
        }
    }
}

pub fn intersects(t: f32, max_t: f32) -> bool {
    if t < MIN_T || t > max_t || !t.is_normal() {
        false
    } else {
        true
    }
}
