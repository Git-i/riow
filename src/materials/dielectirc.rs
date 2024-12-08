use crate::{objects::HitInfo, ray::Ray, vec3::Vec3};

use super::{Material, ScatterInfo};

pub struct Dielectirc {
    pub refraction_index: f64
}

impl Dielectirc {
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }
}

impl Material for Dielectirc {
    fn scatter(&self, ray: &Ray, hit: &HitInfo) -> Option<ScatterInfo> {
        let idx = if !hit.front_face { self.refraction_index } else { 1.0 / self.refraction_index };
        let unit_dir = ray.dir.normalized();
        let cos_theta = Vec3::dot(&unit_dir.negated(), &hit.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        Some(ScatterInfo {
            ray: Ray::new(hit.position.clone(), if idx * sin_theta > 1.0 {
                    unit_dir.reflect(&hit.normal)
                } else {
                    unit_dir.refract(&hit.normal, idx)
                }),
            atten: (1.0, 1.0, 1.0).into(),
        })
    }
}