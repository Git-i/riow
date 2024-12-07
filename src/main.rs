use std::{fs::File, io::{BufWriter, Write}};

mod vec3;
mod ray;
use vec3::Vec3;
use ray::Ray;

fn hit_sphere(ray: &Ray, sphere_rad: f64, sphere_pos: &Vec3) -> f64 {
    let a = Vec3::dot(&ray.dir, &ray.dir);
    let pos_minus_origin = sphere_pos - &ray.origin;
    let b = -2.0 * Vec3::dot(&ray.dir, &pos_minus_origin);
    let c = Vec3::dot(&pos_minus_origin, &pos_minus_origin) - sphere_rad * sphere_rad;
    let desc = b * b - 4.0 * a * c;
    if desc >= 0.0 {
        (-b - desc.sqrt()) / 2.0 * a
    } else {
        -1.0
    }
}


fn ray_color(ray: Ray) -> Vec3 {
    let unit_dir = ray.dir.clone().normalized();
    //from -1 - 1 to 0 - 1
    let a = unit_dir.y * 0.5 + 0.5;
    let sphere_pos = Vec3::from((0.0, 0.0, 2.0));
    let sphere_rad = 1.0;
    let hit_pos = hit_sphere(&ray, sphere_rad, &sphere_pos);
    if hit_pos > 0.0 { //there was an intersection and it was infront of the camera
        let normal = (ray.at(hit_pos) - sphere_pos) * (1.0/hit_pos);
        0.5 * &normal + Vec3::from((0.5, 0.5, 0.5))
    } else {
        Vec3::from((0.5, 0.62, 0.84)) * a + Vec3::from((0.88, 0.9, 0.91)) * (1.0 - a)
    }
}
fn main() {
    // Image
    let aspect_ratio = 1.6; // 16:10
    let image_width = 500;
    let image_height = i32::max((image_width as f64 / aspect_ratio) as i32, 1);

    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64/image_height as f64);
    
    let mut file = BufWriter::new(File::create("output.ppm").unwrap());
    file.write_all(format!("P3\n{} {}\n255\n", image_width, image_height).as_bytes()).unwrap();
    
    let focal_length = 1.0;
    let camera_position = Vec3::zero();
    
    let viewport_u: Vec3 = (viewport_width, 0.0, 0.0).into();
    let viewport_v: Vec3 = (0.0, -viewport_height, 0.0).into();

    let viewport_upper_left = camera_position.clone() 
        + (0.0, 0.0, focal_length).into()
        - viewport_u.clone() * 0.5
        - viewport_v.clone() * 0.5;
    //we trace through the centre pf very pixel
    //pixel du and pixel dv is the distance between pixels on the viewport plane
    let pixel_du = viewport_u.clone() * (1.0 / image_width as f64);
    let pixel_dv = viewport_v.clone() * (1.0/ image_height as f64);

    let first_pixel_world_loc = viewport_upper_left.clone() + 0.5 * &(&pixel_du + &pixel_dv);

    for j in 0..image_height {
        eprintln!("Lines remaining {}", image_height - j);
        for i in 0..image_width {
            let pixel_position = &first_pixel_world_loc + &(i as f64 * &pixel_du) + (j as f64 * &pixel_dv);
            let ray = Ray::new(camera_position.clone(), &pixel_position - &camera_position);
            ray_color(ray).write_as_color(&mut file);
        }
    }
}
