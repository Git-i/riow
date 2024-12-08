pub mod sphere;


use crate::{interval::Interval, materials::Material, Ray, Vec3};

pub struct HitInfo<'a> {
    pub t: f64,
    pub normal: Vec3,
    pub position: Vec3,
    pub front_face: bool,
    pub material: &'a dyn Material
}
pub trait Hittable {
    fn hit(&self, ray: &Ray, tbounds: Interval) -> Option<HitInfo>;
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
    pub fn closest_hit(&self, ray: &Ray, mut tbounds: Interval) -> Option<HitInfo> {
        let mut return_value = None;
        for obj in &self.objects {
            if let Some(info) = obj.hit(ray, tbounds.clone()) {
                tbounds = tbounds.right_rebind(info.t);
                return_value = Some(info);
            }
        }
        return_value
    }
}
