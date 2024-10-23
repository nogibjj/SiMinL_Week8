//takes interval [a,b], number of segments n, and function f as input
// any function that matches Fn(f64) -> 64 works
pub fn trapezoidal_rule<F>(a: f64, b: f64, n: usize, f: F) -> f64
where
    F: Fn(f64) -> f64,
{
    let h = (b - a) / n as f64;
    let mut integral = 0.5 * (f(a) + f(b)); // Starting with the endpoints
    // Sum up the intermediate points
    for i in 1..n {
        let x = a + i as f64 * h;
        integral += f(x);
    }
    integral * h // Multiply by the width of each segment
}


