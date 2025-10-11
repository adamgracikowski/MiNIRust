mod numeric_operations;
mod in_place_numeric_operations;
mod mul_vals;
mod mul_vals_vec;

pub use numeric_operations::NumericOperations;
pub use in_place_numeric_operations::InPlaceNumericOperations;
pub use mul_vals::mul_vals;
pub use mul_vals_vec::mul_vals_vec;

#[derive(Debug, Clone, Default)]
pub struct NumberWithUnits {
    value: f64,
    unit: String,
}

impl NumberWithUnits {
    pub fn unitless(value: f64) -> Self {
        NumberWithUnits {
            value,
            unit: String::new(),
        }
    }

    pub fn with_unit(value: f64, unit: String) -> Self {
        NumberWithUnits { value, unit }
    }

    pub fn with_unit_from(other: &NumberWithUnits, value: f64) -> Self {
        NumberWithUnits {
            value,
            unit: other.unit.clone(),
        }
    }
}

impl NumericOperations for NumberWithUnits {
    fn add(self, other: Self) -> Self {
        if self.unit == other.unit {
            NumberWithUnits {
                value: self.value + other.value,
                unit: self.unit,
            }
        } else {
            panic!("Cannot add numbers with different units");
        }
    }

    fn mul(self, other: Self) -> Self {
        let unit = if self.unit.is_empty() {
            other.unit.clone()
        } else if other.unit.is_empty() {
            self.unit.clone()
        } else {
            format!("{}*{}", self.unit, other.unit)
        };
        NumberWithUnits {
            value: self.value * other.value,
            unit,
        }
    }

    fn div(self, other: Self) -> Self {
        if other.value == 0.0 {
            panic!("Division by zero");
        }

        NumberWithUnits {
            value: self.value / other.value,
            unit: if self.unit == other.unit {
                String::new()
            } else {
                format!("{}/{}", self.unit, other.unit)
            }
        }
    }
}

impl InPlaceNumericOperations for NumberWithUnits {
    fn add_in_place(&mut self, other: &Self) {
        if self.unit == other.unit {
            self.value += other.value;
        } else {
            panic!("Cannot add numbers with different units");
        }
    }

    fn mul_in_place(&mut self, other: &Self) {
        self.value *= other.value;
        self.unit = if self.unit.is_empty() {
            other.unit.clone()
        } else if other.unit.is_empty() {
            self.unit.clone()
        } else {
            format!("{}*{}", self.unit, other.unit)
        };
    }

    fn div_in_place(&mut self, other: &Self) {
        if other.value == 0.0 {
            panic!("Division by zero");
        }

        self.value /= other.value;
        if self.unit == other.unit {
            self.unit.clear();
        } else {
            self.unit = format!("{}/{}", self.unit, other.unit);
        }
    }
}