use crate::{
    hittable::Hit,
    materials::{ReflectiveMaterial, ScatterResult},
    ray::Ray,
    rgb_color::RgbColor,
    vec3::Vec3,
};

#[derive(Copy, Clone)]
pub struct Metal {
    pub albedo: RgbColor,
    pub fuzziness: f32,
}

impl Metal {
    pub const fn new(albedo: RgbColor, fuzziness: f32) -> Self {
        Self { albedo, fuzziness }
    }
}

impl ReflectiveMaterial for Metal {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> ScatterResult {
        let reflected_direction = ray.direction.reflect(hit.normal).unit();

        let specular_direction =
            (reflected_direction + Vec3::random_in_sphere(1.0) * self.fuzziness).unit();
        let specular_ray = ray.secondary(hit.point, specular_direction);

        ScatterResult::Specular {
            ray: specular_ray,
            attenuation: self.albedo,
        }
    }
}
