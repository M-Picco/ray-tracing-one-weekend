use crate::vec3::Vec3;
use crate::point3::Point3;
use crate::ray::Ray;

pub struct Camera {
    origin: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Vec3
}

impl Camera {
    pub fn new() -> Camera {
        let ratio = 16.0 / 9.0;
        let viewport_height = 2.0;
        let viewport_width = ratio * viewport_height;
        let focal_length = 1.0;

        let origin = Vec3::origin();
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height, 0.0);
        let lower_left_corner = origin - horizontal / 2 - vertical / 2 - Vec3::new(0.0, 0.0, focal_length);

        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let dir = self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin;
        Ray::new(self.origin, dir)
    }
}
