use std::fs::File;
use std::io::{Error, Write};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use rayon::prelude::*;

use crate::{
    camera::Camera,
    cie::CieTristimulus,
    helpers,
    photon::Photon,
    render::{output::save_image, Pixel},
    rgb_color::{RawRgbColor, RgbColor},
    scene::Scene,
    tonemap::tonemap,
    vec3::{Point3, Vec3},
};

#[derive(Clone)]
pub struct Renderer {
    pub camera: Arc<Camera>,
    pub scene: Arc<Scene>,
}

impl Renderer {
    pub const fn new(camera: Arc<Camera>, scene: Arc<Scene>) -> Self {
        Self { camera, scene }
    }

    pub fn render(
        &self,
        path: &str,
        image_height: u32,
        samples_per_pixel: u32,
        max_ray_bounces: u32,
        gamma: f32,
    ) -> image::ImageResult<()> {
        let image_width = f32::round(image_height as f32 * self.camera.aspect_ratio) as u32;

        let rgb_list = self.render_in_parallel(
            image_height,
            image_width,
            samples_per_pixel,
            max_ray_bounces,
            gamma,
        );

        save_image(path, &rgb_list[..], image_width, image_height)
    }

    fn render_in_parallel(
        &self,
        image_height: u32,
        image_width: u32,
        samples_per_pixel: u32,
        max_ray_bounces: u32,
        gamma: f32,
    ) -> Vec<RawRgbColor> {
        let progresses = Arc::new(Mutex::new(vec![false; image_height as usize]));

        let rgb_list: Vec<CieTristimulus> = rayon::scope(|_| {
            (0..image_height)
                .into_par_iter()
                .rev()
                .flat_map(|y| {
                    let rgb_row: Vec<CieTristimulus> = (0..image_width)
                        .map(|x| {
                            self.render_pixel(
                                samples_per_pixel,
                                image_height,
                                image_width,
                                x,
                                y,
                                max_ray_bounces,
                            )
                            .cie_tristimulus()
                        })
                        .collect();

                    let progresses_clone = Arc::clone(&progresses);
                    let mut progresses_guard = progresses_clone.lock().unwrap();
                    (*progresses_guard)[y as usize] = true;

                    if y % 5 == 0 {
                        let progress = (progresses_guard
                            .iter()
                            .filter(|&&y| y)
                            .collect::<Vec<&bool>>()
                            .len()
                            * 100)
                            / progresses_guard.len();

                        eprint!("\rRendering... {}%", progress);
                    }

                    rgb_row
                })
                .collect()
        });

        // tonemap(&rgb_list[..])
        rgb_list
            .iter()
            .map(|&d| {
                RgbColor::from_cie_tristimulus(d)
                    .gamma_correct(gamma)
                    .to_raw()
            })
            .collect()
    }

    fn render_pixel(
        &self,
        samples_per_pixel: u32,
        image_height: u32,
        image_width: u32,
        x: u32,
        y: u32,
        max_ray_bounces: u32,
    ) -> Pixel {
        Pixel::new(
            (0..samples_per_pixel)
                .map(|_| self.render_sample(image_height, image_width, x, y, max_ray_bounces))
                .collect(),
        )
    }

    fn render_sample(
        &self,
        image_height: u32,
        image_width: u32,
        x: u32,
        y: u32,
        max_ray_bounces: u32,
    ) -> Photon {
        let s = (x as f32) / (image_width as f32 - 1.0);
        let t = (y as f32) / (image_height as f32 - 1.0);

        let ray = self.camera.cast_ray(s, t);

        Photon::new(ray.wavelength, ray.trace(&self.scene, max_ray_bounces))
    }
}
