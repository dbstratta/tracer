use rand::prelude::*;

pub fn random<T, R>(range: R) -> T
where
    T: rand::distributions::uniform::SampleUniform,
    R: rand::distributions::uniform::SampleRange<T>,
{
    let mut rng = rand::thread_rng();

    rng.gen_range(range)
}
