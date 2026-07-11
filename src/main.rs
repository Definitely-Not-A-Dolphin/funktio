use crate::structs::*;
use image::{DynamicImage, GenericImage, GenericImageView, ImageReader, RgbaImage};
use num_complex::Complex32;
use sap::{Argument, Parser};
use std::path::Path;

mod structs;

fn inverse_transformation(z: Complex32) -> Complex32 {
    z * Complex32::new(2., 0.)
}

fn print_no_valid_format(supported_formats: [&str; 15]) {
    print!(
        "Please select a file with a supported file extension\nSupported file extensions include"
    );
    for supported_format in supported_formats {
        print!(" {}", supported_format);
    }
    print!(".");
}

fn main() {
    let supported_formats = [
        "avif", "bmp", "dds", "exr", "ff", "hdr", "ico", "jpeg", "jpg", "png", "pnm", "qoi", "tga",
        "tiff", "webp",
    ];
    let mut parser = Parser::from_env().unwrap();
    let mut args = Args {
        path: String::from(""),
        format: String::from(""),
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
            Argument::Long("format") => {
                if let Some(format) = parser.value().unwrap() {
                    if supported_formats.contains(&format.as_str()) {
                        args.format = format;
                    } else {
                        print_no_valid_format(supported_formats);
                        return;
                    }
                }
            }
            Argument::Short('v') => args.verbose = true,
            Argument::Short('h') => args.help = true,
            _ => {}
        }
    }

    if args.path.is_empty() {
        print!("No path was provided, set one with the `--path` flag.");
        return;
    }

    if args.format.is_empty() {
        args.format = match Path::new(args.path.as_str()).extension() {
            Some(file_extension) => {
                if supported_formats.contains(&file_extension.to_str().unwrap()) {
                    String::from(file_extension.to_str().unwrap())
                } else {
                    print_no_valid_format(supported_formats);
                    return;
                }
            }
            None => {
                print!("Please select a file with a file extension.");
                return;
            }
        };
    }

    if args.help {
        // Print help
        return;
    }

    let args_path = Path::new(args.path.as_str());

    if !args_path.is_file() {
        print!(
            "An invalid path was provided, make sure the input is an existing image of a supported format.\n"
        );
        return;
    }

    let new_file_path = {
        let path_without_extension = &args.path[0_usize..(args.path.len() - args.format.len())];
        path_without_extension.to_owned() + "funktio." + args.format.as_str()
    };

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

    for x in 0..square_size {
        for y in 0..square_size {
            let output_number = Coordinate::new(x, y).to_math_space(square_size);
            let input_number = inverse_transformation(output_number);
            let input_coordinate = match input_number.to_image_space(square_size) {
                Some(input_coordinate) => input_coordinate,
                None => continue,
            };
            if !input_coordinate.is_out_of_bounds(square_size) {
                let input_color = (&square_img).get_pixel(input_coordinate.x, input_coordinate.y);
                (&mut transformed_img).put_pixel(x, y, input_color);
                placed_pixels += 1;
            }
        }
    }

    if args.verbose {
        print!(
            "Placed {} out of {} pixels, {} pixels were discarded\n",
            placed_pixels,
            square_size.pow(2),
            square_size.pow(2) - placed_pixels
        );
    }

    match transformed_img.save(new_file_path) {
        Ok(()) => {}
        Err(e) => panic!("{}", e),
    };
}
