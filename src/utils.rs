use nalgebra::*;
use vek::*;

pub fn enccol_rgb(r: u8, g: u8, b: u8) -> u32 {
    (r as u32) << 16 | (g as u32) << 8 | (b as u32)
}
pub fn enccol_rgb_vec3(rgb: Vec3<u8>) -> u32 {
    enccol_rgb(rgb.x, rgb.y, rgb.z)
}
pub fn deccol(color: u32) -> Vec3<u8> {
    let r = ((color >> 16) & 0xFF) as u8;
    let g = ((color >> 8) & 0xFF) as u8;
    let b = (color & 0xFF) as u8;
    Vec3::new(r, g, b)
}
pub trait Rotate {
    fn rotate(&self, axis: Vec3<f32>, theta: f32) -> Vec3<f32>;
}
impl Rotate for Vec3<f32> {
    fn rotate(&self, axis: Vec3<f32>, theta: f32) -> Vec3<f32> {
        let mat = Mat3::rotation_3d(theta, axis);
        *self * mat
    }
}
