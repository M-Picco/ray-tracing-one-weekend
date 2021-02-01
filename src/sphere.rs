use std::rc::Rc;
use crate::material::Material;
use crate::point3::Point3;
use crate::hittable::{Hittable, HitRecord};
use crate::ray::Ray;

pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Rc<dyn Material>
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Rc<dyn Material>) -> Sphere {
        Sphere { center, radius, material }
    }
}

/**
 * Surge de calcular el "t" tal que el rayo incide en la esfera
 * La ecuación es: (P(t) - C) * (P(t) - C) = r**2
 * Donde: 
 *  - P(t) = A + t * b, A el origen del rayo y b el vector direccion
 *  - C el centro de la esfera
 *  - r el radio de la esfera
 * Expandiendo la ecuación original se llega a una cuadrática 
*/
impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, tmin: f64, tmax: f64) -> std::option::Option<HitRecord> {
        let dir = ray.dir();
        let oc = ray.origin() - self.center;
    
        let a = dir.squared_norm();
        let half_b = oc.dot(&dir);
        let c = oc.squared_norm() - self.radius * self.radius;

        let discriminant = half_b*half_b - a*c;
        if discriminant < 0.0 {
            return None
        }
        let sqrtd = discriminant.sqrt();

        // raiz más cercana
        let mut root = (-half_b - sqrtd) / a;
        if root < tmin || tmax < root {
            root = (-half_b + sqrtd) / a;
            if root < tmin || tmax < root {
                return None;
            }
        }

        let point = ray.at(root);
        let outward_normal = (point - self.center) / self.radius;
        return Some(HitRecord::create(point, root, ray, outward_normal, Rc::clone(&self.material)))
    }
}
