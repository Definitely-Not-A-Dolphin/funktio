use image::{DynamicImage, GenericImageView, ImageBuffer, ImageReader, Rgba, RgbaImage};
use num_complex::Complex32;

#[derive(Clone, Copy, Debug)]
struct Coordinate {
    x: u32,
    y: u32,
}

impl Coordinate {
    fn new(x: u32, y: u32) -> Self {
        Coordinate { x, y }
    }

    fn to_math(self, (size_x, size_y): (u32, u32)) -> Complex32 {
        let (size_x, size_y) = (size_x as f32, size_y as f32);

        if size_x < size_y {
            let a = 2. / size_x * self.x as f32 - 1.;
            let b = 1. - 2. / size_y * self.y as f32;

            Complex32::new(a, b)
        } else {
            let a = 2. / size_x * self.x as f32 - 1.;
            let b = 1. - 2. / size_y * self.y as f32;

            Complex32::new(b, a)
        }
    }

    fn is_out_of_bounds(self, (size_x, size_y): (u32, u32)) -> bool {
        size_x <= self.x || size_y <= self.y
    }
}

trait ToImage {
    fn to_image(self, size: (u32, u32)) -> Coordinate;
}

impl ToImage for Complex32 {
    fn to_image(self, (size_x, size_y): (u32, u32)) -> Coordinate {
        let (size_x, size_y) = (size_x as f32, size_y as f32);

        let a = (size_x / 2. * (self.re + 1.)) as u32;
        let b = (size_y / 2. * (1. - self.im)) as u32;

        if size_x < size_y {
            Coordinate::new(a, b)
        } else {
            Coordinate::new(b, a)
        }
    }
}

fn transformation_function(z: Complex32) -> Complex32 {
    z * Complex32::i()
}

fn square_image(path: &str) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let img = match ImageReader::open(path) {
        Ok(img_reader) => match img_reader.decode() {
            Ok(dynamic_image) => dynamic_image,
            Err(e) => panic!("{}", e),
        },
        Err(e) => panic!("{}", e),
    };

    let (size_x, size_y) = img.dimensions();
    let new_size = if size_x < size_y { size_y } else { size_x };

    let mut new_img = RgbaImage::new(new_size, new_size);

    // Odd by odd OR Even by even
    if size_x % 2 == size_y % 2 {
        if size_x < size_y {
            // We need to place pixels to the left and right
            let extra_height = ((size_y - size_x) as f32 / 2_f32) as u32;
            for (x, y, color) in img.pixels() {
                (&mut new_img).put_pixel(x + extra_height, y, color);
            }
        } else {
            // We need to place pixels on the top and bottom
            let extra_height = ((size_x - size_y) as f32 / 2_f32) as u32;
            for (x, y, color) in img.pixels() {
                (&mut new_img).put_pixel(x, y + extra_height, color);
            }
        };
    }

    new_img
}

fn main() {
    let img_path = "static/Me2.jpg";
    let thing = square_image(img_path);
    match thing.save(img_path.to_owned() + ".png") {
        Ok(()) => {}
        Err(e) => panic!("{}", e),
    };

    // let size = (2000, 3200);
    // let pixel = Coordinate::new(0, 0);
    // let number = pixel.to_math(size);
    // let pixel2 = number.to_image(size);
    // dbg!(pixel, number, pixel2);

    // let img_path = "static/Me2.jpg";
    // let img = match ImageReader::open(img_path) {
    //     Ok(img_reader) => match img_reader.decode() {
    //         Ok(dynamic_image) => dynamic_image,
    //         Err(e) => panic!("{}", e),
    //     },
    //     Err(e) => panic!("{}", e),
    // };

    // let img_dimensions = img.dimensions();
    // let mut new_img = RgbaImage::new(img_dimensions.0, img_dimensions.1);
    // let mut unplaced_pixels_count = 0_u32;

    // for (x, y, color) in img.pixels() {
    //     let input_number = Coordinate::new(x, y).to_math(img_dimensions);
    //     let output_number = transformation_function(input_number);
    //     let output_coordinate = output_number.to_image(img_dimensions);
    //     if output_coordinate.is_out_of_bounds(img_dimensions) {
    //         unplaced_pixels_count += 1
    //     } else {
    //         (&mut new_img).put_pixel(output_coordinate.x, output_coordinate.y, color);
    //     };
    // }

    // let _ = new_img.save(img_path.to_owned() + ".jpg");
    // print!("{} pixels were unplaced", unplaced_pixels_count);
}
