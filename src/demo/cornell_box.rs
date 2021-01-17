use std::sync::Arc;

use tracer::{
    camera::Camera,
    color::{Color, BLACK},
    hittable::Hittable,
    materials::{Lambertian, Material},
    objects::{Object, ObjectList, Sphere},
    scene::Scene,
    textures::SolidColor,
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
        let mut objects = Vec::<Arc<dyn Object>>::new();
        let mut lights = Vec::<Arc<dyn Object>>::new();

        let sphere = Arc::new(Sphere::new(
            Point3::new(0.0, 0.0, 0.0),
            1.0,
            Arc::new(Material::Reflective(Arc::new(Lambertian::new(
                Arc::new(SolidColor::new(100.0, 2.0)),
                1.0,
            )))),
        ));

        objects.push(sphere);

        Scene::new(
            Arc::new(ObjectList::new(objects)),
            Arc::new(ObjectList::new(lights)),
            |_ray| BLACK,
        )
    }
}
