use image::{DynamicImage, GenericImage, GenericImageView, ImageReader, RgbaImage};
use num_complex::Complex32;
use sap::{Argument, Parser};
use std::path::Path;

struct Args {
    path: String,
    verbose: bool,
    help: bool,
}

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
        let size = size as f32 - 1.;
        Complex32::new(
            2. / size * self.x as f32 - 1.,
            1. - 2. / size * self.y as f32,
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
    z.powc(Complex32::i())
}

fn main() {
    let supported_formats = [
        "avif", "bmp", "dds", "exr", "ff", "hdr", "ico", "jpeg", "jpg", "png", "pnm", "qoi", "tga",
        "tiff", "webp",
    ];
    let mut parser = Parser::from_env().unwrap();
    let mut args = Args {
        path: String::from(""),
        verbose: false,
        help: false,
    };

    while let Some(arg) = parser.forward().unwrap() {
        match arg {
            Argument::Long("path") => {
                if let Some(path) = parser.value().unwrap() {
                    args.path = path;
                }
            }
            Argument::Short('v') => args.verbose = true,
            Argument::Short('h') => args.help = true,
            _ => {}
        }
    }

    if args.path.is_empty() {
        print!("No path was provided, set one with the `--path` flag");
        return;
    }

    let file_extension = match Path::new(args.path.as_str()).extension() {
        Some(file_extension) => {
            let supported_formats = [
                "avif", "bmp", "dds", "exr", "ff", "hdr", "ico", "jpeg", "jpg", "png", "pnm",
                "qoi", "tga", "tiff", "webp",
            ];
            if !supported_formats.contains(&file_extension.to_str().unwrap()) {
                print!(
                    "Please select a file with a supported file extension\nSupported file extensions include "
                );
                for supported_format in supported_formats {
                    print!("{} ", supported_format);
                }
                print!(".");
                return;
            }
        }
        None => {
            print!("Please select a file with a file extension");
            return;
        }
    };

    if args.help {
        // Print help
        return;
    }

    if !Path::new(args.path.as_str()).is_file() {
        print!(
            "An invalid path was provided, make sure the input is an existing image of a supported format.\n"
        );
        return;
    }

    let img = match ImageReader::open(&args.path) {
        Ok(img_reader) => match img_reader.decode() {
            Ok(dynamic_image) => dynamic_image,
            Err(e) => panic!("{}", e),
        },
        Err(e) => panic!("{}", e),
    };
    let (size_x, size_y) = img.dimensions();
    let square_size = if size_x < size_y { size_y } else { size_x };
    print!(
        "Found dimensions: {}x{}\nOutput image dimensions: {}x{}\n",
        size_x, size_y, square_size, square_size
    );
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
    let mut placed_pixels = 0_u32;
    let mut unplaced_pixels = 0_u32;

    for (x, y, _) in transformed_img.clone().pixels() {
        let output_number = Coordinate::new(x, y).to_math(square_size);
        let input_number = inverse_transformation(output_number);
        let input_coordinate = match input_number.to_image(square_size) {
            Some(input_coordinate) => input_coordinate,
            None => continue,
        };
        if !input_coordinate.is_out_of_bounds(square_size) {
            let input_color = (&square_img).get_pixel(input_coordinate.x, input_coordinate.y);
            (&mut transformed_img).put_pixel(x, y, input_color);
            placed_pixels += 1;
        } else {
            unplaced_pixels += 1;
        }
    }

    if args.verbose {
        print!(
            "Placed {} pixels, {} pixels were unplaced\n",
            placed_pixels, unplaced_pixels
        );
    }

    match transformed_img.save(args.path) {
        Ok(()) => {}
        Err(e) => panic!("{}", e),
    };
}
