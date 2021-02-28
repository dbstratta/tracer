use crate::{
    helpers::random,
    hittable::Hit,
    materials::{ReflectiveMaterial, ScatterResult},
    ray::Ray,
    rgb_color::RgbColor,
    vec3::dot,
};

#[derive(Copy, Clone)]
pub struct Dielectric {
    pub refractive_index: f32,
    pub color: RgbColor,
}

impl Dielectric {
    pub const fn new(refractive_index: f32, color: RgbColor) -> Self {
        Self {
            refractive_index,
            color,
        }
    }

    fn reflectance(cos_theta: f32, refraction_ratio: f32) -> f32 {
        let r0 = ((1.0 - refraction_ratio) / (1.0 + refraction_ratio)).powi(2);

        r0 + (1.0 - r0) * (1.0 - cos_theta).powi(5)
    }
}

impl ReflectiveMaterial for Dielectric {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> ScatterResult {
        let refraction_ratio = if hit.front_face {
            1.0 / self.refractive_index
        } else {
            self.refractive_index
        };

        let unit_direction = ray.direction;

        let cos_theta = f32::min(dot(-unit_direction, hit.normal), 1.0);
        let sin_theta = f32::sqrt(1.0 - cos_theta.powi(2));

        let can_refract = refraction_ratio * sin_theta <= 1.0;

        let specular_direction = if can_refract
            && Dielectric::reflectance(cos_theta, refraction_ratio) <= random(0.0..1.0)
        {
            unit_direction.refract(hit.normal, refraction_ratio)
        } else {
            unit_direction.reflect(hit.normal)
        }
        .unit();

        let specular_ray = ray.secondary(hit.point, specular_direction);

        ScatterResult::Specular {
            ray: specular_ray,
            attenuation: self.color,
        }
    }
}
