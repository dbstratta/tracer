use rand::prelude::*;

pub fn random<T, R>(range: R) -> T
where
    T: rand::distributions::uniform::SampleUniform,
    R: rand::distributions::uniform::SampleRange<T>,
{
    let mut rng = rand::thread_rng();

    rng.gen_range(range)
}

// TODO: rust 1.50 has this built-in
pub fn clamp(number: f32, min: f32, max: f32) -> f32 {
    if number > max {
        max
    } else if number < min {
        min
    } else {
        number
    }
}
