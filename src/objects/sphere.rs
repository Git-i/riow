use crate::{Vec3, Ray};

use super::{HitInfo, Hittable};
pub struct Sphere {
    position: Vec3,
    radius: f64
}
impl Sphere {
    pub fn new(position: Vec3, radius: f64) -> Self {
        Sphere{
            position, radius
        }
    }
}
impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, tmin: f64, tmax: f64) -> Option<super::HitInfo> {
        let a = ray.dir.sq_len();
        let pos_minus_origin = &self.position - &ray.origin;
        let h = Vec3::dot(&ray.dir, &pos_minus_origin);
        let c = pos_minus_origin.sq_len() - self.radius * self.radius;
        let desc = h * h - a * c;
        
        let root_desc = desc.sqrt();
        let mut t = (h - root_desc) / a;
        if t <= tmin || t >= tmax {
            t = (h + desc) / a;
            if t <= tmin || t >= tmax {
                return None;
            }
        }
        let position = ray.at(t);
        let mut normal = &position - &self.position;
        let front_face = Vec3::dot(&ray.dir, &normal) < 0.0;
        if !front_face { normal.negate(); }
        if desc >= 0.0 {
            Some(HitInfo {
                t,
                normal,
                front_face,
                position
            })
        } else {
            None
        }
    }
}