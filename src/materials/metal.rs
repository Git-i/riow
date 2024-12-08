use crate::{objects::HitInfo, ray::Ray, vec3::Vec3};

use super::{Material, ScatterInfo};

pub struct Metal {
    color: Vec3,
    fuzz: f64
}

impl Metal {
    pub fn new(color: Vec3, fuzz: f64) -> Self {
        Self {color, fuzz}
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &HitInfo) -> Option<ScatterInfo> {
        let direction = Vec3::random_unit_vec() * self.fuzz + ray.dir.reflect(&hit.normal).normalized();
        Some(ScatterInfo{
            atten: self.color.clone(),
            ray: Ray::new(hit.position.clone(), direction)
        })
    }
}