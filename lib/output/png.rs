use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

pub fn write_pixels_to_file(file_path: &str, width: u32, height: u32, pixels: &[u8]) {
    let path = Path::new(file_path);
    let file = File::create(path).expect("Unable to create file");
    let w = &mut BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, width, height);
    encoder.set_color(png::ColorType::RGB);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder
        .write_header()
        .expect("Unable to get a buffer for writing the png");

    writer.write_image_data(pixels).unwrap();
}
