use crate::vec3::{Point3, Vec3};

pub trait Mobile {
    fn accelerate(&mut self, acceleration: Vec3, initial_velocity: Vec3);
}

pub fn position(
    time: f32,
    acceleration: Vec3,
    initial_velocity: Vec3,
    initial_position: Point3,
) -> Point3 {
    acceleration * time.powi(2) / 2.0 + initial_velocity * time + initial_position
}
