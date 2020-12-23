use num_iter::range_step;
use raytracer_lib::{
    entities::ray::Ray, output::png::write_pixels_to_file, structures::vec3::Vec3,
};

fn color(r: &Ray) -> Vec3 {
    let unit_direction = r.direction.unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Vec3::from((1.0, 1.0, 1.0)) + t * Vec3::from((0.5, 0.7, 1.0))
}

fn main() {
    let nx = 200;
    let ny = 100;
    let mut pixels: Vec<u8> = Vec::new();
    let lower_left_corner = Vec3::from((-2.0, -1.0, -1.0));
    let horizontal = Vec3::from((4.0, 0.0, 0.0));
    let vertical = Vec3::from((0.0, 2.0, 0.0));
    let origin = Vec3::from((0.0, 0.0, 0.0));

    for j in range_step(ny as i32, 0, -1) {
        for i in range_step(0, nx as i32, 1) {
            let u = i as f32 / nx as f32;
            let v = j as f32 / ny as f32;
            let r = Ray {
                origin,
                direction: lower_left_corner + u * horizontal + v * vertical,
            };
            let col = color(&r);

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
