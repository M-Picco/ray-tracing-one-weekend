mod vec3;
mod color;
mod point3;
mod ray;
mod hittable;
mod sphere;
mod camera;
mod material;

use crate::material::Metal;
use std::rc::Rc;
use crate::material::Lambertian;
use color::Color;
use ray::Ray;
use hittable::HittableList;
use point3::Point3;
use camera::Camera;
use vec3::Vec3;

/**
 * Determina el color del pixel en base a la incidencia del rayo en la esfera
 *  - Si no incide, se utiliza el gradiente de azul a blanco.
 *  - Si incide, se usa la normal a la superficie en el punto donde incide el rayo
 * y se arma un gradiente en base a las componentes de la normal
*/
fn ray_color(ray: &Ray, world: &HittableList, depth: i32) -> Color {
    if depth <= 0 {
        return Color::black();
    }

    let res = world.hit(ray, 0.001, f64::INFINITY);

    return match res {
        Some(rec) => {
            let mut attenuation: Color = Color::black();
            return match rec.material.scatter(ray, &mut attenuation, &rec) {
                Some(scattered_ray) => attenuation * ray_color(&scattered_ray, world, depth - 1),
                None => Color::black()
            }
        },
        None => {
            let dir = ray.dir().normalize();
            let t = 0.5 * (dir.y() + 1.0);
            Color::white() * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
        }
    };
}

fn main() {
    // image
    let ratio = 16.0 / 9.0;
    let width = 400;
    let height = (width as f64 / ratio) as usize;
    let samples_per_pixel = 100;
    let max_depth = 50;

    // world
    let mut world = HittableList::new();

    let material_ground = Rc::from(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Rc::from(Lambertian::new(Color::new(0.7, 0.3, 0.3)));
    let material_left = Rc::from(Metal::new(Color::new(0.8, 0.8, 0.8)));
    let material_right = Rc::from(Metal::new(Color::new(0.8, 0.6, 0.2)));

    world.add(Rc::from(sphere::Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground)
    ));
    world.add(Rc::from(sphere::Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
        material_center)
    ));
    world.add(Rc::from(sphere::Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left)
    ));
    world.add(Rc::from(sphere::Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        material_right)
    ));

    // camera
    let camera = Camera::new();

    // render

    print!("P3\n{} {}\n255\n", width, height);

    for j in (0..height).rev() {
        eprintln!("\rLines remaining: {}", j);
        for i in 0..width {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);

            for _ in 0..samples_per_pixel {
                let u = (i as f64 + rand::random::<f64>()) / (width - 1) as f64;
                let v = (j as f64 + rand::random::<f64>()) / (height - 1) as f64;

                let ray = camera.get_ray(u, v);

                pixel_color += ray_color(&ray, &world, max_depth);
            }

            print!("{}", color::encode(&pixel_color, samples_per_pixel));
        }
    }
}
