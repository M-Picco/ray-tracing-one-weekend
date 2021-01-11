mod vec3;
mod color;
mod point3;
mod ray;

use color::Color;
use ray::Ray;
use vec3::Vec3;
use point3::Point3;

fn hit_sphere(center: &Point3, rad: f64, ray: &Ray) -> bool {
    let dir = ray.dir();
    let oc = ray.origin() - *center;

    let a = dir.dot(&dir);
    let b = 2.0 * oc.dot(&dir);
    let c = oc.dot(&oc) - rad * rad;
    let discriminant = b*b - 4.0*a*c; // cuadrática

    discriminant > 0.0
}

fn ray_color(ray: &Ray) -> Color {
    if hit_sphere(&Point3::new(0.0, 0.0, -1.0), 0.5, ray) {
        return Color::new(1.0, 0.0, 0.0);
    }

    let dir = ray.dir().normalize();
    let t = 0.5 * (dir.y() + 1.0);
    Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
}

fn main() {
    // image
    let ratio = 16.0 / 9.0;
    let width = 400;
    let height = (width as f64 / ratio) as usize;

    // camera
    let viewport_height = 2.0;
    let viewport_width = ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Vec3::origin();
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - horizontal / 2 - vertical / 2 - Vec3::new(0.0, 0.0, focal_length);

    // render

    print!("P3\n{} {}\n255\n", width, height);

    for j in (0..height).rev() {
        eprintln!("\rLines remaining: {}", j);
        for i in 0..width {
            // let r = i as f64 / (width - 1) as f64;
            // let g = j as f64 / (height - 1) as f64;
            // let b = 0.25;

            // let color: Color = Color::new(r, g, b);
            // print!("{}", color::encode(&color));
            let u = i as f64 / (width - 1) as f64;
            let v = j as f64 / (height - 1) as f64;

            let ray = Ray::new(origin, lower_left_corner + horizontal * u + vertical * v - origin);

            let pixel_color = ray_color(&ray);
            print!("{}", color::encode(&pixel_color));
        }
    }
}
