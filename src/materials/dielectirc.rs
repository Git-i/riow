use rand::random;

use crate::{objects::HitInfo, ray::Ray, vec3::Vec3};

use super::{Material, ScatterInfo};

pub struct Dielectirc {
    pub refraction_index: f64
}

impl Dielectirc {
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }
    fn reflectance(cos: f64, ri: f64) -> f64 {
        let root_r0 = (1.0 - ri) / (1.0 + ri);
        let r0 = root_r0 * root_r0;
        r0 + (1.0 - r0) * (1.0-cos).powi(5)
    }
}

impl Material for Dielectirc {
    fn scatter(&self, ray: &Ray, hit: &HitInfo) -> Option<ScatterInfo> {
        let idx = if !hit.front_face { self.refraction_index } else { 1.0 / self.refraction_index };
        let unit_dir = ray.dir.normalized();
        let cos_theta = Vec3::dot(&unit_dir.negated(), &hit.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        Some(ScatterInfo {
            ray: Ray::new(hit.position.clone(), 
            if idx * sin_theta > 1.0 || Self::reflectance(cos_theta, idx) > random::<f64>() {
                    unit_dir.reflect(&hit.normal)
                } else {
                    unit_dir.refract(&hit.normal, idx)
                }),
            atten: (1.0, 1.0, 1.0).into(),
        })
    }
}