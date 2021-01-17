use std::sync::Arc;

use crate::{
    helpers::random,
    hittable::Hit,
    materials::ReflectiveMaterial,
    ray::Ray,
    textures::Texture,
    vec3::{dot, Vec3},
};

pub struct Lambertian {
    pub texture: Arc<dyn Texture>,
    pub reflectance: f32,
}

impl Lambertian {
    pub fn new(texture: Arc<dyn Texture>, reflectance: f32) -> Self {
        Self {
            texture,
            reflectance,
        }
    }
}

impl ReflectiveMaterial for Lambertian {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<Ray> {
        let scattered_ray_probability = ray.probability
            * self.reflectance
            * self.texture.probability(ray.wavelength, hit.u, hit.v);

        if scattered_ray_probability == 0.0 {
            return None;
        }

        let scattered_direction = Vec3::from_polar(1.0, f32::acos(dot(hit.normal, Vec3::x())));

        Some(Ray::new(
            hit.point,
            scattered_direction,
            ray.wavelength,
            scattered_ray_probability,
            ray.time,
        ))
    }
}
