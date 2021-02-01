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
    pub albedo: Color,
    pub fuzz: f64
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Metal {
        Metal { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, attenuation: &mut Color, record: &HitRecord) -> Option<Ray> {
        // v - 2 * dot(v,n) * n
        let reflected = ray.dir() - record.normal * ray.dir().dot(&record.normal) * 2.0;        
        let scattered = Ray::new(record.point, reflected + self.fuzz * Vec3::random_in_unit_sphere());

        *attenuation = self.albedo;
        if scattered.dir().dot(&record.normal) > 0.0 {
            return Option::from(scattered)
        }

        None
    }
}

pub struct Dielectric {
    pub refraction_index: f64
}

impl Dielectric {
    pub fn new(ir: f64) -> Dielectric {
        Dielectric { refraction_index: ir }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, attenuation: &mut Color, record: &HitRecord) -> Option<Ray> {
        *attenuation = Color::white();

        let mut refraction_ratio = self.refraction_index;
        if record.is_outward { 
            refraction_ratio = 1.0 / refraction_ratio;
        }
        
        let unit_dir = ray.dir().normalize();

        // refract
        let cos_theta = f64::min((-unit_dir).dot(&record.normal), 1.0);
        let r_out_perp = refraction_ratio * (unit_dir + cos_theta * record.normal);
        let r_out_parallel = -(1.0 - r_out_perp.squared_norm()).abs().sqrt() * record.normal;
        let refracted = r_out_perp + r_out_parallel;

        let scattered = Ray::new(record.point, refracted);

        Some(scattered)
    }
}
