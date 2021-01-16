use std::sync::Arc;

use crate::{
    hittables::{Hit, Hittable},
    ray::Ray,
};

#[derive(Clone)]
pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>,
}

impl HittableList {
    pub fn new(objects: Vec<Arc<dyn Hittable>>) -> Self {
        Self { objects }
    }
}

impl Hittable for HittableList {
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
}
