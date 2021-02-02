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
        let reflected = ray.dir().reflect(&record.normal);
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

impl Dielectric {
    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        // Schlick's approximation
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * f64::powi(1.0 - cosine, 5)
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
        let cos_theta = f64::min((-unit_dir).dot(&record.normal), 1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let cannot_refract = refraction_ratio * sin_theta > 1.0;

        let reflectance = Dielectric::reflectance(cos_theta, refraction_ratio);

        let direction = if cannot_refract || reflectance > rand::random::<f64>() {
            unit_dir.reflect(&record.normal)
        } else {
            unit_dir.refract(&record.normal, refraction_ratio)
        };

        let scattered = Ray::new(record.point, direction);

        Some(scattered)
    }
}
