use crate::vec3::Vec3;

pub type Color = Vec3;

pub fn encode(color: &Color) -> String {
    let ir: u8 = (255.99 * color.x()) as u8;
    let ig: u8 = (255.99 * color.y()) as u8;
    let ib: u8 = (255.99 * color.z()) as u8;

    format!("{} {} {}\n", ir, ig, ib)
}    
