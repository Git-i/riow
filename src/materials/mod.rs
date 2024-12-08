use crate::{objects::HitInfo, ray::Ray, vec3::Vec3};
pub mod lambertian;
pub mod metal;
pub mod dielectirc;
pub struct ScatterInfo{
    pub ray: Ray,
    pub atten: Vec3
}
pub trait Material {
    fn scatter(&self, ray: &Ray, hit: &HitInfo) -> Option<ScatterInfo>;
}