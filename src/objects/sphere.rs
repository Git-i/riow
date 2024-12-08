use crate::{interval::Interval, materials::Material, Ray, Vec3};

use super::{HitInfo, Hittable};
pub struct Sphere {
    position: Vec3,
    radius: f64,
    material: Box<dyn Material>
}
impl Sphere {
    pub fn new(position: Vec3, radius: f64, material: Box<dyn Material>) -> Self {
        Sphere{
            position, radius, material
        }
    }
}
impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, tbounds: Interval) -> Option<super::HitInfo> {
        let a = ray.dir.sq_len();
        let pos_minus_origin = &self.position - &ray.origin;
        let h = Vec3::dot(&ray.dir, &pos_minus_origin);
        let c = pos_minus_origin.sq_len() - self.radius * self.radius;
        let desc = h * h - a * c;
        if desc < 0.0 { return None; }
        let root_desc = desc.sqrt();
        let mut t = (h - root_desc) / a;
        if !tbounds.surrounds(t) {
            t = (h + root_desc) / a;
            if !tbounds.surrounds(t) {
                return None;
            }
        }
        let position = ray.at(t);
        let mut normal = (&position - &self.position) * (1.0/self.radius);
        let front_face = Vec3::dot(&ray.dir, &normal) < 0.0;
        if !front_face { normal.negate(); }
        Some(HitInfo {
            t,
            normal,
            front_face,
            position,
            material: self.material.as_ref()
        })
        
    }
}