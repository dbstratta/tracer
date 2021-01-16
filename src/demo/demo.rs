use std::sync::Arc;

use tracer::{camera::Camera, render::Renderer, scene::Scene};

pub trait Demo {
    fn camera(&self) -> Camera;
    fn scene(&self) -> Scene;

    fn render(
        &self,
        path: &str,
        image_height: u32,
        samples_per_pixel: u32,
        max_ray_bounces: u32,
        gamma: f32,
    ) -> Result<(), image::ImageError> {
        let renderer = Renderer::new(Arc::new(self.camera()), Arc::new(self.scene()));

        renderer.render(
            path,
            image_height,
            samples_per_pixel,
            max_ray_bounces,
            gamma,
        )
    }
}
