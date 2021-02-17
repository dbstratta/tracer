use std::sync::Arc;

use crate::{objects::Object, ray::Ray, rgb_color::RgbColor};

#[derive(Clone)]
pub struct Scene {
    background: fn(&Ray) -> RgbColor,
    pub objects: Arc<dyn Object>,
    pub lights: Arc<dyn Object>,
}

impl Scene {
    pub fn new(
        objects: Arc<dyn Object>,
        lights: Arc<dyn Object>,
        background: fn(&Ray) -> RgbColor,
    ) -> Self {
        Self {
            objects,
            lights,
            background,
        }
    }

    pub fn background(&self, ray: &Ray) -> RgbColor {
        (self.background)(ray)
    }
}
