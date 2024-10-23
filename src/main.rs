// main.rs is the entry point of Rust program.
// Define and handle CLI arguments using clap crate

use integration::trapezoidal_rule;

fn main() {
    // Define the function to integrate (eg. f(x) = x^2)
    let f = |x: f64| x * x;

    // Define the interval and number of segments
    let a = 0.0;
    let b = 1.0;
    let n = 100;

    // Calculate the integral
    let result = trapezoidal_rule(a, b, n, f);
    
    // Print the result
    println!("The approximate integral of x^2 over [0, 1] is: {}", result);
}
