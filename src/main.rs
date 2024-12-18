use std::{fs::File, io::{BufWriter, Write}};

mod vec3;
mod ray;
mod objects;
mod camera;
mod interval;
mod materials;

use materials::{dielectirc::Dielectirc, lambertian::Lambertian, metal::Metal};
use objects::{sphere::Sphere, ObjectList};
use vec3::Vec3;
use ray::Ray;
use camera::Camera;

fn main() {
    let cam = Camera::new((-2.0, 2.0, -1.0).into(), (0.0, 0.0, 1.0).into(), Vec3::UP, 500, 90.0, 1.6)
        .sample_count(10)
        .recursion_depth(40);
    
    let mut file = BufWriter::new(File::create("output.ppm").unwrap());
    
    let mut world = ObjectList::new();
    world.add(Box::new(Sphere::new(
        (0.0, 0.0, 1.2).into(), 
        0.5, 
        Box::new(Metal::new((0.57, 0.44, 0.23).into(), 0.2))
    )));
    world.add(Box::new(Sphere::new(
        (1.0, 0.0, 1.0).into(), 
        0.5, 
        Box::new(Lambertian::new((0.27, 0.44, 0.73).into()))
    )));
    world.add(Box::new(Sphere::new(
        (-1.0, 0.0, 1.0).into(), 
        0.5, 
        Box::new(Dielectirc::new(1.0/1.33))
    )));
    world.add(Box::new(Sphere::new(
        (0.0,-100.5,1.0).into(), 
        100.0,
        Box::new(Lambertian::new((0.57, 0.23, 0.62).into()))
    )));

    cam.render_world(&world, &mut file);
}
