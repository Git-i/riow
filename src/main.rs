use std::{fs::File, io::{BufWriter, Write}};

mod vec3;
mod ray;
mod objects;
mod camera;
mod interval;

use objects::{sphere::Sphere, ObjectList};
use vec3::Vec3;
use ray::Ray;
use camera::Camera;

fn main() {
    
    let cam = Camera::new(Vec3::zero(), 1.0, 500, 2.0, 1.6);
    
    let mut file = BufWriter::new(File::create("output.ppm").unwrap());
    file.write_all(format!("P3\n{} {}\n255\n", cam.image_width(), cam.image_height()).as_bytes()).unwrap();
    
    
    let mut world = ObjectList::new();
    world.add(Box::new(Sphere::new((0.0, 0.0, 1.0).into(), 0.5)));
    world.add(Box::new(Sphere::new((0.0,-100.5,1.0).into(), 100.0)));

    cam.render_world(&world, &mut file);
}
