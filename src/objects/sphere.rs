use std::f32::consts::PI;
use std::sync::Arc;

use crate::{
    helpers::random,
    hittable::{Hit, Hittable},
    materials::Material,
    mobile::{position, Mobile},
    objects::Object,
    onb::Onb,
    ray::{Ray, MAX_T, MIN_T},
    vec3::{dot, Point3, Vec3},
};

#[derive(Clone)]
pub struct Sphere {
    pub initial_center: Point3,
    pub radius: f32,
    pub initial_velocity: Vec3,
    pub acceleration: Vec3,
    pub material: Arc<Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f32, material: Arc<Material>) -> Self {
        Self {
            initial_center: center,
            radius,
            acceleration: Vec3::zero(),
            initial_velocity: Vec3::zero(),
            material,
        }
    }

    pub fn center(&self, time: f32) -> Point3 {
        position(
            time,
            self.acceleration,
            self.initial_velocity,
            self.initial_center,
        )
    }

    fn hit_from_t(&self, ray: &Ray, t: f32, max_t: f32, center: Point3) -> Option<Hit> {
        if t < MIN_T || t > max_t {
            return None;
        }

        let outward_normal = ray.direction - center;

        let hit_point = ray.at(t);
        let (u, v) = Self::texture_uv(hit_point, center);

        Some(Hit::new(
            hit_point,
            t,
            outward_normal,
            ray,
            Arc::clone(&self.material),
            u,
            v,
        ))
    }

    fn texture_uv(hit_point: Point3, center: Point3) -> (f32, f32) {
        let d = (hit_point - center).unit();

        (0.5 + f32::atan2(d.x, d.z), 0.5 - f32::asin(d.y))
    }
}

impl Object for Sphere {}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, max_t: f32) -> Option<Hit> {
        let center = self.center(ray.time);
        let translated_ray_origin = ray.origin - center;
        let half_b = dot(ray.direction, translated_ray_origin);
        let c = dot(translated_ray_origin, translated_ray_origin) - f32::powi(self.radius, 2);

        let discriminant = f32::powi(half_b, 2) - c;

        if discriminant < 0.0 {
            return None;
        }

        let t0 = -half_b - f32::sqrt(discriminant);

        match self.hit_from_t(ray, t0, max_t, center) {
            None => {
                let t1 = -half_b + f32::sqrt(discriminant);

                self.hit_from_t(ray, t1, max_t, center)
            }
            hit => hit,
        }
    }

    fn ray_to_self_probability(&self, ray: &Ray) -> f32 {
        match self.hit(ray, MAX_T) {
            Some(_) => {
                let cos_theta_max = (1.0
                    - self.radius.powi(2) / (self.center(ray.time) - ray.origin).len_squared())
                .sqrt();

                let solid_angle = 2.0 * PI * (1.0 - cos_theta_max);

                1.0 / solid_angle
            }
            None => 0.0,
        }
    }

    fn random_direction_to_self(&self, origin: Point3, time: f32) -> Vec3 {
        let direction = self.center(time) - origin;

        let r1 = random(0.0..1.0);
        let r2 = random(0.0..1.0);

        let z = 1.0 + r2 * (f32::sqrt(1.0 - self.radius.powi(2) / direction.len_squared()) - 1.0);

        let phi = 2.0 * PI * r1;
        let x = phi.cos() * (1.0 - z.powi(2)).sqrt();
        let y = phi.sin() * (1.0 - z.powi(2)).sqrt();

        let direction = Vec3::new(x, y, z);

        Onb::from_w(direction).local(direction)
    }
}

impl Mobile for Sphere {
    fn accelerate(&mut self, acceleration: Vec3, initial_velocity: Vec3) {
        self.acceleration = acceleration;
        self.initial_velocity = initial_velocity;
    }
}
