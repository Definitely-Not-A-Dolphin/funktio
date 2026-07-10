use image::{DynamicImage, GenericImage, GenericImageView, ImageReader, RgbaImage};
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

    fn to_math(self, size: u32) -> Complex32 {
        let size = size as f32;
        Complex32::new(
            2. / (size - 1.) * self.x as f32 - 1.,
            1. - 2. / (size - 1.) * self.y as f32,
        )
    }

    fn is_out_of_bounds(self, size: u32) -> bool {
        size <= self.x || size <= self.y
    }
}

trait ToImage {
    fn to_image(self, size: u32) -> Option<Coordinate>;
}

impl ToImage for Complex32 {
    fn to_image(self, size: u32) -> Option<Coordinate> {
        let size = size as f32 - 1.;

        if -1. <= self.re && self.re <= 1. && -1. <= self.im && self.im <= 1. {
            Some(Coordinate::new(
                (size * (self.re + 1.) / 2.).round() as u32,
                (size * (1. - self.im) / 2.).round() as u32,
            ))
        } else {
            None
        }
    }
}

fn inverse_transformation(z: Complex32) -> Complex32 {
    z.atan()
}

fn main() {
    let img_path = "static/myimage.png";
    let img = match ImageReader::open(img_path) {
        Ok(img_reader) => match img_reader.decode() {
            Ok(dynamic_image) => dynamic_image,
            Err(e) => panic!("{}", e),
        },
        Err(e) => panic!("{}", e),
    };
    let (size_x, size_y) = img.dimensions();
    let square_size = if size_x < size_y { size_y } else { size_x };
    let mut square_img: DynamicImage = RgbaImage::new(square_size, square_size).into();
    let extra_height = ((size_y as f32 - size_x as f32).abs() / 2.) as u32;

    if size_x < size_y {
        for (x, y, color) in img.pixels() {
            // We need to place pixels to the left and right
            (&mut square_img).put_pixel(x + extra_height, y, color);
        }
    } else {
        for (x, y, color) in img.pixels() {
            // We need to place pixels on the top and bottom
            (&mut square_img).put_pixel(x, y + extra_height, color);
        }
    };

    let mut transformed_img: DynamicImage = RgbaImage::new(square_size, square_size).into();

    for (x, y, _) in transformed_img.clone().pixels() {
        let output_coordinate = Coordinate::new(x, y);
        let output_number = output_coordinate.to_math(square_size);
        let input_number = inverse_transformation(output_number);
        let input_coordinate = match input_number.to_image(square_size) {
            Some(input_coordinate) => input_coordinate,
            None => continue,
        };
        if !input_coordinate.is_out_of_bounds(square_size) {
            let input_color = (&square_img).get_pixel(input_coordinate.x, input_coordinate.y);
            (&mut transformed_img).put_pixel(x, y, input_color);
        }
    }

    match transformed_img.save(img_path.to_owned() + ".png") {
        Ok(()) => {}
        Err(e) => panic!("{}", e),
    };
}
