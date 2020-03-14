use std::f64::consts::TAU;

fn main() {
    // Variable and Mutability
    let a = 5;
    let mut b = 6;
    // I can b
    b *= 6;
    println!("The value of b is {} and a {}", b, a);
    // I can't change a but I can shadow it
    let a = 7;
    println!("The value of a is {}", a);
    // I can create constant but I *must* annotate them
    const QUADRUPLE_PI: f64 = 2.0 * TAU;
    // Btw happy PI day ! (look at the commit date)
}
