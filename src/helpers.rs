use rand::prelude::*;

use crate::vec3::{Point3, Vec3};

pub fn random<T, R>(range: R) -> T
where
    T: rand::distributions::uniform::SampleUniform,
    R: rand::distributions::uniform::SampleRange<T>,
{
    let mut rng = rand::thread_rng();

    rng.gen_range(range)
}

pub fn position(
    time: f32,
    acceleration: Vec3,
    initial_velocity: Vec3,
    initial_position: Point3,
) -> Point3 {
    acceleration * time.powi(2) / 2.0 + initial_velocity * time + initial_position
}
