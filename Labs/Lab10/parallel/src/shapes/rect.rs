use super::Shape;

#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub w: f64,
    pub h: f64,
}

impl Shape for Rect {
    fn area(&self) -> f64 {
        self.w * self.h
    }
}
