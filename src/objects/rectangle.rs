use std::sync::Arc;

use crate::{
    helpers::random,
    hittable::{intersects, Hit, Hittable},
    materials::Material,
    mobile::{position, Mobile},
    objects::Object,
    onb::Onb,
    ray::{Ray, MAX_T},
    vec3::{cross, dot, Point3, Vec3},
};

#[derive(Clone)]
pub struct Rectangle {
    pub initial_top_left_corner: Point3,
    pub normal: Vec3,
    pub height: f32,
    pub width: f32,
    pub vertical_edge: Vec3,
    pub horizontal_edge: Vec3,
    pub initial_velocity: Vec3,
    pub acceleration: Vec3,
    pub material: Arc<Material>,
}

impl Rectangle {
    pub fn new(
        top_left_corner: Point3,
        normal: Vec3,
        up: Vec3,
        height: f32,
        width: f32,
        material: Arc<Material>,
    ) -> Self {
        let right = -cross(normal, up);

        Self {
            initial_top_left_corner: top_left_corner,
            normal,
            height,
            vertical_edge: height * -up,
            width,
            horizontal_edge: width * right,
            acceleration: Vec3::zero(),
            initial_velocity: Vec3::zero(),
            material,
        }
    }

    pub fn top_left_corner(&self, time: f32) -> Point3 {
        position(
            time,
            self.acceleration,
            self.initial_velocity,
            self.initial_top_left_corner,
        )
    }

    fn random_point_in_self(&self, time: f32) -> Point3 {
        self.top_left_corner(time)
            + random(0.0..1.0) * self.vertical_edge
            + random(0.0..1.0) * self.horizontal_edge
    }

    fn area(&self) -> f32 {
        self.width * self.height
    }
}

impl Object for Rectangle {}

impl Hittable for Rectangle {
    fn hit(&self, ray: &Ray, max_t: f32) -> Option<Hit> {
        let top_left_corner = self.top_left_corner(ray.time);
        let t = dot(top_left_corner - ray.origin, self.normal) / dot(ray.direction, self.normal);

        if !intersects(t, max_t) {
            return None;
        }

        let hit_point = ray.at(t);

        let v = hit_point - top_left_corner;

        let projection1 = dot(v, self.horizontal_edge) / self.width;
        let projection2 = dot(v, self.vertical_edge) / self.height;

        if projection1 > self.width
            || projection1 < 0.0
            || projection2 > self.height
            || projection2 < 0.0
        {
            return None;
        }

        Some(Hit::new(
            ray,
            t,
            self.normal,
            Arc::clone(&self.material),
            projection1,
            projection2,
        ))
    }

    fn ray_to_self_probability(&self, ray: &Ray) -> f32 {
        match self.hit(ray, MAX_T) {
            Some(hit) => {
                let cosine = dot(ray.direction, hit.normal).abs();

                if cosine == 0.0 {
                    0.0
                } else {
                    hit.t.powi(2) / (cosine * self.area())
                }
            }
            None => 0.0,
        }
    }

    fn random_direction_to_self(&self, origin: Point3, time: f32) -> Vec3 {
        (self.random_point_in_self(time) - origin).unit()
    }
}

impl Mobile for Rectangle {
    fn accelerate(&mut self, acceleration: Vec3, initial_velocity: Vec3) {
        self.acceleration = acceleration;
        self.initial_velocity = initial_velocity;
    }
}
