# Auto Diffentiation for Rust

## Introduction

This is toy project for me to learn Rust and how to implement auto differentiation in Rust.

## How to use

Add this to your `Cargo.toml`:

```toml
[dependencies]
auto_diff = { git = "https://github.com/alifa98/rust-auto-differentiation.git" }

```

## Example

```rust
use auto_diff::Dual;

/// The Rosenbrock function: f(x, y) = (1 - x)^2 + 100 * (y - x^2)^2
fn rosenbrock(x: Dual, y: Dual) -> Dual {
    let term1 = (Dual::new(1.0, 0.0) - x).powi(2);
    let term2 = Dual::new(100.0, 0.0) * (y - x * x).powi(2);
    term1 + term2
}

/// Gradient descent for the Rosenbrock function
/// Stops early if gradient magnitude is below `tolerance`.
fn gradient_descent(
    initial_x: f64,
    initial_y: f64,
    learning_rate: f64,
    max_iterations: usize,
    tolerance: f64,
) {
    let mut x = initial_x;
    let mut y = initial_y;

    for i in 0..max_iterations {
        // Compute partial derivatives
        let grad_x = rosenbrock(Dual::new(x, 1.0), Dual::new(y, 0.0)).derivative;
        let grad_y = rosenbrock(Dual::new(x, 0.0), Dual::new(y, 1.0)).derivative;

        // Compute the gradient magnitude
        let grad_magnitude = (grad_x * grad_x + grad_y * grad_y).sqrt();

        println!(
            "Iteration {}: x = {:.4}, y = {:.4}, f(x, y) = {:.4}, grad_x = {:.4}, grad_y = {:.4}, grad_mag = {:.4}",
            i + 1,
            x,
            y,
            rosenbrock(Dual::new(x, 0.0), Dual::new(y, 0.0)).value,
            grad_x,
            grad_y,
            grad_magnitude
        );

        // Stop if gradient magnitude is below tolerance
        if grad_magnitude < tolerance {
            println!("Gradient magnitude is below the tolerance threshold. Stopping early.");
            break;
        }

        // Update variables using the gradient
        x -= learning_rate * grad_x;
        y -= learning_rate * grad_y;
    }
}

fn main() {
    // Initial values
    let initial_x = -1.2; // Start far from the minimum
    let initial_y = 1.0;

    // Parameters
    let learning_rate = 0.001;
    let max_iterations = 10000;
    let tolerance = 1e-6;

    // Perform gradient descent
    gradient_descent(initial_x, initial_y, learning_rate, max_iterations, tolerance);
}
```

Have fun!
