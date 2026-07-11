use num_complex::Complex32;

pub struct Args {
    /// Path to the image that needs to be transformed
    pub path: String,
    /// Format of the image the output image
    pub format: String,
    /// Verbose or not
    pub verbose: bool,
    /// Show help or not
    pub help: bool,
}

#[derive(Clone, Copy, Debug)]
pub struct Coordinate {
    pub x: u32,
    pub y: u32,
}

impl Coordinate {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }

    /// Convert from image coordinate to complex number
    pub fn to_math_space(self, size: u32) -> Complex32 {
        let size = size as f32 - 1.;
        Complex32::new(
            2. / size * self.x as f32 - 1.,
            1. - 2. / size * self.y as f32,
        )
    }

    /// Checks whether a coordinate is out of bounds a given square image size
    pub fn is_out_of_bounds(self, size: u32) -> bool {
        size <= self.x || size <= self.y
    }
}

pub trait ToImage {
    fn to_image_space(self, size: u32) -> Option<Coordinate>;
}

impl ToImage for Complex32 {
    /// Convert from complex to image coordinate
    fn to_image_space(self, size: u32) -> Option<Coordinate> {
        // Checks whether the number lies in the unit square.
        if -1. <= self.re && self.re <= 1. && -1. <= self.im && self.im <= 1. {
            let size = size as f32 - 1.;
            Some(Coordinate::new(
                (size * (self.re + 1.) / 2.).round() as u32,
                (size * (1. - self.im) / 2.).round() as u32,
            ))
        } else {
            None
        }
    }
}
