use crate::{objects::HitInfo, ray::Ray, vec3::Vec3};

use super::{Material, ScatterInfo};

pub struct Metal {
    color: Vec3
}

impl Metal {
    pub fn new(color: Vec3) -> Self {
        Self {color}
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &HitInfo) -> Option<ScatterInfo> {
        Some(ScatterInfo{
            atten: self.color.clone(),
            ray: Ray::new(hit.position.clone(), ray.dir.reflect(&hit.normal))
        })
    }
}