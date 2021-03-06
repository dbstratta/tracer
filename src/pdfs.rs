use std::f32::consts::PI;
use std::sync::Arc;

use crate::{
    helpers::random,
    objects::Object,
    onb::Onb,
    ray::Ray,
    vec3::{dot, Point3, Vec3},
};

pub trait ScatteringPdf {
    fn value(&self, direction: Vec3) -> f32;
    fn sample(&self) -> Vec3;
}

pub struct CosinePdf {
    pub onb: Onb,
}

impl CosinePdf {
    pub fn new(w: Vec3) -> Self {
        Self {
            onb: Onb::from_w(w),
        }
    }
}

impl ScatteringPdf for CosinePdf {
    fn value(&self, direction: Vec3) -> f32 {
        let cosine = dot(direction, self.onb.w());

        if cosine <= 0.0 {
            0.0
        } else {
            cosine / PI
        }
    }

    fn sample(&self) -> Vec3 {
        self.onb.local(random_cosine_direction()).unit()
    }
}

fn random_cosine_direction() -> Vec3 {
    let r1 = random(0.0..1.0);
    let r2 = random(0.0..1.0);
    let z = f32::sqrt(1.0 - r2);

    let phi = 2.0 * PI * r1;

    let x = f32::cos(phi) * f32::sqrt(r2);
    let y = f32::sin(phi) * f32::sqrt(r2);

    Vec3::new(x, y, z)
}

pub struct EmissivePdf {
    pub object: Arc<dyn Object>,
    pub origin: Point3,
    pub time: f32,
}

impl EmissivePdf {
    pub fn new(object: Arc<dyn Object>, origin: Point3, time: f32) -> Self {
        Self {
            object,
            origin,
            time,
        }
    }
}

impl ScatteringPdf for EmissivePdf {
    fn value(&self, direction: Vec3) -> f32 {
        let ray = Ray::new(self.origin, direction, self.time);

        self.object.ray_to_self_probability(&ray)
    }

    fn sample(&self) -> Vec3 {
        self.object.random_direction_to_self(self.origin, self.time)
    }
}

pub struct MixturePdf {
    pub factor: f32,
    pub pdf1: Arc<dyn ScatteringPdf>,
    pub pdf2: Arc<dyn ScatteringPdf>,
}

impl MixturePdf {
    pub fn new(factor: f32, pdf1: Arc<dyn ScatteringPdf>, pdf2: Arc<dyn ScatteringPdf>) -> Self {
        Self { factor, pdf1, pdf2 }
    }
}

impl ScatteringPdf for MixturePdf {
    fn value(&self, direction: Vec3) -> f32 {
        self.factor * self.pdf1.value(direction) + (1.0 - self.factor) * self.pdf2.value(direction)
    }

    fn sample(&self) -> Vec3 {
        if random(0.0..1.0) <= self.factor {
            self.pdf1.sample()
        } else {
            self.pdf2.sample()
        }
    }
}
