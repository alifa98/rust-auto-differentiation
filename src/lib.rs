pub mod dual;

pub use dual::Dual;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_arithmetic() {
        let x = Dual::new(3.0, 1.0);
        let y = Dual::new(4.0, 0.0);

        let result = x + y;
        assert_eq!(result.value, 7.0);
        assert_eq!(result.derivative, 1.0);
    }

    #[test]
    fn test_derivative_of_polynomial() {
        let x = Dual::new(2.0, 1.0); // Differentiating w.r.t x
        let result = x * x + x * Dual::new(3.0, 0.0) + Dual::new(5.0, 0.0);

        assert_eq!(result.value, 2.0 * 2.0 + 2.0 * 3.0 + 5.0);
        assert_eq!(result.derivative, 2.0 * 2.0 + 3.0);
    }

    #[test]
    fn test_exp_function() {
        let x = Dual::new(1.0, 1.0); // Differentiating w.r.t x
        let result = x.exp();

        assert_eq!(result.value, x.value.exp());
        assert_eq!(result.derivative, x.value.exp());
    }

    #[test]
    fn test_multivariable_function() {
        fn f(x: Dual, y: Dual) -> Dual {
            x * x + y * y
        }

        let x = Dual::new(3.0, 1.0); // Differentiating w.r.t x
        let y = Dual::new(4.0, 0.0); // Not differentiating w.r.t y
        let result = f(x, y);

        assert_eq!(result.value, 3.0 * 3.0 + 4.0 * 4.0);
        assert_eq!(result.derivative, 2.0 * 3.0); // ∂f/∂x
    }
}
