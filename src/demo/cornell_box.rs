use std::sync::Arc;

use tracer::{
    camera::Camera,
    materials::{BlackBody, Lambertian, Material},
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
                Arc::new(SolidColor::new(400.0, 100.0)),
                1.0,
            )))),
        ));

        objects.push(sphere);

        let sphere2: Arc<dyn Object> = Arc::new(Sphere::new(
            Point3::new(0.0, -2.0, 0.0),
            1.0,
            // Arc::new(Material::Reflective(Arc::new(Lambertian::new(
            //     Arc::new(SolidColor::new(400.0, 100.0)),
            //     10.0,
            // )))),
            Arc::new(Material::Emissive(Arc::new(BlackBody::new(5000.0, 1.0)))),
        ));

        // objects.push(Arc::clone(&sphere2));
        // lights.push(sphere2);

        let light: Arc<dyn Object> = Arc::new(Sphere::new(
            Point3::new(0.0, 10.0, 0.0),
            7.0,
            Arc::new(Material::Emissive(Arc::new(BlackBody::new(3000.0, 1.0)))),
        ));

        objects.push(Arc::clone(&light));
        lights.push(light);

        Scene::new(
            Arc::new(ObjectList::new(objects)),
            Arc::new(ObjectList::new(lights)),
            |_ray| 0.0,
        )
    }
}
