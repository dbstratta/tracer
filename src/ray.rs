use crate::{
    color::{Color, BLACK},
    scene::Scene,
    vec3::{Point3, Vec3},
};

pub const MIN_T: f32 = 0.0001;
pub const MAX_T: f32 = f32::INFINITY;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
    pub wavelength: f32,
    pub probability: f32,
    pub time: f32,
}

impl Ray {
    pub fn new(
        origin: Point3,
        direction: Vec3,
        wavelength: f32,
        probability: f32,
        time: f32,
    ) -> Self {
        Self {
            origin,
            direction,
            wavelength,
            probability,
            time,
        }
    }

    pub fn at(&self, t: f32) -> Point3 {
        self.origin + t * self.direction
    }

    pub fn color(&self, scene: &Scene, depth: u32) -> Color {
        if depth == 0 {
            return BLACK;
        }

        match scene.objects.hit(self, MAX_T) {
            None => scene.background(self),
            Some(hit) => Color::new(0.0, 1.0, 0.0),
        }
    }
}
