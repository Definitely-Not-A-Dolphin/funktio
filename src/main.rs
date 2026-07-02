use rug::Complex;

fn main() {
    let z0 = Complex::with_val(24, (0, 1));

    let square = Complex::square(z0.clone());

    println!("Hello, world!, {}", square);
}
