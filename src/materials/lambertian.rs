use crate::{objects::HitInfo, ray::Ray, vec3::Vec3};

use super::{Material, ScatterInfo};

pub struct Lambertian {
    pub albedo: Vec3
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Self {
        Self {albedo}
    }
}
impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, hit: &HitInfo) -> Option<ScatterInfo> {
        let reflection_dir = &hit.normal + &Vec3::random_unit_vec();
        Some(ScatterInfo{ 
            ray: Ray::new(ray.origin.clone(), 
            if reflection_dir.is_near_zero() {
                hit.normal.clone()
            } else {
               reflection_dir 
            }),
            atten: self.albedo.clone() 
        })
    }
}