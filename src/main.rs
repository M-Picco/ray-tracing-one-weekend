mod vec3;
mod color;
mod point3;
mod ray;
mod hittable;
mod sphere;
mod camera;

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

    let res = world.hit(ray, 0.0, f64::INFINITY);

    return match res {
        Some(rec) => {
            let target = rec.point + rec.normal + Vec3::random_in_unit_sphere();
            let child_ray = Ray::new(rec.point, target - rec.point);

            return ray_color(&child_ray, world, depth - 1) * 0.5;
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
    world.add(std::rc::Rc::from(sphere::Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5)
    ));
    world.add(std::rc::Rc::from(sphere::Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0)
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
