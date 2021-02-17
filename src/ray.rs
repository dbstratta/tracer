use std::sync::Arc;

use crate::{
    materials::{Material, ScatterResult},
    pdfs::{EmissivePdf, MixturePdf, ScatteringPdf},
    rgb_color::{RgbColor, BLACK},
    scene::Scene,
    vec3::{Point3, Vec3},
};

pub const MIN_T: f32 = 0.0001;
pub const MAX_T: f32 = f32::INFINITY;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
    pub time: f32,
}

impl Ray {
    pub const fn new(origin: Point3, direction: Vec3, time: f32) -> Self {
        Self {
            origin,
            direction,
            time,
        }
    }

    pub fn secondary(&self, origin: Point3, direction: Vec3) -> Self {
        Self::new(origin, direction, self.time)
    }

    pub fn at(&self, t: f32) -> Point3 {
        self.origin + t * self.direction
    }

    pub fn trace(&self, scene: &Scene, depth: u32) -> RgbColor {
        if depth == 0 {
            return BLACK;
        }

        match scene.objects.hit(self, MAX_T) {
            None => scene.background(self),

            Some(hit) => match hit.material.as_ref() {
                Material::Reflective(material) => match material.scatter(self, &hit) {
                    ScatterResult::Diffuse { pdf, attenuation } => {
                        let lights_pdf = Arc::new(EmissivePdf::new(
                            Arc::clone(&scene.lights),
                            hit.point,
                            self.time,
                        ));
                        let mixture_pdf = MixturePdf::new(0.5, lights_pdf, Arc::clone(&pdf));

                        let scattered_ray = self.secondary(hit.point, mixture_pdf.sample());
                        let pdf_value = mixture_pdf.value(scattered_ray.direction);
                        let material_pdf_value = pdf.value(scattered_ray.direction);

                        if !pdf_value.is_normal() || !material_pdf_value.is_normal() {
                            return BLACK;
                        }

                        attenuation
                            * (scattered_ray.trace(scene, depth - 1) / pdf_value)
                            * material_pdf_value
                    }
                    ScatterResult::Specular { ray, attenuation } => {
                        attenuation * ray.trace(scene, depth - 1)
                    }
                },
                Material::Emissive(material) => material.emitted(self, &hit),
            },
        }
    }
}
