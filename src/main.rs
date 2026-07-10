use image::{DynamicImage, GenericImageView, ImageReader, RgbaImage};
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
    fn to_image(self, size: u32) -> Coordinate;
}

impl ToImage for Complex32 {
    fn to_image(self, size: u32) -> Coordinate {
        let size = size as f32;
        Coordinate::new(
            ((size - 1.) * (self.re + 1.) / 2.).round() as u32,
            ((size - 1.) * (1. - self.im) / 2.).round() as u32,
        )
    }
}

fn inverse_transformation(z: Complex32) -> Complex32 {
    z.exp()
}

fn main() {
    // let coordinate_input = Coordinate::new(0, 200);
    // let number_input = coordinate_input.to_math(256);
    // let number_output = transformation_function(number_input);
    // let coordinate_output = number_output.to_image(256);
    // dbg!(
    //     coordinate_input,
    //     number_input,
    //     number_output,
    //     coordinate_output
    // );

    let img_path = "static/big_malko.jpg";
    let img = match ImageReader::open(img_path) {
        Ok(img_reader) => match img_reader.decode() {
            Ok(dynamic_image) => dynamic_image,
            Err(e) => panic!("{}", e),
        },
        Err(e) => panic!("{}", e),
    };
    let (size_x, size_y) = img.dimensions();
    let square_size = if size_x < size_y { size_y } else { size_x };
    let mut square_img = RgbaImage::new(square_size, square_size);

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

    let square_img: DynamicImage = square_img.into();
    let mut transformed_img = RgbaImage::new(square_size, square_size);

    for (x, y, color) in square_img.pixels() {
        let input_coordinate = Coordinate::new(x, y);
        let input_number = input_coordinate.to_math(square_size);
        let output_number = transformation_function(input_number);
        //dbg!(input_number, output_number);
        let ouput_coordinate = output_number.to_image(square_size);
        if !ouput_coordinate.is_out_of_bounds(square_size) {
            (&mut transformed_img).put_pixel(ouput_coordinate.x, ouput_coordinate.y, color);
        }
    }

    match transformed_img.save(img_path.to_owned() + ".png") {
        Ok(()) => {}
        Err(e) => panic!("{}", e),
    };
}
