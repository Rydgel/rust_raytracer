use num_iter::range_step;
use raytracer_lib::{output::png::write_pixels_to_file, structures::vec3::Vec3};

fn main() {
    let nx = 200;
    let ny = 100;
    let mut pixels: Vec<u8> = Vec::new();

    for j in range_step(ny as i32, 0, -1) {
        for i in range_step(0, nx as i32, 1) {
            let r = i as f32 / nx as f32;
            let g = j as f32 / ny as f32;
            let b = 0.0f32;
            let col = Vec3::new(r, g, b);
            let ir = (255.99 * col[0]).floor() as u8;
            let ig = (255.99 * col[1]).floor() as u8;
            let ib = (255.99 * col[2]).floor() as u8;

            pixels.push(ir);
            pixels.push(ig);
            pixels.push(ib);
        }
    }

    write_pixels_to_file("test.png", nx, ny, &pixels);
}
