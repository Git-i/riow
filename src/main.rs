use std::{fs::File, io::{BufWriter, Write}};

mod vec3;
use vec3::Vec3;
fn main() {
    // Image

    let image_width = 256;
    let image_height = 256;
    
    let mut file = BufWriter::new(File::create("output.ppm").unwrap());
    file.write_all(format!("P3\n{} {}\n255\n", image_width, image_height).as_bytes()).unwrap();
    
    for j in 0..image_height {
        eprintln!("Lines remaining {}", image_height - j);
        for i in 0..image_width {
            let color = Vec3{
                x: (i as f64 / image_width as f64),
                y: (j as f64 / image_height as f64),
                z: 0.0
            };
            color.write_as_color(&mut file);
        }
    }
}
