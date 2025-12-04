use super::Shape;

#[derive(Debug, Clone, Copy)]
pub struct Circle {
    pub r: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.r * self.r
    }
}
