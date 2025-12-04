use super::Transform;

pub struct Mul {
    pub k: f64,
}

impl Transform<f64> for Mul {
    fn name(&self) -> &str {
        "mul"
    }

    fn apply(&self, x: f64) -> f64 {
        x * self.k
    }
}
