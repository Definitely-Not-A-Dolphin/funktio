use num_complex::Complex32;

pub struct Args {
    pub path: String,
    pub verbose: bool,
    pub help: bool,
}

#[derive(Clone, Copy, Debug)]
pub struct Coordinate {
    pub x: u32,
    pub y: u32,
}

impl Coordinate {
    pub fn new(x: u32, y: u32) -> Self {
        Coordinate { x, y }
    }

    pub fn to_math(self, size: u32) -> Complex32 {
        let size = size as f32 - 1.;
        Complex32::new(
            2. / size * self.x as f32 - 1.,
            1. - 2. / size * self.y as f32,
        )
    }

    pub fn is_out_of_bounds(self, size: u32) -> bool {
        size <= self.x || size <= self.y
    }
}

pub trait ToImage {
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
