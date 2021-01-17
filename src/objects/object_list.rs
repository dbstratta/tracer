use std::sync::Arc;

use crate::{
    hittable::{Hit, Hittable},
    mobile::Mobile,
    objects::Object,
    ray::Ray,
    vec3::Vec3,
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
}
