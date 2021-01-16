use crate::{
    hittables::{Hit, Hittable},
    ray::{Ray, MIN_T},
    vec3::{dot, Point3, Vec3},
};

#[derive(Copy, Clone, Debug)]
pub struct Sphere {
    pub initial_center: Point3,
    pub radius: f32,
    pub velocity: Option<Vec3>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f32, velocity: Option<Vec3>) -> Self {
        Self {
            initial_center: center,
            radius,
            velocity,
        }
    }

    pub fn center(&self, time: f32) -> Point3 {
        match self.velocity {
            Some(velocity) => self.initial_center + time * velocity,
            None => self.initial_center,
        }
    }

    fn hit_from_t(&self, ray: &Ray, t: f32, max_t: f32) -> Option<Hit> {
        if t < MIN_T || t > max_t {
            return None;
        }

        Some(Hit::new(ray.at(t), t))
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, max_t: f32) -> Option<Hit> {
        let translated_ray_origin = ray.origin - self.center(ray.time);
        let half_b = dot(ray.direction, translated_ray_origin);
        let c = dot(translated_ray_origin, translated_ray_origin) - f32::powi(self.radius, 2);

        let discriminant = f32::powi(half_b, 2) - c;

        if discriminant < 0.0 {
            return None;
        }

        let t0 = -half_b - f32::sqrt(discriminant);

        match self.hit_from_t(ray, t0, max_t) {
            None => {
                let t1 = -half_b + f32::sqrt(discriminant);

                self.hit_from_t(ray, t1, max_t)
            }
            hit => hit,
        }
    }
}
