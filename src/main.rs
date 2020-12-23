use num_iter::range_step;
use raytracer_lib::output::png::write_pixels_to_file;

fn main() {
    let nx = 200;
    let ny = 100;
    let mut pixels: Vec<u8> = Vec::new();

    for j in range_step(ny as i32, 0, -1) {
        for i in range_step(0, nx as i32, 1) {
            let r = i as f64 / nx as f64;
            let g = j as f64 / ny as f64;
            let b = 0.2f64;
            let ir = (255.99 * r).floor() as u8;
            let ig = (255.99 * g).floor() as u8;
            let ib = (255.99 * b).floor() as u8;

            pixels.push(ir);
            pixels.push(ig);
            pixels.push(ib);
        }
    }

    write_pixels_to_file("test.png", nx, ny, &pixels);
}
