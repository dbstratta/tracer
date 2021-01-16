use std::f32::consts::PI;
use std::sync::Arc;

use tracer::{
    camera::Camera,
    color::{Color, BLACK},
    hittables::{Hittable, HittableList, Sphere},
    scene::Scene,
    vec3::{Point3, Vec3},
};

use crate::demo::Demo;

pub struct CornellBox {}

impl CornellBox {
    pub fn new() -> Self {
        Self {}
    }
}

impl Demo for CornellBox {
    fn camera(&self) -> Camera {
        Camera::new(
            Point3::new(10.0, 10.0, 10.0),
            Point3::new(0.0, 0.0, 0.0),
            1.0,
            90.0,
            0.0,
            1.0,
            0.2,
            10.0,
        )
    }

    fn scene(&self) -> Scene {
        let mut objects = Vec::<Arc<dyn Hittable>>::new();
        let mut lights = Vec::<Arc<dyn Hittable>>::new();

        let sphere = Arc::new(Sphere::new(Point3::new(0.0, 0.0, 0.0), 1.0, None));

        objects.push(sphere);

        Scene::new(
            Arc::new(HittableList::new(objects)),
            Arc::new(HittableList::new(lights)),
            |_ray| BLACK,
        )
    }
}
