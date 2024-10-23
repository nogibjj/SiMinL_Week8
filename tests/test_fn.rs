//[cfg(test)]
use integration::trapezoidal_rule;

mod tests {
    use super::*;
    // means import everything from parent module

    #[test]
    fn test_trapezoidal_rule_with_square() {
        let result = trapezoidal_rule(0.0, 1.0, 100, |x| x * x);
        let expected = 1.0 / 3.0; // Exact result of the integral of x^2 from 0 to 1
        assert!((result - expected).abs() < 0.001); // Allow small numerical error
    }

    #[test]
    fn test_trapezoidal_rule_with_constant() {
        let result = trapezoidal_rule(0.0, 2.0, 100, |_| 5.0);
        let expected = 10.0; // Integral of f(x) = 5 over [0, 2] is 5 * (2 - 0)
        assert!((result - expected).abs() < 0.001);
    }

    #[test]
    fn test_trapezoidal_rule_with_sine() {
        // Define a sine function f(x) = sin(x) over [0, π]
        let result = trapezoidal_rule(0.0, std::f64::consts::PI, 100, |x| x.sin());
        let expected = 2.0; // Exact integral of sin(x) over [0, π] is 2
        assert!((result - expected).abs() < 0.001);
    }


}
