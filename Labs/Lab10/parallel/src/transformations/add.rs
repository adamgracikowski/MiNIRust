use super::Transform;

pub struct Add {
    pub k: f64,
}

impl Transform<f64> for Add {
    fn name(&self) -> &str {
        "add"
    }

    fn apply(&self, x: f64) -> f64 {
        x + self.k
    }
}
