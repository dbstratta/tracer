use crate::hittable::Hittable;

pub trait Object: Hittable + Send + Sync {}
