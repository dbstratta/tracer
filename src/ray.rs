use std::sync::Arc;

use crate::{
    helpers::random,
    materials::{Material, ScatterResult},
    objects::Object,
    pdfs::{self, ScatteringPdf},
    scene::Scene,
    vec3::{Point3, Vec3},
};

pub const MIN_T: f32 = 0.0001;
pub const MAX_T: f32 = f32::INFINITY;

pub const MIN_WAVELENGTH: f32 = 380.0;
pub const MAX_WAVELENGTH: f32 = 700.0;

pub const SHADOW_RAYS_PER_LIGHT: u32 = 5;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
    pub wavelength: f32,
    pub probability: f32,
    pub time: f32,
}

impl Ray {
    pub const fn new(
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

    pub fn primary(origin: Point3, direction: Vec3, time: f32) -> Self {
        Self::new(origin, direction, Self::random_wavelength(), 1.0, time)
    }

    pub fn secondary(&self, origin: Point3, direction: Vec3, probability: f32) -> Self {
        Self::new(
            origin,
            direction,
            self.wavelength,
            self.probability * probability,
            self.time,
        )
    }

    pub fn at(&self, t: f32) -> Point3 {
        self.origin + t * self.direction
    }

    pub fn trace(&self, scene: &Scene, depth: u32) -> f32 {
        if depth == 0 {
            return 0.0;
        }

        match scene.objects.hit(self, MAX_T) {
            None => scene.background(self),
            Some(hit) => match hit.material.as_ref() {
                Material::Reflective(material) => match material.scatter(self, &hit) {
                    None => 0.0,
                    Some(ScatterResult::Diffuse {
                        pdf,
                        scatter_probability,
                    }) => {
                        let lights_pdf = Arc::new(pdfs::Emissive::new(
                            Arc::clone(&scene.lights),
                            hit.point,
                            self.time,
                        ));
                        let mixture_pdf = pdfs::Mixture::new(0.5, lights_pdf, pdf);

                        let scattered_ray =
                            self.secondary(hit.point, mixture_pdf.sample(), scatter_probability);
                        let pdf_value = mixture_pdf.value(scattered_ray.direction);

                        scattered_ray.trace(scene, depth - 1) * pdf_value
                    }
                    Some(ScatterResult::Specular { ray }) => ray.trace(scene, depth - 1),
                },
                Material::Emissive(material) => {
                    self.probability * material.intensity(self.wavelength)
                }
            },
        }
    }

    pub fn random_wavelength() -> f32 {
        random(MIN_WAVELENGTH..=MAX_WAVELENGTH)
    }
}
