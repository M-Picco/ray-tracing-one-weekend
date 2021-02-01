use crate::Vec3;
use super::ray::Ray;
use super::color::Color;
use super::hittable::HitRecord;

pub trait Material {
    fn scatter(&self, ray: &Ray, attenuation: &mut Color, record: &HitRecord) -> Option<Ray>;
}

pub struct Lambertian {
    pub albedo: Color
}

impl Lambertian {
    pub fn new(color: Color) -> Lambertian {
        Lambertian { albedo: color }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _: &Ray, attenuation: &mut Color, record: &HitRecord) -> Option<Ray> {
        let mut scatter_direction = record.normal + Vec3::random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = record.normal;
        }

        let scattered = Ray::new(record.point, scatter_direction);
        *attenuation = self.albedo;
        Option::from(scattered)
    }
}

pub struct Metal {
    pub albedo: Color
}

impl Metal {
    pub fn new(color: Color) -> Metal {
        Metal { albedo: color }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, attenuation: &mut Color, record: &HitRecord) -> Option<Ray> {
        // v - 2 * dot(v,n) * n
        let reflected = ray.dir() - record.normal * ray.dir().dot(&record.normal) * 2.0;        
        let scattered = Ray::new(record.point, reflected);

        *attenuation = self.albedo;
        if scattered.dir().dot(&record.normal) > 0.0 {
            return Option::from(scattered)
        }

        None
    }
}
