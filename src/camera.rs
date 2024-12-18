use std::io::{Write, BufWriter};

use rand::random;

use crate::{interval::Interval, objects::ObjectList, ray::Ray, vec3::Vec3};

pub struct Camera {
    pixel_du: Vec3,
    pixel_dv: Vec3,
    image_width: i32,
    image_height: i32,
    first_pixel_world_loc: Vec3,
    position: Vec3,
    samples_per_pixel: i32,
    max_depth: u32
}
impl Camera {
    pub fn new(position: Vec3, focus_point: Vec3, up: Vec3, image_width: i32, fov: f64, aspect: f64) -> Self {
        let image_height = i32::max((image_width as f64 / aspect) as i32, 1);
        let focal_length = (&focus_point - &position).len();
        let viewport_height = 2.0 * focal_length * (0.5 * fov.to_radians()).tan();
        let viewport_width = viewport_height * (image_width as f64/image_height as f64);

        let forward = (&focus_point - &position).normalized();
        let right = up.cross(&forward).normalized();
        let actual_up = right.cross(&forward);

        let viewport_u = right * viewport_width;
        let viewport_v = actual_up * viewport_height;

        let pixel_du = viewport_u.clone() * (1.0 / image_width as f64);
        let pixel_dv = viewport_v.clone() * (1.0/ image_height as f64);
        let viewport_upper_left = position.clone() 
            + forward * focal_length
            - viewport_u.clone() * 0.5
            - viewport_v.clone() * 0.5;
        let first_pixel_world_loc = viewport_upper_left.clone() + 0.5 * &(&pixel_du + &pixel_dv);
        Self { 
            pixel_du, 
            pixel_dv,
            image_width,
            image_height, 
            first_pixel_world_loc,
            position,
            samples_per_pixel: 1 ,
            max_depth: 10
        }
    }
    pub fn sample_count(self, samples_per_pixel: i32) -> Self {
        Self { samples_per_pixel, ..self }
    }
    pub fn recursion_depth(self, max_depth: u32) -> Self {
        Self { max_depth, ..self }
    }
    fn ray_color(ray: Ray, world: &ObjectList, depth: u32) -> Vec3 {
        if depth == 0 {
            return Vec3::zero();
        }
        let unit_dir = ray.dir.clone().normalized();
        //from -1 - 1 to 0 - 1
        let a = unit_dir.y * 0.5 + 0.5;
        if let Some(rec) = world.closest_hit(&ray, Interval::UNIVERSE.left_rebind(0.0001)) {
            if let Some(scatter) = rec.material.scatter(&ray, &rec) {
                Self::ray_color(scatter.ray, world, depth - 1) * scatter.atten
            } else {
                Vec3::zero()
            }
        } else {
            Vec3::from((0.5, 0.62, 0.84)) * a + Vec3::from((0.88, 0.9, 0.91)) * (1.0 - a)
        }
    }
    pub fn render_world(&self, world: &ObjectList, file: &mut BufWriter<impl Write>) {
        let inv_num_samples = 1.0 / self.samples_per_pixel as f64;
        file.write_all(format!("P3\n{} {}\n255\n", self.image_width, self.image_height).as_bytes()).unwrap();
        for j in 0..self.image_height {
            eprintln!("Lines remaining {}", self.image_height - j);
            for i in 0..self.image_width {
                let mut color = Vec3::zero();
                for _ in 0..self.samples_per_pixel {
                    color = color + Self::ray_color(self.get_random_ray(i, j), &world, self.max_depth)
                }
                color = color * inv_num_samples;
                color.write_as_color(file);
            }
        }
    }
    pub fn get_random_ray(&self, i: i32, j: i32) -> Ray {
        //random is in [0, 1) we -1 so its [-0.5, 0.5) so its in a square around the pixel
        let pixel_position = &self.first_pixel_world_loc 
            + &((i as f64 + random::<f64>() - 0.5) * &self.pixel_du) 
            + ((j as f64 + random::<f64>() - 0.5) * &self.pixel_dv);
        Ray::new(self.position.clone(), &pixel_position - &self.position)
    }
    pub fn image_width(&self) -> i32 {
        self.image_width
    }
    pub fn image_height(&self) -> i32 {
        self.image_height
    }
}