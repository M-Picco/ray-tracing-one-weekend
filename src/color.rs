use crate::vec3::Vec3;

pub type Color = Vec3;

fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min { return min }
    if x > max { return max }
    x
}

pub fn encode(color: &Color, samples: usize) -> String {
    let mut r = color.x();
    let mut g = color.y();
    let mut b = color.z();

    let scale = 1.0 / samples as f64;
    r *= scale;
    g *= scale;
    b *= scale;

    let ir: u8 = (255.99 * clamp(r, 0.0, 0.999)) as u8;
    let ig: u8 = (255.99 * clamp(g, 0.0, 0.999)) as u8;
    let ib: u8 = (255.99 * clamp(b, 0.0, 0.999)) as u8;

    format!("{} {} {}\n", ir, ig, ib)
}    
