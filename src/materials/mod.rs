use crate::{objects::HitInfo, ray::Ray, vec3::Vec3};
pub mod lambertian;
pub struct ScatterInfo{
    ray: Ray,
    atten: Vec3
}
pub trait Material {
    fn scatter(&self, ray: &Ray, hit: &HitInfo) -> Option<ScatterInfo>;
}