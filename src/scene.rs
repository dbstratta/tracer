use std::sync::Arc;

use crate::{objects::Object, ray::Ray};

#[derive(Clone)]
pub struct Scene {
    background: fn(&Ray) -> f32,
    pub objects: Arc<dyn Object>,
    pub lights: Arc<dyn Object>,
}

impl Scene {
    pub fn new(
        objects: Arc<dyn Object>,
        lights: Arc<dyn Object>,
        background: fn(&Ray) -> f32,
    ) -> Self {
        Self {
            objects,
            lights,
            background,
        }
    }

    pub fn background(&self, ray: &Ray) -> f32 {
        (self.background)(ray)
    }
}
