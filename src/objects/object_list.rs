use std::sync::Arc;

use crate::{
    helpers::random,
    hittable::{Hit, Hittable},
    objects::Object,
    ray::Ray,
    vec3::{Point3, Vec3},
};

#[derive(Clone)]
pub struct ObjectList {
    pub objects: Vec<Arc<dyn Object>>,
}

impl ObjectList {
    pub fn new(objects: Vec<Arc<dyn Object>>) -> Self {
        Self { objects }
    }
}

impl Object for ObjectList {}

impl Hittable for ObjectList {
    fn hit(&self, ray: &Ray, max_t: f32) -> Option<Hit> {
        let mut smallest_t = max_t;
        let mut closest_hit: Option<Hit> = None;

        for object in self.objects.iter() {
            if let Some(hit) = object.hit(ray, smallest_t) {
                smallest_t = hit.t;
                closest_hit = Some(hit);
            };
        }

        closest_hit
    }

    fn ray_to_self_probability(&self, ray: &Ray) -> f32 {
        if self.objects.is_empty() {
            return 0.0;
        }

        let weight = 1.0 / self.objects.len() as f32;

        let sum = self.objects.iter().fold(0.0, |accumulator, object| {
            accumulator + weight * object.ray_to_self_probability(ray)
        });

        sum
    }

    fn random_direction_to_self(&self, origin: Point3, time: f32) -> Vec3 {
        if self.objects.is_empty() {
            Vec3::random()
        } else {
            self.objects[random(0..self.objects.len())].random_direction_to_self(origin, time)
        }
    }
}
