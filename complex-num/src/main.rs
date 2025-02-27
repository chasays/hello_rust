use num::complex::Complex;

fn main() {
    let a = Complex { re: 2.1, im: -1.2 };
    let b  = Complex::new(11.2,22.3);
    let result = a+b;
    println!("{}", result);
    println!("Hello, world!");
}
