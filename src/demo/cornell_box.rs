use std::f32::consts::PI;
use std::sync::Arc;

use tracer::{
    camera::Camera,
    materials::{Dielectric, DiffuseLight, Lambertian, Material},
    objects::{Object, ObjectList, RectangleBuilder, RectangularPrismBuilder, SphereBuilder},
    rgb_color::{RgbColor, BLACK, WHITE},
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

    pub fn walls() -> Vec<Arc<dyn Object>> {
        let white_material = Arc::new(Material::Reflective(Arc::new(Lambertian::from_rgb(
            0.85, 0.85, 0.85,
        ))));

        let back = Arc::new(
            RectangleBuilder::new(
                Point3::new(600.0, 600.0, 600.0),
                -Vec3::z(),
                Vec3::y(),
                600.0,
                600.0,
                Arc::clone(&white_material),
            )
            .build(),
        );

        let ceilling = Arc::new(
            RectangleBuilder::new(
                Point3::new(600.0, 600.0, 0.0),
                -Vec3::y(),
                -Vec3::z(),
                600.0,
                600.0,
                Arc::clone(&white_material),
            )
            .build(),
        );

        let floor = Arc::new(
            RectangleBuilder::new(
                Point3::new(600.0, 0.0, 600.0),
                Vec3::y(),
                Vec3::z(),
                600.0,
                600.0,
                Arc::clone(&white_material),
            )
            .build(),
        );

        let left = Arc::new(
            RectangleBuilder::new(
                Point3::new(600.0, 600.0, 0.0),
                -Vec3::x(),
                Vec3::y(),
                600.0,
                600.0,
                Arc::new(Material::Reflective(Arc::new(Lambertian::from_rgb(
                    0.722, 0.427, 0.455,
                )))),
            )
            .build(),
        );

        let right = Arc::new(
            RectangleBuilder::new(
                Point3::new(0.0, 600.0, 600.0),
                Vec3::x(),
                Vec3::y(),
                600.0,
                600.0,
                Arc::new(Material::Reflective(Arc::new(Lambertian::from_rgb(
                    0.463, 0.431, 0.733,
                )))),
            )
            .build(),
        );

        vec![back, ceilling, floor, left, right]
    }
}

impl Demo for CornellBox {
    fn camera(&self) -> Camera {
        Camera::new(
            Point3::new(300.0, 300.0, -800.0),
            Point3::new(300.0, 300.0, 0.0),
            1.0,
            40.0,
            0.0,
            1.0,
            0.2,
            800.0,
        )
    }

    fn scene(&self) -> Scene {
        let mut objects = Vec::<Arc<dyn Object>>::new();
        let mut lights = Vec::<Arc<dyn Object>>::new();

        objects.append(&mut Self::walls());

        let light: Arc<dyn Object> = Arc::new(
            RectangleBuilder::new(
                Point3::new(400.0, 599.9, -200.0),
                Vec3::new(0.0, -1.0, 1.0).unit(),
                -Vec3::z(),
                400.0,
                400.0,
                Arc::new(Material::Emissive(Arc::new(DiffuseLight::from_rgb(
                    16.0, 16.0, 16.0,
                )))),
            )
            .build(),
        );

        objects.push(Arc::clone(&light));
        lights.push(light);

        let sphere = Arc::new(
            SphereBuilder::new(
                Point3::new(200.0, 170.0, 200.0),
                90.0,
                Arc::new(Material::Reflective(Arc::new(Dielectric::new(1.5, WHITE)))),
            )
            .build(),
        );

        objects.push(sphere);

        let box1 = Arc::new(
            RectangularPrismBuilder::new(
                Point3::new(260.0, 60.0, 150.0),
                Vec3::from_polar(0.0, PI - PI / 10.0),
                Vec3::y(),
                60.0,
                150.0,
                150.0,
                Arc::new(Material::Reflective(Arc::new(Lambertian::from_rgb(
                    0.9, 0.9, 0.9,
                )))),
            )
            .build(),
        );

        objects.push(box1);

        let box2 = Arc::new(
            RectangularPrismBuilder::new(
                Point3::new(470.0, 440.0, 200.0),
                Vec3::from_polar(0.0, PI + PI / 8.0),
                Vec3::y(),
                340.0,
                170.0,
                170.0,
                Arc::new(Material::Reflective(Arc::new(Lambertian::from_rgb(
                    0.9, 0.9, 0.9,
                )))),
            )
            .build(),
        );

        objects.push(box2);

        Scene::new(
            Arc::new(ObjectList::new(objects)),
            Arc::new(ObjectList::new(lights)),
            |_ray| BLACK,
        )
    }
}
