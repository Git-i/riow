fn main() {
    // Image

    let image_width = 256;
    let image_height = 256;
    println!("P3\n{} {}\n255\n", image_width, image_height);

    for j in 0..image_height {
        for i in 0..image_width {
            let r = (i as f64 * 256.0 / image_width as f64) as i32;
            let g = (j as f64 * 256.0 / image_height as f64) as i32;
            let b = 0;

            println!("{} {} {}", r, g, b);
        }
    }
}
