mod vec3;
mod color;
mod point3;
mod ray;
mod hittable;
mod sphere;
mod camera;
mod material;
mod math;

use crate::material::Dielectric;
use crate::material::Metal;
use std::rc::Rc;
use crate::material::Lambertian;
use color::Color;
use rand::Rng;
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

fn random_double() -> f64 {
    rand::random::<f64>()
}

fn random_double_in_range(low: f64, high: f64) -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(low, high)
}

fn random_scene() -> HittableList {
    let mut world = HittableList::new();

    let ground_material = Rc::from(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Rc::new(sphere::Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material
    )));

    for a in -11..12 {
        for b in -11..12 {
            let choose_mat = rand::random::<f64>();
            let center = Point3::new(
                (a as f64) + 0.9 * random_double(),
                0.2,
                (b as f64) + 0.9 * random_double()
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).norm() > 0.9 {
                if choose_mat < 0.8 {
                    //diffuse
                    let albedo = Color::random(0.0, 1.0) * Color::random(0.0, 1.0);
                    world.add(Rc::new(sphere::Sphere::new(
                        center,
                        0.2,
                        Rc::new(Lambertian::new(albedo))
                    )));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random(0.5, 1.0);
                    let fuzz = random_double_in_range(0.0, 0.5);
                    world.add(Rc::new(sphere::Sphere::new(
                        center,
                        0.2,
                        Rc::new(Metal::new(albedo, fuzz))
                    )));
                } else {
                    // glass
                    world.add(Rc::new(sphere::Sphere::new(
                        center,
                        0.2,
                        Rc::new(Dielectric::new(1.5))
                    )));
                }

            }
        }
    }

    world.add(Rc::new(sphere::Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        Rc::new(Dielectric::new(1.5))
    )));
    world.add(Rc::new(sphere::Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        Rc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)))
    )));
    world.add(Rc::new(sphere::Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0))
    )));


    world
}

fn main() {
    // image
    let aspect_ratio = 3.0 / 2.0;
    let width = 1200;
    let height = (width as f64 / aspect_ratio) as usize;
    let samples_per_pixel = 100;
    let max_depth = 50;

    // world
    let world = random_scene();

    // camera
    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::origin();
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    let camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        aspect_ratio,
        aperture,
        dist_to_focus
    );

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
