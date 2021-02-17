use std::sync::Arc;

use crate::{
    hittable::{Hit, Hittable},
    materials::Material,
    objects::{Object, ObjectList, Rectangle},
    ray::Ray,
    vec3::{cross, Point3, Vec3},
};

#[derive(Clone)]
pub struct RectangularPrism {
    pub front: Arc<Rectangle>,
    pub sides: ObjectList,
}

impl RectangularPrism {
    pub fn new(
        top_left_corner: Point3,
        normal: Vec3,
        up: Vec3,
        height: f32,
        width: f32,
        depth: f32,
        material: Arc<Material>,
    ) -> Self {
        let right = -cross(normal, up);

        let front = Arc::new(Rectangle::new(
            top_left_corner,
            normal,
            up,
            height,
            width,
            Arc::clone(&material),
        ));

        let back = Arc::new(Rectangle::new(
            top_left_corner - depth * normal + width * right,
            -normal,
            up,
            height,
            width,
            Arc::clone(&material),
        ));

        let left_side = Arc::new(Rectangle::new(
            top_left_corner - depth * normal,
            -right,
            up,
            height,
            depth,
            Arc::clone(&material),
        ));

        let right_side = Arc::new(Rectangle::new(
            top_left_corner + width * right,
            right,
            up,
            height,
            depth,
            Arc::clone(&material),
        ));

        let top = Arc::new(Rectangle::new(
            top_left_corner - depth * normal,
            up,
            -normal,
            depth,
            width,
            Arc::clone(&material),
        ));

        let bottom = Arc::new(Rectangle::new(
            top_left_corner - height * up,
            -up,
            normal,
            depth,
            width,
            Arc::clone(&material),
        ));

        let sides = ObjectList::new(vec![
            Arc::clone(&front) as Arc<dyn Object>,
            back,
            left_side,
            right_side,
            top,
            bottom,
        ]);

        Self { front, sides }
    }

    pub fn top_left_corner(&self, time: f32) -> Point3 {
        self.front.top_left_corner(time)
    }
}

impl Object for RectangularPrism {}

impl Hittable for RectangularPrism {
    fn hit(&self, ray: &Ray, max_t: f32) -> Option<Hit> {
        self.sides.hit(ray, max_t)
    }

    fn ray_to_self_probability(&self, ray: &Ray) -> f32 {
        self.sides.ray_to_self_probability(ray)
    }

    fn random_direction_to_self(&self, origin: Point3, time: f32) -> Vec3 {
        self.sides.random_direction_to_self(origin, time)
    }
}
