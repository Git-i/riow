mod sphere;
use crate::{Ray, Vec3};

pub struct HitInfo {
    t: f64,
    normal: Vec3,
    position: Vec3,
    front_face: bool
}
pub trait Hittable {
    fn hit(&self, ray:&Ray, tmin: f64, tmax: f64) -> Option<HitInfo>;
}
