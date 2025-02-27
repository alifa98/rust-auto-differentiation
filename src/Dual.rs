use std::ops::{Add, Sub, Mul, Div};

#[derive(Debug, Clone, Copy)]
pub struct Dual {
    pub value: f64,
    pub derivative: f64,
}

impl Dual {
    pub fn new(value: f64, derivative: f64) -> Self {
        Dual { value, derivative }
    }

    pub fn exp(self) -> Dual {
        let exp_value = self.value.exp();
        Dual {
            value: exp_value,
            derivative: exp_value * self.derivative,
        }
    }

    pub fn ln(self) -> Dual {
        Dual {
            value: self.value.ln(),
            derivative: self.derivative / self.value,
        }
    }

    pub fn sin(self) -> Dual {
        Dual {
            value: self.value.sin(),
            derivative: self.derivative * self.value.cos(),
        }
    }

    pub fn cos(self) -> Dual {
        Dual {
            value: self.value.cos(),
            derivative: -self.derivative * self.value.sin(),
        }
    }

    /// Raises the `Dual` number to an integer power `n` using the power rule for differentiation.
    pub fn powi(self, n: i32) -> Dual {
        let value_pow_n = self.value.powi(n); // g(x)^n
        let derivative = n as f64 * self.value.powi(n - 1) * self.derivative; // n * g(x)^(n-1) * g'(x)

        Dual {
            value: value_pow_n,
            derivative,
        }
    }
}


// Operator overloading for Dual numbers
impl Add for Dual {
    type Output = Dual;

    fn add(self, other: Dual) -> Dual {
        Dual {
            value: self.value + other.value,
            derivative: self.derivative + other.derivative,
        }
    }
}

impl Sub for Dual {
    type Output = Dual;

    fn sub(self, other: Dual) -> Dual {
        Dual {
            value: self.value - other.value,
            derivative: self.derivative - other.derivative,
        }
    }
}

impl Mul for Dual {
    type Output = Dual;

    fn mul(self, other: Dual) -> Dual {
        Dual {
            value: self.value * other.value,
            derivative: self.value * other.derivative + self.derivative * other.value,
        }
    }
}

impl Div for Dual {
    type Output = Dual;

    fn div(self, other: Dual) -> Dual {
        Dual {
            value: self.value / other.value,
            derivative: (self.derivative * other.value - self.value * other.derivative)
                / (other.value * other.value),
        }
    }
}
