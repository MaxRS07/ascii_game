use crate::physics::*;
use num_traits::abs;
use vek::*;

pub struct Triangle {
    pub a: Vec3<f32>,
    pub b: Vec3<f32>,
    pub c: Vec3<f32>,
    pub material: Material,
}
impl Triangle {
    pub fn new(a: Vec3<f32>, b: Vec3<f32>, c: Vec3<f32>, material: Material) -> Self {
        Triangle { a, b, c, material }
    }
}

pub trait GameObject {
    fn center(&self) -> Vec3<f32>;
    fn colliding(&self, ray: Ray<f32>) -> Option<HitInfo>;
    fn get_name(&self) -> String;
}
impl GameObject for Triangle {
    fn center(&self) -> Vec3<f32> {
        (self.a + self.b + self.c) / 3f32
    }
    fn colliding(&self, ray: Ray<f32>) -> Option<HitInfo> {
        let mut e1: Vec3<f32> = Vec3::zero();
        let mut e2: Vec3<f32> = Vec3::zero();
        let mut h: Vec3<f32> = Vec3::zero();
        let mut s: Vec3<f32> = Vec3::zero();
        let mut q: Vec3<f32> = Vec3::zero();

        let a: f32;
        let f: f32;
        let u: f32;
        let v: f32;
        let t: f32;

        e1 = self.b - self.a;
        e2 = self.c - self.a;

        h = ray.direction.cross(e2);
        a = e1.dot(h);

        if abs(a) < 0.01 {
            return None;
        }

        f = 1.0 / a;
        s = ray.origin - self.a;
        u = f * s.dot(h);

        if u < 0.0 || u > 1.0 {
            return None;
        }

        q = s.cross(e1);
        v = f * ray.direction.dot(q);

        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        t = f * e2.dot(q);

        if t > 0.01 {
            return Some(HitInfo::new(
                self.get_name(),
                ray.origin + ray.direction * t,
                t,
                self.material,
            ));
        } else {
            return None;
        }
    }
    fn get_name(&self) -> String {
        return "".to_string();
    }
}
pub trait Polygon {
    fn construct(&self) -> Vec<Triangle>;
}
pub struct Plane {
    pub max: Vec3<f32>,
    pub min: Vec3<f32>,
    pub material: Material,
}
impl Plane {
    pub fn new(min: Vec3<f32>, max: Vec3<f32>, material: Material) -> Self {
        Plane { max, min, material }
    }
}
impl Polygon for Plane {
    fn construct(&self) -> Vec<Triangle> {
        let v1 = Vec3::new(self.min.x, self.max.y, self.max.z);
        let v2 = Vec3::new(self.max.x, self.min.y, self.min.z);
        let t1 = Triangle::new(self.min, v1, v2, self.material);
        let t2 = Triangle::new(self.max, v1, v2, self.material);
        vec![t1, t2]
    }
}
