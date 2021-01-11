use crate::vec3::Vec3;
use crate::point3::Point3;

pub struct Ray {
    origin: Point3,
    dir: Vec3
}

impl Ray {
    pub fn new(origin: Point3, dir: Vec3) -> Ray {
        Ray { origin, dir }
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + self.dir * t
    }

    pub fn dir(&self) -> Vec3 {
        self.dir
    }

    pub fn origin(&self) -> Vec3 {
        self.origin
    }
}
