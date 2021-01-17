use std::sync::Arc;

use crate::{color::Color, objects::Object, ray::Ray};

#[derive(Clone)]
pub struct Scene {
    background: fn(&Ray) -> Color,
    pub objects: Arc<dyn Object>,
    pub lights: Arc<dyn Object>,
}

impl Scene {
    pub fn new(
        objects: Arc<dyn Object>,
        lights: Arc<dyn Object>,
        background: fn(&Ray) -> Color,
    ) -> Self {
        Self {
            objects,
            lights,
            background,
        }
    }

    pub fn background(&self, ray: &Ray) -> Color {
        (self.background)(ray)
    }
}
