use std::sync::{Arc, Mutex};

use rayon::prelude::*;

use crate::{
    camera::Camera,
    helpers::random,
    render::output::save_image,
    rgb_color::{RawRgbColor, RgbColor, BLACK},
    scene::Scene,
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

        let raw_rgb_list = self.render_in_parallel(
            image_height,
            image_width,
            samples_per_pixel,
            max_ray_bounces,
            gamma,
        );

        save_image(path, &raw_rgb_list[..], image_width, image_height)
    }

    fn render_in_parallel(
        &self,
        image_height: u32,
        image_width: u32,
        samples_per_pixel: u32,
        max_ray_bounces: u32,
        gamma: f32,
    ) -> Vec<RawRgbColor> {
        let pixel_count = image_height * image_width;

        let render_data = Arc::new(Mutex::new((vec![BLACK; pixel_count as usize], 0)));

        (0..samples_per_pixel).into_par_iter().for_each(|_| {
            let image_layer: Vec<_> = (0..image_height)
                .rev()
                .flat_map(|y| {
                    (0..image_width)
                        .map(|x| {
                            self.render_sample(image_height, image_width, x, y, max_ray_bounces)
                        })
                        .collect::<Vec<_>>()
                })
                .collect();

            let render_data_clone = Arc::clone(&render_data);
            let mut data = render_data_clone.lock().unwrap();

            (*data).0 = (*data)
                .0
                .iter()
                .zip(image_layer.iter())
                .map(|(&color1, &color2)| color1 + color2 / samples_per_pixel as f32)
                .collect();
            (*data).1 += 1;

            eprint!("\rRendering... {}%", (*data).1 * 100 / samples_per_pixel);
        });

        let render_data_clone = Arc::clone(&render_data);
        let data = render_data_clone.lock().unwrap();

        (*data)
            .0
            .iter()
            .map(|color| color.gamma_correct(gamma).to_raw())
            .collect()
    }

    fn render_sample(
        &self,
        image_height: u32,
        image_width: u32,
        x: u32,
        y: u32,
        max_ray_bounces: u32,
    ) -> RgbColor {
        let s = (x as f32 + random(0.0..1.0)) / (image_width as f32 - 1.0);
        let t = (y as f32 + random(0.0..1.0)) / (image_height as f32 - 1.0);

        let ray = self.camera.cast_ray(s, t);

        ray.trace(&self.scene, max_ray_bounces)
    }
}
