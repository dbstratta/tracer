use crate::{hittables::Hit, ray::Ray};

pub trait Material: Send + Sync {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<Ray>;
}
