pub mod sphere;


use crate::{Ray, Vec3};

pub struct HitInfo {
    pub t: f64,
    pub normal: Vec3,
    pub position: Vec3,
    pub front_face: bool
}
pub trait Hittable {
    fn hit(&self, ray:&Ray, tmin: f64, tmax: f64) -> Option<HitInfo>;
}

pub struct ObjectList {
    objects: Vec<Box<dyn Hittable>>
}

impl ObjectList {
    pub fn new() -> Self {
        Self { objects: Vec::new() }
    }
    pub fn add(&mut self, obj: Box<dyn Hittable>) {
        self.objects.push(obj);
    }
    pub fn closest_hit(&self, ray: &Ray, tmin: f64, tmax: f64) -> Option<HitInfo> {
        let mut return_value = None;
        let mut closest = tmax;
        for obj in &self.objects {
            if let Some(info) = obj.hit(ray, tmin, closest) {
                closest = info.t;
                return_value = Some(info);
            }
        }
        return_value
    }
}
