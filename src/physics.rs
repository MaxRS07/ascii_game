use vek::*;

#[derive(Clone, Copy)]
pub struct Material {
    pub color: u32,
}
impl Material {
    pub fn new(color: u32) -> Self {
        Material { color }
    }
    pub fn empty() -> Self {
        Material { color: 0u32 }
    }
}
pub struct HitInfo {
    pub hit_name: String,
    pub phit: Vec3<f32>,
    pub dist: f32,
    pub material: Material,
}
impl HitInfo {
    pub fn new(hit_name: String, phit: Vec3<f32>, dist: f32, material: Material) -> Self {
        HitInfo {
            hit_name,
            phit,
            dist,
            material,
        }
    }
    pub fn empty() -> Self {
        HitInfo {
            hit_name: "".to_string(),
            phit: Vec3::zero(),
            dist: 0f32,
            material: Material::empty(),
        }
    }
}
