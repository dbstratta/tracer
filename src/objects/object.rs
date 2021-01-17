use crate::{
    hittable::Hittable,
    ray::Ray,
    vec3::{Point3, Vec3},
};

pub trait Object: Hittable + Send + Sync {}
