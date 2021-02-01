use crate::material::Material;
use crate::point3::Point3;
use crate::vec3::Vec3;
use crate::ray::Ray;
use std::rc::Rc;

pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub is_outward: bool,
    pub material: Rc<dyn Material>
}

impl HitRecord {
    pub fn create(point: Point3, t: f64, ray: &Ray, normal: Vec3, material: Rc<dyn Material>) -> HitRecord {
        let mut record = HitRecord {
            point,
            normal,
            t,
            is_outward: false,
            material
        };
        record.set_face_normal(ray, &normal);
        record
    }

    fn set_face_normal(&mut self, ray: &Ray, normal: &Vec3) {
        // si producto < 0 entonces tienen direcciones opuestas 
        // (la normal hacia fuera de la esfera,
        // mientras que el rayo incide en la esfera)
        self.is_outward = ray.dir().dot(normal) < 0.0;
        self.normal = if self.is_outward { *normal } else { -(*normal) }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, tmin: f64, tmax: f64) -> std::option::Option<HitRecord>;
}

pub struct HittableList {
    objects: Vec<Rc<dyn Hittable>>
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList { objects: Vec::new() }
    }

    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.objects.push(object)
    }

    pub fn hit(&self, ray: &Ray, tmin: f64, tmax: f64) -> std::option::Option<HitRecord> {
        self.objects
            .iter()
            .filter_map(|obj| obj.hit(ray, tmin, tmax))
            .min_by(|x, y| x.t.partial_cmp(&y.t).unwrap())
    }
}