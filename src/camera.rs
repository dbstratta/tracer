use crate::{
    helpers::random,
    ray::Ray,
    vec3::{cross, Point3, Vec3},
};

const VIEW_UP: Vec3 = Vec3::new(0.0, 1.0, 0.0);

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Camera {
    pub origin: Point3,
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
    pub viewport_horizontal: Vec3,
    pub viewport_vertical: Vec3,
    pub viewport_lower_left_corner: Point3,

    pub aspect_ratio: f32,
    pub time: f32,
    pub shutter_speed: f32,
    pub aperture: f32,
}

impl Camera {
    pub fn new(
        look_from: Point3,
        look_at: Point3,
        aspect_ratio: f32,
        vertical_fov: f32,
        time: f32,
        shutter_speed: f32,
        aperture: f32,
        focus_distance: f32,
    ) -> Self {
        let viewport_height = 2.0 * f32::tan(vertical_fov.to_radians() / 2.0) * focus_distance;
        let viewport_width = viewport_height * aspect_ratio;

        let w = (look_from - look_at).unit();
        let u = cross(VIEW_UP, w);
        let v = cross(w, u);

        let origin = look_from;

        let viewport_horizontal = u * viewport_width;
        let viewport_vertical = v * viewport_height;
        let viewport_lower_left_corner =
            origin - viewport_horizontal / 2.0 - viewport_vertical / 2.0 - w * focus_distance;

        Self {
            origin,
            aspect_ratio,

            time,
            shutter_speed,
            aperture,

            u,
            v,
            w,

            viewport_horizontal,
            viewport_vertical,
            viewport_lower_left_corner,
        }
    }

    pub fn cast_ray(&self, s: f32, t: f32) -> Ray {
        let point_in_aperture_sphere = Point3::random_in_sphere(self.aperture / 2.0);
        let origin_offset =
            point_in_aperture_sphere.x * self.u + point_in_aperture_sphere.y * self.v;

        let ray_origin = self.origin + origin_offset;
        let ray_direction = (self.viewport_lower_left_corner
            + self.viewport_horizontal * s
            + self.viewport_vertical * t
            - ray_origin)
            .unit();

        Ray::new(ray_origin, ray_direction, self.random_time())
    }

    fn random_time(&self) -> f32 {
        random(self.time..(self.time + self.shutter_speed))
    }
}
