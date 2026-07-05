use image::{Rgb, RgbImage};

fn main() {
    let name = "static/new_image.png";
    let _ = std::fs::remove_file(name);

    let mut img = RgbImage::new(255, 255);
    for x in 0..255 {
        for y in 0..255 {
            (&mut img).put_pixel(x, y, Rgb([x as u8, y as u8, 100]));
        }
    }
    let _ = img.save(name);
}
