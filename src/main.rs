use std::{fs::File, io::{BufWriter, Write}};

mod vec3;
mod ray;
use vec3::Vec3;
use ray::Ray;

fn ray_color(ray: Ray) -> Vec3 {
    let unit_dir = ray.origin.clone().normalized();
    Vec3::from((0.1, 0.84, 0.22)) * (1.0 - unit_dir.y) + Vec3::from((0.88, 0.9, 0.91)) * unit_dir.y
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
    let viewport_v: Vec3 = (0.0, viewport_height, 0.0).into();

    let viewport_upper_left = camera_position.clone() 
        + (0.0, 0.0, focal_length).into()
        - viewport_u.clone() * 0.5
        + viewport_v.clone() * 0.5;
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
